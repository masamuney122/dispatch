use std::sync::Arc;
use std::time::Duration;

use base64::Engine;
use dispatch_core::{
    ApiRequest, GlobalHttpSettings, HttpVersionPreference, PreparedBody, ProxyMode,
};
use reqwest::header::{ACCEPT, ACCEPT_ENCODING, USER_AGENT};

use crate::services::cookie_service::ManagedCookieJar;

const SUPPORTED_METHODS: [&str; 7] = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];

pub(super) fn build_client(
    settings: &GlobalHttpSettings,
    cookie_jar: Option<Arc<ManagedCookieJar>>,
) -> Result<reqwest::Client, String> {
    let redirect_policy = if settings.follow_redirects {
        reqwest::redirect::Policy::limited(settings.max_redirects)
    } else {
        reqwest::redirect::Policy::none()
    };
    let mut builder = reqwest::Client::builder()
        .danger_accept_invalid_certs(!settings.verify_ssl)
        .referer(!settings.remove_referer_on_redirect)
        .redirect(redirect_policy);
    builder = configure_proxy(builder, settings)?;
    if settings.request_timeout_ms > 0 {
        builder = builder.timeout(Duration::from_millis(settings.request_timeout_ms));
    }
    if settings.cookies_enabled {
        if let Some(cookie_jar) = cookie_jar {
            builder = builder.cookie_provider(cookie_jar);
        }
    }
    builder = match settings.http_version {
        HttpVersionPreference::Auto => builder,
        HttpVersionPreference::Http1 => builder.http1_only(),
        HttpVersionPreference::Http2 => builder.http2_prior_knowledge(),
    };
    builder
        .build()
        .map_err(|error| format!("HTTP client could not be configured: {error}"))
}

fn configure_proxy(
    builder: reqwest::ClientBuilder,
    settings: &GlobalHttpSettings,
) -> Result<reqwest::ClientBuilder, String> {
    match settings.proxy_mode {
        ProxyMode::System => Ok(builder),
        ProxyMode::NoProxy => Ok(builder.no_proxy()),
        ProxyMode::Manual => {
            let proxy_url = settings.proxy_url.trim();
            if proxy_url.is_empty() {
                return Err("Manual proxy is enabled, but no proxy server was configured.".into());
            }
            if !settings.proxy_use_http && !settings.proxy_use_https {
                return Err("Manual proxy must be enabled for HTTP, HTTPS, or both.".into());
            }

            let mut proxy = match (settings.proxy_use_http, settings.proxy_use_https) {
                (true, true) => reqwest::Proxy::all(proxy_url),
                (true, false) => reqwest::Proxy::http(proxy_url),
                (false, true) => reqwest::Proxy::https(proxy_url),
                (false, false) => unreachable!(),
            }
            .map_err(|error| format!("Invalid manual proxy server: {error}"))?;

            let bypass = settings.proxy_bypass.trim();
            if !bypass.is_empty() {
                proxy = proxy.no_proxy(reqwest::NoProxy::from_string(bypass));
            }

            if settings.proxy_auth_enabled {
                if settings.proxy_username.trim().is_empty() {
                    return Err(
                        "Proxy authentication is enabled, but no username was configured.".into(),
                    );
                }
                let password = crate::services::proxy_credential_service::load_password()?
                    .ok_or_else(|| {
                        "Proxy authentication is enabled, but no password is stored. Open Global Settings and enter the proxy password."
                            .to_string()
                    })?;
                proxy = proxy.basic_auth(settings.proxy_username.trim(), &password);
            }

            Ok(builder.no_proxy().proxy(proxy))
        }
    }
}

pub(super) fn build_request(
    client: &reqwest::Client,
    request: &ApiRequest,
    body: &PreparedBody,
) -> Result<reqwest::RequestBuilder, String> {
    let method = request.method.to_uppercase();
    if !SUPPORTED_METHODS.contains(&method.as_str()) {
        return Err(format!("Unsupported HTTP method: {}", request.method));
    }
    let method =
        reqwest::Method::from_bytes(method.as_bytes()).map_err(|error| error.to_string())?;
    let mut builder = client.request(method, &request.url);
    if !has_header(request, ACCEPT.as_str()) {
        builder = builder.header(ACCEPT, "*/*");
    }
    if !has_header(request, ACCEPT_ENCODING.as_str()) {
        builder = builder.header(ACCEPT_ENCODING, "gzip, deflate, br, zstd");
    }
    if !has_header(request, USER_AGENT.as_str()) {
        builder = builder.header(USER_AGENT, concat!("Dispatch/", env!("CARGO_PKG_VERSION")));
    }
    for (key, value) in &request.headers {
        builder = builder.header(key, value);
    }
    apply_body(builder, body)
}

fn has_header(request: &ApiRequest, expected: &str) -> bool {
    request
        .headers
        .keys()
        .any(|name| name.eq_ignore_ascii_case(expected))
}

fn apply_body(
    mut builder: reqwest::RequestBuilder,
    body: &PreparedBody,
) -> Result<reqwest::RequestBuilder, String> {
    match body {
        PreparedBody::None => {}
        PreparedBody::Text { value } | PreparedBody::UrlEncoded { value } => {
            builder = builder.body(value.clone());
        }
        PreparedBody::FormData { fields } => {
            let form = fields
                .iter()
                .fold(reqwest::multipart::Form::new(), |form, field| {
                    form.text(field.key.clone(), field.value.clone())
                });
            builder = builder.multipart(form);
        }
        PreparedBody::Binary { data_base64 } => {
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(data_base64)
                .map_err(|error| format!("Invalid binary payload: {error}"))?;
            builder = builder.body(bytes);
        }
    }
    Ok(builder)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_proxy_mode_builds_a_direct_client() {
        let settings = GlobalHttpSettings {
            proxy_mode: ProxyMode::NoProxy,
            ..Default::default()
        };

        assert!(build_client(&settings, None).is_ok());
    }

    #[test]
    fn manual_proxy_requires_a_server() {
        let settings = GlobalHttpSettings {
            proxy_mode: ProxyMode::Manual,
            proxy_url: String::new(),
            ..Default::default()
        };

        let error = build_client(&settings, None).expect_err("reject empty proxy server");
        assert!(error.contains("no proxy server"));
    }

    #[test]
    fn manual_proxy_requires_at_least_one_request_protocol() {
        let settings = GlobalHttpSettings {
            proxy_mode: ProxyMode::Manual,
            proxy_url: "http://127.0.0.1:8080".into(),
            proxy_use_http: false,
            proxy_use_https: false,
            ..Default::default()
        };

        let error = build_client(&settings, None).expect_err("reject empty proxy scope");
        assert!(error.contains("HTTP, HTTPS, or both"));
    }

    #[test]
    fn manual_proxy_accepts_a_bypass_list() {
        let settings = GlobalHttpSettings {
            proxy_mode: ProxyMode::Manual,
            proxy_url: "http://127.0.0.1:8080".into(),
            proxy_bypass: "localhost,127.0.0.1,::1,.company.local".into(),
            ..Default::default()
        };

        assert!(build_client(&settings, None).is_ok());
    }
}
