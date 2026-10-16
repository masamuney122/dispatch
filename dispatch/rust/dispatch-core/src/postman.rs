use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

use crate::{
    ApiKeyLocation, ApiRequest, AuthConfig, BodyField, Collection, Environment,
    EnvironmentVariableMetadata, Folder, OAuth2ClientAuthentication, OAuth2GrantType,
    RequestBodyType, RequestHttpSettings, RequestScripts, SavedRequest,
};

pub const MAX_POSTMAN_SIZE: usize = 20 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PostmanWarning {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PostmanDocumentKind {
    Collection,
    Environment,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PostmanImportPreview {
    pub kind: PostmanDocumentKind,
    pub title: String,
    pub schema: String,
    pub request_count: usize,
    pub folder_count: usize,
    pub variable_count: usize,
    #[serde(default)]
    pub enabled_variable_count: usize,
    #[serde(default)]
    pub disabled_variable_count: usize,
    #[serde(default)]
    pub sensitive_variable_count: usize,
    #[serde(default)]
    pub empty_variable_count: usize,
    pub script_count: usize,
    pub warnings: Vec<PostmanWarning>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PostmanImportContext {
    pub collection_id: String,
    pub id_prefix: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostmanImportDocument {
    pub kind: PostmanDocumentKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection: Option<Collection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(default)]
    pub environment_variables: HashMap<String, String>,
    #[serde(default)]
    pub environment_variable_metadata: HashMap<String, EnvironmentVariableMetadata>,
    pub warnings: Vec<PostmanWarning>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PostmanExportDocument {
    pub content: String,
    pub mime_type: String,
    pub extension: String,
    pub request_count: usize,
    pub variable_count: usize,
    pub warnings: Vec<PostmanWarning>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct PostmanEnvironmentExportOptions {
    #[serde(default)]
    pub include_sensitive_values: bool,
    #[serde(default)]
    pub exported_at: String,
}

pub fn parse_postman(content: &str) -> Result<Value, String> {
    if content.len() > MAX_POSTMAN_SIZE {
        return Err(format!(
            "Postman document exceeds the {} MB limit",
            MAX_POSTMAN_SIZE / 1024 / 1024
        ));
    }
    serde_json::from_str(content)
        .map_err(|error| format!("Postman document is not valid JSON: {error}"))
}

pub fn inspect_postman(
    document: &Value,
    fallback_title: &str,
) -> Result<PostmanImportPreview, String> {
    let object = document
        .as_object()
        .ok_or_else(|| "Postman document must be a JSON object".to_string())?;
    if object.contains_key("info") && object.contains_key("item") {
        let schema = document
            .pointer("/info/schema")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let mut warnings = Vec::new();
        if !schema.contains("v2.1") {
            warnings.push(warning("postman.schema", "The document is not marked as Postman Collection v2.1; compatible fields will still be imported.", None));
        }
        let mut request_count = 0;
        let mut folder_count = 0;
        let mut script_count = events(document.get("event")).0;
        count_items(
            document.get("item"),
            &mut request_count,
            &mut folder_count,
            &mut script_count,
        );
        let variable_count = enabled_variables(document.get("variable")).len();
        Ok(PostmanImportPreview {
            kind: PostmanDocumentKind::Collection,
            title: document
                .pointer("/info/name")
                .and_then(Value::as_str)
                .unwrap_or(fallback_title)
                .to_string(),
            schema,
            request_count,
            folder_count,
            variable_count,
            enabled_variable_count: variable_count,
            disabled_variable_count: 0,
            sensitive_variable_count: 0,
            empty_variable_count: 0,
            script_count,
            warnings,
        })
    } else if object.contains_key("values") {
        let stats = environment_variable_stats(document.get("values"));
        let mut warnings = Vec::new();
        let empty_variable_count = environment_empty_variable_count(document.get("values"));
        if empty_variable_count > 0 {
            warnings.push(warning(
                "postman.environment.empty_values",
                &format!(
                    "The source Postman file contains {empty_variable_count} environment variable value(s) with no data. Dispatch will import those values as empty strings."
                ),
                None,
            ));
        }
        Ok(PostmanImportPreview {
            kind: PostmanDocumentKind::Environment,
            title: document
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or(fallback_title)
                .to_string(),
            schema: document
                .get("_postman_variable_scope")
                .and_then(Value::as_str)
                .unwrap_or("environment")
                .to_string(),
            request_count: 0,
            folder_count: 0,
            variable_count: stats.0,
            enabled_variable_count: stats.1,
            disabled_variable_count: stats.2,
            sensitive_variable_count: stats.3,
            empty_variable_count,
            script_count: 0,
            warnings,
        })
    } else {
        Err("This JSON file is neither a Postman Collection nor a Postman Environment".to_string())
    }
}

pub fn import_postman(
    document: &Value,
    requested_name: Option<&str>,
    context: &PostmanImportContext,
) -> Result<PostmanImportDocument, String> {
    if context.id_prefix.trim().is_empty() || context.timestamp.trim().is_empty() {
        return Err("Postman import context is incomplete".to_string());
    }
    let preview = inspect_postman(document, "Imported Postman document")?;
    if preview.kind == PostmanDocumentKind::Environment {
        let (environment_variables, environment_variable_metadata, mut warnings) =
            import_environment_variables(document.get("values"));
        warnings.extend(preview.warnings);
        return Ok(PostmanImportDocument {
            kind: PostmanDocumentKind::Environment,
            collection: None,
            environment_name: Some(
                requested_name
                    .filter(|name| !name.trim().is_empty())
                    .unwrap_or(&preview.title)
                    .to_string(),
            ),
            environment_variables,
            environment_variable_metadata,
            warnings,
        });
    }

    let mut state = ImportState {
        folders: Vec::new(),
        requests: Vec::new(),
        warnings: preview.warnings,
        next_id: 0,
    };
    import_items(
        document.get("item"),
        None,
        context,
        &mut state,
        "collection",
    )?;
    let name = requested_name
        .filter(|name| !name.trim().is_empty())
        .unwrap_or(&preview.title)
        .to_string();
    let collection = Collection {
        id: context.collection_id.clone(),
        name,
        folders: state.folders,
        requests: state.requests,
        created_at: context.timestamp.clone(),
        updated_at: context.timestamp.clone(),
        description: description(
            document
                .get("info")
                .and_then(|info| info.get("description")),
        ),
        variables: enabled_variables(document.get("variable")),
        auth: convert_auth(document.get("auth"), &mut state.warnings, "collection"),
        scripts: convert_events(document.get("event"), &mut state.warnings, "collection"),
    };
    Ok(PostmanImportDocument {
        kind: PostmanDocumentKind::Collection,
        collection: Some(collection),
        environment_name: None,
        environment_variables: HashMap::new(),
        environment_variable_metadata: HashMap::new(),
        warnings: state.warnings,
    })
}

struct ImportState {
    folders: Vec<Folder>,
    requests: Vec<SavedRequest>,
    warnings: Vec<PostmanWarning>,
    next_id: usize,
}

fn import_items(
    items: Option<&Value>,
    parent: Option<String>,
    context: &PostmanImportContext,
    state: &mut ImportState,
    path: &str,
) -> Result<(), String> {
    for (order, item) in items
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .enumerate()
    {
        let name = item
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or("Untitled");
        let location = format!("{path}/{name}");
        if let Some(request) = item.get("request") {
            if let Some(converted) = convert_request(request, item, &location, &mut state.warnings)
            {
                let id = next_id(context, state, "request");
                state.requests.push(SavedRequest {
                    id,
                    name: name.to_string(),
                    request: converted,
                    folder_id: parent.clone(),
                    order: order as i64,
                    created_at: context.timestamp.clone(),
                    updated_at: context.timestamp.clone(),
                    description: description(request.get("description")),
                });
            }
        } else if item.get("item").and_then(Value::as_array).is_some() {
            let id = next_id(context, state, "folder");
            state.folders.push(Folder {
                id: id.clone(),
                name: name.to_string(),
                collection_id: context.collection_id.clone(),
                parent_folder_id: parent.clone(),
                order: order as i64,
                created_at: context.timestamp.clone(),
                updated_at: context.timestamp.clone(),
                description: description(item.get("description")),
                auth: convert_auth(item.get("auth"), &mut state.warnings, &location),
                scripts: convert_events(item.get("event"), &mut state.warnings, &location),
            });
            import_items(item.get("item"), Some(id), context, state, &location)?;
        } else {
            state.warnings.push(warning(
                "postman.item.empty",
                "An empty item was skipped.",
                Some(location),
            ));
        }
    }
    Ok(())
}

fn next_id(context: &PostmanImportContext, state: &mut ImportState, kind: &str) -> String {
    let id = format!("{}-{kind}-{}", context.id_prefix, state.next_id);
    state.next_id += 1;
    id
}

fn convert_request(
    value: &Value,
    item: &Value,
    location: &str,
    warnings: &mut Vec<PostmanWarning>,
) -> Option<ApiRequest> {
    if item
        .get("response")
        .and_then(Value::as_array)
        .is_some_and(|responses| !responses.is_empty())
    {
        warnings.push(warning(
            "postman.examples.unsupported",
            "Saved response examples are not supported by the Dispatch collection model and were skipped.",
            Some(location.to_string()),
        ));
    }
    let request = if let Some(url) = value.as_str() {
        json!({"url": url, "method": "GET"})
    } else {
        value.clone()
    };
    let method = request
        .get("method")
        .and_then(Value::as_str)
        .unwrap_or("GET")
        .to_uppercase();
    let supported = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];
    if !supported.contains(&method.as_str()) {
        warnings.push(warning(
            "postman.method.unsupported",
            &format!("Unsupported HTTP method '{method}' was skipped."),
            Some(location.to_string()),
        ));
        return None;
    }
    let mut headers = HashMap::new();
    let mut seen = HashSet::new();
    for header in request
        .get("header")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        if header
            .get("disabled")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            continue;
        }
        let Some(key) = header.get("key").and_then(Value::as_str) else {
            continue;
        };
        let value = value_string(header.get("value"));
        if !seen.insert(key.to_ascii_lowercase()) {
            warnings.push(warning(
                "postman.header.duplicate",
                &format!(
                    "Duplicate header '{key}' cannot be represented; the last value was kept."
                ),
                Some(location.to_string()),
            ));
        }
        headers.insert(key.to_string(), value);
    }
    let (body, body_type, form_fields) = convert_body(request.get("body"), warnings, location);
    Some(ApiRequest {
        method,
        url: convert_url(request.get("url"), warnings, location),
        body,
        body_type,
        form_fields,
        binary: None,
        headers,
        auth: convert_auth(request.get("auth"), warnings, location),
        settings: RequestHttpSettings::default(),
        scripts: convert_events(item.get("event"), warnings, location),
    })
}

fn convert_url(
    value: Option<&Value>,
    warnings: &mut Vec<PostmanWarning>,
    location: &str,
) -> String {
    let Some(value) = value else {
        return String::new();
    };
    if let Some(raw) = value.as_str() {
        return raw.to_string();
    }
    if let Some(raw) = value.get("raw").and_then(Value::as_str) {
        if value
            .get("query")
            .and_then(Value::as_array)
            .is_some_and(|items| {
                items.iter().any(|item| {
                    item.get("disabled")
                        .and_then(Value::as_bool)
                        .unwrap_or(false)
                })
            })
        {
            warnings.push(warning("postman.query.disabled", "Disabled query parameters may still be present in the raw URL and should be reviewed.", Some(location.to_string())));
        }
        return raw.to_string();
    }
    let protocol = value
        .get("protocol")
        .and_then(Value::as_str)
        .map(|p| format!("{p}://"))
        .unwrap_or_default();
    let host = join_value(value.get("host"), ".");
    let path = join_value(value.get("path"), "/");
    format!(
        "{protocol}{host}{}",
        if path.is_empty() {
            String::new()
        } else {
            format!("/{path}")
        }
    )
}

fn convert_body(
    value: Option<&Value>,
    warnings: &mut Vec<PostmanWarning>,
    location: &str,
) -> (String, RequestBodyType, Vec<BodyField>) {
    let Some(body) = value else {
        return (String::new(), RequestBodyType::None, Vec::new());
    };
    match body.get("mode").and_then(Value::as_str).unwrap_or("") {
        "raw" => {
            let raw = value_string(body.get("raw"));
            let language = body
                .pointer("/options/raw/language")
                .and_then(Value::as_str)
                .unwrap_or("");
            let kind = match language {
                "json" => RequestBodyType::Json,
                "xml" => RequestBodyType::Xml,
                "html" => RequestBodyType::Html,
                _ => RequestBodyType::Text,
            };
            (raw, kind, Vec::new())
        }
        "urlencoded" => (
            String::new(),
            RequestBodyType::XWwwFormUrlencoded,
            fields(body.get("urlencoded"), false, warnings, location),
        ),
        "formdata" => (
            String::new(),
            RequestBodyType::FormData,
            fields(body.get("formdata"), true, warnings, location),
        ),
        "file" => {
            warnings.push(warning(
                "postman.body.file",
                "A Postman file body cannot be embedded automatically and was left empty.",
                Some(location.to_string()),
            ));
            (String::new(), RequestBodyType::Binary, Vec::new())
        }
        mode if !mode.is_empty() => {
            warnings.push(warning(
                "postman.body.unsupported",
                &format!("Body mode '{mode}' is not supported and was left empty."),
                Some(location.to_string()),
            ));
            (String::new(), RequestBodyType::None, Vec::new())
        }
        _ => (String::new(), RequestBodyType::None, Vec::new()),
    }
}

fn fields(
    value: Option<&Value>,
    warn_files: bool,
    warnings: &mut Vec<PostmanWarning>,
    location: &str,
) -> Vec<BodyField> {
    value.and_then(Value::as_array).into_iter().flatten().filter(|entry| !entry.get("disabled").and_then(Value::as_bool).unwrap_or(false)).filter_map(|entry| {
        if warn_files && entry.get("type").and_then(Value::as_str) == Some("file") {
            warnings.push(warning("postman.form.file", "A form-data file field was skipped because Dispatch cannot import the local file.", Some(location.to_string())));
            return None;
        }
        Some(BodyField { key: entry.get("key")?.as_str()?.to_string(), value: value_string(entry.get("value")) })
    }).collect()
}

fn convert_auth(
    value: Option<&Value>,
    warnings: &mut Vec<PostmanWarning>,
    location: &str,
) -> Option<AuthConfig> {
    let auth = value?.as_object()?;
    let kind = auth.get("type")?.as_str()?;
    let values = |name: &str| auth.get(name).and_then(Value::as_array);
    let get = |list: Option<&Vec<Value>>, key: &str| -> String {
        list.into_iter()
            .flatten()
            .find(|entry| entry.get("key").and_then(Value::as_str) == Some(key))
            .map(|entry| value_string(entry.get("value")))
            .unwrap_or_default()
    };
    match kind {
        "noauth" => Some(AuthConfig::None),
        "bearer" => Some(AuthConfig::Bearer {
            token: get(values("bearer"), "token"),
        }),
        "basic" => Some(AuthConfig::Basic {
            username: get(values("basic"), "username"),
            password: get(values("basic"), "password"),
        }),
        "apikey" => {
            let list = values("apikey");
            Some(AuthConfig::ApiKey {
                key: get(list, "key"),
                value: get(list, "value"),
                add_to: if get(list, "in").eq_ignore_ascii_case("query") {
                    ApiKeyLocation::QueryParam
                } else {
                    ApiKeyLocation::Header
                },
            })
        }
        "oauth2" => {
            let list = values("oauth2");
            let grant = get(list, "grant_type").replace('-', "_");
            Some(AuthConfig::OAuth2 {
                grant_type: match grant.as_str() {
                    "password" => OAuth2GrantType::Password,
                    "authorization_code" => OAuth2GrantType::AuthorizationCode,
                    _ => OAuth2GrantType::ClientCredentials,
                },
                access_token_url: get(list, "accessTokenUrl"),
                client_id: get(list, "clientId"),
                client_secret: get(list, "clientSecret"),
                scope: get(list, "scope"),
                username: get(list, "username"),
                password: get(list, "password"),
                access_token: get(list, "accessToken"),
                client_authentication: match get(list, "client_authentication").as_str() {
                    "body" => Some(OAuth2ClientAuthentication::Body),
                    "header" => Some(OAuth2ClientAuthentication::Header),
                    _ => None,
                },
                authorization_url: get(list, "authUrl"),
                redirect_uri: get(list, "redirect_uri"),
            })
        }
        other => {
            warnings.push(warning("postman.auth.unsupported", &format!("Authentication type '{other}' is not supported and will inherit from its parent."), Some(location.to_string())));
            None
        }
    }
}

fn convert_events(
    value: Option<&Value>,
    warnings: &mut Vec<PostmanWarning>,
    location: &str,
) -> RequestScripts {
    let mut scripts = RequestScripts::default();
    for event in value.and_then(Value::as_array).into_iter().flatten() {
        let code = match event.pointer("/script/exec") {
            Some(Value::Array(lines)) => lines
                .iter()
                .filter_map(Value::as_str)
                .collect::<Vec<_>>()
                .join("\n"),
            Some(Value::String(code)) => code.clone(),
            _ => String::new(),
        };
        if code.is_empty() {
            continue;
        }
        if code.contains("pm.sendRequest")
            || code.contains("pm.visualizer")
            || code.contains("pm.collectionVariables")
        {
            warnings.push(warning(
                "postman.script.compatibility",
                "The script uses a Postman API that may not be supported by Dispatch.",
                Some(location.to_string()),
            ));
        }
        match event.get("listen").and_then(Value::as_str) {
            Some("prerequest") => append_script(&mut scripts.pre_request, &code),
            Some("test") => append_script(&mut scripts.post_response, &code),
            _ => warnings.push(warning(
                "postman.event.unsupported",
                "An unsupported Postman event was skipped.",
                Some(location.to_string()),
            )),
        }
    }
    scripts
}

pub fn export_collection_postman(collection: &Collection) -> Result<PostmanExportDocument, String> {
    let mut warnings = Vec::new();
    let contains_secrets = collection.auth.is_some()
        || collection.variables.keys().any(|key| sensitive_name(key))
        || collection.requests.iter().any(|saved| {
            saved.request.auth.is_some()
                || saved.request.headers.keys().any(|key| sensitive_name(key))
        });
    if contains_secrets {
        warnings.push(warning(
            "postman.export.secrets",
            "The exported Postman file contains authentication or potentially sensitive values. Store and share it securely.",
            None,
        ));
    }
    let items = export_level(collection, None, &mut warnings);
    let mut root = Map::new();
    root.insert(
        "info".into(),
        json!({
            "name": collection.name,
            "description": collection.description,
            "schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"
        }),
    );
    root.insert("item".into(), Value::Array(items));
    if !collection.variables.is_empty() {
        root.insert("variable".into(), export_variables(&collection.variables));
    }
    if let Some(auth) = &collection.auth {
        root.insert("auth".into(), export_auth(auth));
    }
    let events = export_events(&collection.scripts);
    if !events.is_empty() {
        root.insert("event".into(), Value::Array(events));
    }
    let content =
        serde_json::to_string_pretty(&Value::Object(root)).map_err(|error| error.to_string())?;
    Ok(PostmanExportDocument {
        content,
        mime_type: "application/json".into(),
        extension: "postman_collection.json".into(),
        request_count: collection.requests.len(),
        variable_count: collection.variables.len(),
        warnings,
    })
}

pub fn export_environment_postman(
    environment: &Environment,
    options: &PostmanEnvironmentExportOptions,
) -> Result<PostmanExportDocument, String> {
    let mut entries = environment.variables.iter().collect::<Vec<_>>();
    entries.sort_by_key(|(name, _)| {
        environment
            .variable_metadata
            .get(*name)
            .map(|metadata| metadata.order)
            .unwrap_or(i64::MAX)
    });

    let mut warnings = Vec::new();
    let mut sensitive_count = 0usize;
    let values = entries
        .into_iter()
        .map(|(key, value)| {
            let metadata = environment.variable_metadata.get(key);
            let sensitive = metadata.map_or(false, |item| item.sensitive) || sensitive_name(key);
            if sensitive {
                sensitive_count += 1;
            }
            let exported_value = if sensitive && !options.include_sensitive_values {
                String::new()
            } else {
                value.clone()
            };
            let mut entry = Map::new();
            entry.insert("key".into(), Value::String(key.clone()));
            entry.insert("value".into(), Value::String(exported_value));
            entry.insert(
                "type".into(),
                Value::String(if sensitive { "secret" } else { "default" }.into()),
            );
            entry.insert("secret".into(), Value::Bool(sensitive));
            entry.insert(
                "enabled".into(),
                Value::Bool(metadata.map_or(true, |item| item.enabled)),
            );
            if let Some(description) = metadata
                .map(|item| item.description.trim())
                .filter(|description| !description.is_empty())
            {
                entry.insert("description".into(), Value::String(description.to_string()));
            }
            Value::Object(entry)
        })
        .collect::<Vec<_>>();

    if sensitive_count > 0 {
        warnings.push(warning(
            if options.include_sensitive_values {
                "postman.environment.secrets_included"
            } else {
                "postman.environment.secrets_redacted"
            },
            if options.include_sensitive_values {
                "The exported Postman environment contains sensitive values. Store and share it securely."
            } else {
                "Sensitive environment values were excluded from the exported Postman file."
            },
            None,
        ));
    }

    let content = serde_json::to_string_pretty(&json!({
        "id": environment.id,
        "name": environment.name,
        "values": values,
        "_postman_variable_scope": "environment",
        "_postman_exported_at": if options.exported_at.is_empty() { &environment.updated_at } else { &options.exported_at },
        "_postman_exported_using": "Dispatch"
    }))
    .map_err(|error| error.to_string())?;

    Ok(PostmanExportDocument {
        content,
        mime_type: "application/json".into(),
        extension: "postman_environment.json".into(),
        request_count: 0,
        variable_count: environment.variables.len(),
        warnings,
    })
}

fn export_level(
    collection: &Collection,
    parent: Option<&str>,
    warnings: &mut Vec<PostmanWarning>,
) -> Vec<Value> {
    let mut entries: Vec<(i64, bool, Value)> = Vec::new();
    for folder in collection
        .folders
        .iter()
        .filter(|folder| folder.parent_folder_id.as_deref() == parent)
    {
        let mut item = Map::new();
        item.insert("name".into(), Value::String(folder.name.clone()));
        if !folder.description.is_empty() {
            item.insert(
                "description".into(),
                Value::String(folder.description.clone()),
            );
        }
        item.insert(
            "item".into(),
            Value::Array(export_level(collection, Some(&folder.id), warnings)),
        );
        if let Some(auth) = &folder.auth {
            item.insert("auth".into(), export_auth(auth));
        }
        let events = export_events(&folder.scripts);
        if !events.is_empty() {
            item.insert("event".into(), Value::Array(events));
        }
        entries.push((folder.order, false, Value::Object(item)));
    }
    for saved in collection
        .requests
        .iter()
        .filter(|request| request.folder_id.as_deref() == parent)
    {
        let request = &saved.request;
        let mut object = Map::new();
        object.insert("method".into(), Value::String(request.method.clone()));
        object.insert(
            "header".into(),
            Value::Array(
                request
                    .headers
                    .iter()
                    .map(|(key, value)| json!({"key": key, "value": value, "type": "text"}))
                    .collect(),
            ),
        );
        // Postman accepts either a URL string or a fully decomposed URL object.
        // A partial object containing only `raw` may be normalized to an empty
        // URL by the desktop importer. The string form keeps Dispatch templates
        // and query strings intact while remaining valid Postman v2.1 JSON.
        object.insert("url".into(), Value::String(request.url.clone()));
        if let Some(auth) = &request.auth {
            object.insert("auth".into(), export_auth(auth));
        }
        if let Some(body) = export_body(request, warnings, &saved.name) {
            object.insert("body".into(), body);
        }
        let mut item = Map::new();
        item.insert("name".into(), Value::String(saved.name.clone()));
        if !saved.description.is_empty() {
            item.insert(
                "description".into(),
                Value::String(saved.description.clone()),
            );
        }
        item.insert("request".into(), Value::Object(object));
        let events = export_events(&request.scripts);
        if !events.is_empty() {
            item.insert("event".into(), Value::Array(events));
        }
        entries.push((saved.order, true, Value::Object(item)));
    }
    entries.sort_by_key(|(order, is_request, _)| (*order, *is_request));
    entries.into_iter().map(|(_, _, value)| value).collect()
}

fn export_body(
    request: &ApiRequest,
    warnings: &mut Vec<PostmanWarning>,
    location: &str,
) -> Option<Value> {
    match request.body_type {
        RequestBodyType::None => None,
        RequestBodyType::Json => {
            Some(json!({"mode":"raw","raw":request.body,"options":{"raw":{"language":"json"}}}))
        }
        RequestBodyType::Xml => {
            Some(json!({"mode":"raw","raw":request.body,"options":{"raw":{"language":"xml"}}}))
        }
        RequestBodyType::Html => {
            Some(json!({"mode":"raw","raw":request.body,"options":{"raw":{"language":"html"}}}))
        }
        RequestBodyType::Text => Some(json!({"mode":"raw","raw":request.body})),
        RequestBodyType::FormData => Some(
            json!({"mode":"formdata","formdata":request.form_fields.iter().map(|field| json!({"key":field.key,"value":field.value,"type":"text"})).collect::<Vec<_>>()}),
        ),
        RequestBodyType::XWwwFormUrlencoded => Some(
            json!({"mode":"urlencoded","urlencoded":request.form_fields.iter().map(|field| json!({"key":field.key,"value":field.value,"type":"text"})).collect::<Vec<_>>()}),
        ),
        RequestBodyType::Binary => {
            warnings.push(warning(
                "postman.export.binary",
                "Binary data was not embedded in the exported collection.",
                Some(location.to_string()),
            ));
            Some(json!({"mode":"file","file":{}}))
        }
    }
}

fn export_auth(auth: &AuthConfig) -> Value {
    match auth {
        AuthConfig::None => json!({"type":"noauth"}),
        AuthConfig::Bearer { token } => {
            json!({"type":"bearer","bearer":[{"key":"token","value":token,"type":"string"}]})
        }
        AuthConfig::Basic { username, password } => {
            json!({"type":"basic","basic":[{"key":"username","value":username,"type":"string"},{"key":"password","value":password,"type":"string"}]})
        }
        AuthConfig::ApiKey { key, value, add_to } => {
            json!({"type":"apikey","apikey":[{"key":"key","value":key,"type":"string"},{"key":"value","value":value,"type":"string"},{"key":"in","value":if matches!(add_to, ApiKeyLocation::QueryParam) {"query"} else {"header"},"type":"string"}]})
        }
        AuthConfig::OAuth2 {
            grant_type,
            access_token_url,
            client_id,
            client_secret,
            scope,
            username,
            password,
            access_token,
            client_authentication,
            authorization_url,
            redirect_uri,
        } => {
            let grant = match grant_type {
                OAuth2GrantType::ClientCredentials => "client_credentials",
                OAuth2GrantType::Password => "password",
                OAuth2GrantType::AuthorizationCode => "authorization_code",
            };
            let client_auth = match client_authentication {
                Some(OAuth2ClientAuthentication::Body) => "body",
                Some(OAuth2ClientAuthentication::Header) => "header",
                None => "",
            };
            json!({"type":"oauth2","oauth2":[
                {"key":"grant_type","value":grant,"type":"string"},
                {"key":"accessTokenUrl","value":access_token_url,"type":"string"},
                {"key":"clientId","value":client_id,"type":"string"},
                {"key":"clientSecret","value":client_secret,"type":"string"},
                {"key":"scope","value":scope,"type":"string"},
                {"key":"username","value":username,"type":"string"},
                {"key":"password","value":password,"type":"string"},
                {"key":"accessToken","value":access_token,"type":"string"},
                {"key":"client_authentication","value":client_auth,"type":"string"},
                {"key":"authUrl","value":authorization_url,"type":"string"},
                {"key":"redirect_uri","value":redirect_uri,"type":"string"}
            ]})
        }
    }
}

fn export_events(scripts: &RequestScripts) -> Vec<Value> {
    let mut result = Vec::new();
    if !scripts.pre_request.is_empty() {
        result.push(json!({"listen":"prerequest","script":{"type":"text/javascript","exec":scripts.pre_request.lines().collect::<Vec<_>>()}}));
    }
    if !scripts.post_response.is_empty() {
        result.push(json!({"listen":"test","script":{"type":"text/javascript","exec":scripts.post_response.lines().collect::<Vec<_>>()}}));
    }
    result
}

fn environment_variable_stats(value: Option<&Value>) -> (usize, usize, usize, usize) {
    let mut total = 0;
    let mut enabled = 0;
    let mut disabled = 0;
    let mut sensitive = 0;
    for entry in value.and_then(Value::as_array).into_iter().flatten() {
        if entry.get("key").and_then(Value::as_str).is_none() {
            continue;
        }
        total += 1;
        let is_enabled = !entry
            .get("disabled")
            .and_then(Value::as_bool)
            .unwrap_or(false)
            && entry.get("enabled").and_then(Value::as_bool) != Some(false);
        if is_enabled {
            enabled += 1;
        } else {
            disabled += 1;
        }
        if environment_variable_is_sensitive(entry) {
            sensitive += 1;
        }
    }
    (total, enabled, disabled, sensitive)
}

fn environment_empty_variable_count(value: Option<&Value>) -> usize {
    value
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|entry| entry.get("key").and_then(Value::as_str).is_some())
        .filter(|entry| value_string(entry.get("value")).is_empty())
        .count()
}

fn environment_variable_is_sensitive(entry: &Value) -> bool {
    entry.get("type").and_then(Value::as_str) == Some("secret")
        || entry
            .get("secret")
            .and_then(Value::as_bool)
            .unwrap_or(false)
}

fn import_environment_variables(
    value: Option<&Value>,
) -> (
    HashMap<String, String>,
    HashMap<String, EnvironmentVariableMetadata>,
    Vec<PostmanWarning>,
) {
    let mut variables = HashMap::new();
    let mut metadata = HashMap::new();
    let mut warnings = Vec::new();
    for (order, entry) in value
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .enumerate()
    {
        let Some(key) = entry
            .get("key")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|key| !key.is_empty())
        else {
            warnings.push(warning(
                "postman.environment.invalid_key",
                "An environment variable without a name was skipped.",
                Some(format!("values[{order}]")),
            ));
            continue;
        };
        if variables.contains_key(key) {
            warnings.push(warning(
                "postman.environment.duplicate_key",
                "A duplicate environment variable was found; the last value was imported.",
                Some(key.to_string()),
            ));
        }
        variables.insert(key.to_string(), value_string(entry.get("value")));
        metadata.insert(
            key.to_string(),
            EnvironmentVariableMetadata {
                enabled: !entry
                    .get("disabled")
                    .and_then(Value::as_bool)
                    .unwrap_or(false)
                    && entry.get("enabled").and_then(Value::as_bool) != Some(false),
                sensitive: environment_variable_is_sensitive(entry),
                description: description(entry.get("description")),
                order: order as i64,
            },
        );
    }
    (variables, metadata, warnings)
}

