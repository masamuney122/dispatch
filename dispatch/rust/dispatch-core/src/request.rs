use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::AuthConfig;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HttpVersionPreference {
    Auto,
    Http1,
    Http2,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum CookieCredentials {
    Omit,
    SameOrigin,
    Include,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProxyMode {
    System,
    Manual,
    NoProxy,
}

impl Default for ProxyMode {
    fn default() -> Self {
        Self::System
    }
}

impl Default for CookieCredentials {
    fn default() -> Self {
        Self::SameOrigin
    }
}

impl Default for HttpVersionPreference {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct GlobalHttpSettings {
    pub http_version: HttpVersionPreference,
    pub verify_ssl: bool,
    pub follow_redirects: bool,
    pub remove_referer_on_redirect: bool,
    pub max_redirects: usize,
    pub request_timeout_ms: u64,
    pub max_response_size_mb: usize,
    pub cookies_enabled: bool,
    pub cookie_credentials: CookieCredentials,
    pub proxy_mode: ProxyMode,
    pub proxy_url: String,
    pub proxy_use_http: bool,
    pub proxy_use_https: bool,
    pub proxy_bypass: String,
    pub proxy_auth_enabled: bool,
    pub proxy_username: String,
}

impl Default for GlobalHttpSettings {
    fn default() -> Self {
        Self {
            http_version: HttpVersionPreference::Auto,
            verify_ssl: true,
            follow_redirects: true,
            remove_referer_on_redirect: false,
            max_redirects: 10,
            request_timeout_ms: 0,
            max_response_size_mb: 50,
            cookies_enabled: true,
            cookie_credentials: CookieCredentials::SameOrigin,
            proxy_mode: ProxyMode::System,
            proxy_url: String::new(),
            proxy_use_http: true,
            proxy_use_https: true,
            proxy_bypass: "localhost,127.0.0.1,::1".into(),
            proxy_auth_enabled: false,
            proxy_username: String::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct RequestHttpSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_version: Option<HttpVersionPreference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_ssl: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_redirects: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_referer_on_redirect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_redirects: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_credentials: Option<CookieCredentials>,
}

pub fn resolve_http_settings(
    global: &GlobalHttpSettings,
    request: &RequestHttpSettings,
) -> GlobalHttpSettings {
    GlobalHttpSettings {
        http_version: request.http_version.unwrap_or(global.http_version),
        verify_ssl: request.verify_ssl.unwrap_or(global.verify_ssl),
        follow_redirects: request.follow_redirects.unwrap_or(global.follow_redirects),
        remove_referer_on_redirect: request
            .remove_referer_on_redirect
            .unwrap_or(global.remove_referer_on_redirect),
        max_redirects: request
            .max_redirects
            .unwrap_or(global.max_redirects)
            .clamp(1, 100),
        request_timeout_ms: global.request_timeout_ms.min(3_600_000),
        max_response_size_mb: global.max_response_size_mb.min(1_024),
        cookies_enabled: global.cookies_enabled,
        cookie_credentials: request
            .cookie_credentials
            .unwrap_or(global.cookie_credentials),
        proxy_mode: global.proxy_mode,
        proxy_url: global.proxy_url.clone(),
        proxy_use_http: global.proxy_use_http,
        proxy_use_https: global.proxy_use_https,
        proxy_bypass: global.proxy_bypass.clone(),
        proxy_auth_enabled: global.proxy_auth_enabled,
        proxy_username: global.proxy_username.clone(),
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum RequestBodyType {
    None,
    #[default]
    Json,
    Text,
    Html,
    Xml,
    FormData,
    XWwwFormUrlencoded,
    Binary,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BodyField {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BinaryBody {
    pub name: String,
    pub mime_type: String,
    pub data_base64: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct RequestScripts {
    #[serde(default)]
    pub pre_request: String,
    #[serde(default)]
    pub post_response: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApiRequest {
    pub method: String,
    pub url: String,
    pub body: String,
    #[serde(default)]
    pub body_type: RequestBodyType,
    #[serde(default)]
    pub form_fields: Vec<BodyField>,
    #[serde(default)]
    pub binary: Option<BinaryBody>,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(default)]
    pub auth: Option<AuthConfig>,
    #[serde(default)]
    pub settings: RequestHttpSettings,
    #[serde(default)]
    pub scripts: RequestScripts,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_settings_serialize_without_null_overrides() {
        assert_eq!(
            serde_json::to_value(RequestHttpSettings::default()).unwrap(),
            serde_json::json!({})
        );
    }

    #[test]
    fn legacy_requests_get_empty_scripts() {
        let request: ApiRequest = serde_json::from_value(serde_json::json!({
            "method": "GET",
            "url": "https://example.com",
            "body": "",
            "headers": {}
        }))
        .unwrap();
        assert_eq!(request.body_type, RequestBodyType::Json);
        assert_eq!(request.scripts, RequestScripts::default());
    }

    #[test]
    fn legacy_global_settings_default_to_the_system_proxy() {
        let settings: GlobalHttpSettings = serde_json::from_value(serde_json::json!({})).unwrap();

        assert_eq!(settings.proxy_mode, ProxyMode::System);
        assert!(settings.proxy_use_http);
        assert!(settings.proxy_use_https);
        assert_eq!(settings.proxy_bypass, "localhost,127.0.0.1,::1");
        assert!(!settings.proxy_auth_enabled);
    }

    #[test]
    fn request_http_settings_override_globals_and_clamp_redirects() {
        let request = RequestHttpSettings {
            verify_ssl: Some(false),
            max_redirects: Some(500),
            cookie_credentials: Some(CookieCredentials::Include),
            ..Default::default()
        };
        let resolved = resolve_http_settings(&GlobalHttpSettings::default(), &request);
        assert!(!resolved.verify_ssl);
        assert_eq!(resolved.max_redirects, 100);
        assert_eq!(resolved.request_timeout_ms, 0);
        assert_eq!(resolved.max_response_size_mb, 50);
        assert!(resolved.cookies_enabled);
        assert_eq!(resolved.cookie_credentials, CookieCredentials::Include);
        assert!(resolved.follow_redirects);
    }
}