fn enabled_variables(value: Option<&Value>) -> HashMap<String, String> {
    value
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|entry| {
            !entry
                .get("disabled")
                .and_then(Value::as_bool)
                .unwrap_or(false)
                && entry.get("enabled").and_then(Value::as_bool) != Some(false)
        })
        .filter_map(|entry| {
            Some((
                entry.get("key")?.as_str()?.to_string(),
                value_string(entry.get("value")),
            ))
        })
        .collect()
}
fn export_variables(values: &HashMap<String, String>) -> Value {
    Value::Array(
        values
            .iter()
            .map(|(key, value)| json!({"key":key,"value":value,"type":"string"}))
            .collect(),
    )
}
fn value_string(value: Option<&Value>) -> String {
    match value {
        Some(Value::String(value)) => value.clone(),
        Some(Value::Null) | None => String::new(),
        Some(value) => value.to_string(),
    }
}
fn join_value(value: Option<&Value>, separator: &str) -> String {
    match value {
        Some(Value::Array(values)) => values
            .iter()
            .filter_map(Value::as_str)
            .collect::<Vec<_>>()
            .join(separator),
        Some(Value::String(value)) => value.clone(),
        _ => String::new(),
    }
}
fn description(value: Option<&Value>) -> String {
    value
        .and_then(Value::as_str)
        .or_else(|| {
            value
                .and_then(|value| value.get("content"))
                .and_then(Value::as_str)
        })
        .unwrap_or_default()
        .to_string()
}
fn append_script(target: &mut String, code: &str) {
    if !target.is_empty() {
        target.push_str("\n\n");
    }
    target.push_str(code);
}
fn events(value: Option<&Value>) -> (usize, ()) {
    (value.and_then(Value::as_array).map_or(0, Vec::len), ())
}
fn count_items(
    value: Option<&Value>,
    requests: &mut usize,
    folders: &mut usize,
    scripts: &mut usize,
) {
    for item in value.and_then(Value::as_array).into_iter().flatten() {
        *scripts += events(item.get("event")).0;
        if item.get("request").is_some() {
            *requests += 1;
        } else if item.get("item").is_some() {
            *folders += 1;
            count_items(item.get("item"), requests, folders, scripts);
        }
    }
}
fn warning(code: &str, message: &str, location: Option<String>) -> PostmanWarning {
    PostmanWarning {
        code: code.into(),
        message: message.into(),
        location,
    }
}

fn sensitive_name(name: &str) -> bool {
    let normalized = name.to_ascii_lowercase().replace(['-', '_', ' '], "");
    [
        "authorization",
        "token",
        "password",
        "secret",
        "apikey",
        "session",
        "cookie",
    ]
    .iter()
    .any(|part| normalized.contains(part))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imports_nested_collection_and_environment() {
        let value = json!({"info":{"name":"Demo","schema":"https://schema.getpostman.com/json/collection/v2.1.0/collection.json"},"variable":[{"key":"base","value":"https://example.com"}],"item":[{"name":"Users","item":[{"name":"List","request":{"method":"GET","url":{"raw":"{{base}}/users"}}}]}]});
        let result = import_postman(
            &value,
            None,
            &PostmanImportContext {
                collection_id: "c".into(),
                id_prefix: "i".into(),
                timestamp: "now".into(),
            },
        )
        .unwrap();
        let collection = result.collection.unwrap();
        assert_eq!(collection.folders.len(), 1);
        assert_eq!(collection.requests[0].request.url, "{{base}}/users");
        assert_eq!(collection.variables["base"], "https://example.com");

        let environment =
            json!({"name":"Local","values":[{"key":"host","value":"localhost","enabled":true}]});
        let result = import_postman(
            &environment,
            None,
            &PostmanImportContext {
                collection_id: "c".into(),
                id_prefix: "i".into(),
                timestamp: "now".into(),
            },
        )
        .unwrap();
        assert_eq!(result.environment_variables["host"], "localhost");
    }

    #[test]
    fn exported_collection_is_reimportable() {
        let expected_url = "https://example.com/search?q={{term}}&page=2";
        let imported = import_postman(
            &json!({"info":{"name":"Demo"},"item":[{"name":"Ping","request":{"method":"GET","url":expected_url}}]}),
            None,
            &PostmanImportContext {
                collection_id: "c".into(),
                id_prefix: "i".into(),
                timestamp: "now".into(),
            },
        )
        .unwrap()
        .collection
        .unwrap();
        let exported = export_collection_postman(&imported).unwrap();
        let parsed = parse_postman(&exported.content).unwrap();
        assert_eq!(
            parsed
                .pointer("/item/0/request/url")
                .and_then(Value::as_str),
            Some(expected_url)
        );
        assert_eq!(
            inspect_postman(&parsed, "fallback").unwrap().request_count,
            1
        );

        let reimported = import_postman(
            &parsed,
            None,
            &PostmanImportContext {
                collection_id: "reimported".into(),
                id_prefix: "r".into(),
                timestamp: "later".into(),
            },
        )
        .unwrap()
        .collection
        .unwrap();
        assert_eq!(reimported.requests[0].request.url, expected_url);
    }

    #[test]
    fn environment_round_trip_preserves_metadata_and_redacts_secrets() {
        let source = json!({
            "name": "Development",
            "values": [
                {"key":"baseUrl","value":"https://dev.example.com","enabled":true},
                {"key":"apiToken","value":"secret-value","type":"default","secret":true,"enabled":true,"description":"API token"},
                {"key":"disabledValue","value":"keep-me","enabled":false},
                {"key":"emptyValue","value":"","enabled":true}
            ],
            "_postman_variable_scope": "environment"
        });
        let preview = inspect_postman(&source, "fallback").unwrap();
        assert_eq!(preview.variable_count, 4);
        assert_eq!(preview.enabled_variable_count, 3);
        assert_eq!(preview.disabled_variable_count, 1);
        assert_eq!(preview.sensitive_variable_count, 1);
        assert_eq!(preview.empty_variable_count, 1);
        assert_eq!(preview.warnings[0].code, "postman.environment.empty_values");

        let imported = import_postman(
            &source,
            None,
            &PostmanImportContext {
                collection_id: "unused".into(),
                id_prefix: "environment".into(),
                timestamp: "now".into(),
            },
        )
        .unwrap();
        assert_eq!(imported.environment_variables.len(), 4);
        assert!(!imported.environment_variable_metadata["disabledValue"].enabled);
        assert!(imported.environment_variable_metadata["apiToken"].sensitive);
        assert_eq!(
            imported.environment_variable_metadata["apiToken"].description,
            "API token"
        );

        let environment = Environment {
            id: "environment-1".into(),
            name: imported.environment_name.unwrap(),
            variables: imported.environment_variables,
            variable_metadata: imported.environment_variable_metadata,
            workspace_id: Some("workspace-1".into()),
            created_at: "now".into(),
            updated_at: "now".into(),
        };
        let safe = export_environment_postman(
            &environment,
            &PostmanEnvironmentExportOptions {
                include_sensitive_values: false,
                exported_at: "later".into(),
            },
        )
        .unwrap();
        let safe_json: Value = serde_json::from_str(&safe.content).unwrap();
        let values = safe_json["values"].as_array().unwrap();
        let token = values
            .iter()
            .find(|value| value["key"] == "apiToken")
            .unwrap();
        assert_eq!(token["value"], "");
        assert_eq!(token["type"], "secret");
        let disabled = values
            .iter()
            .find(|value| value["key"] == "disabledValue")
            .unwrap();
        assert_eq!(disabled["enabled"], false);

        let full = export_environment_postman(
            &environment,
            &PostmanEnvironmentExportOptions {
                include_sensitive_values: true,
                exported_at: "later".into(),
            },
        )
        .unwrap();
        assert!(full.content.contains("secret-value"));
    }
}
