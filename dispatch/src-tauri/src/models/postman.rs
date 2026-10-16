use serde::{Deserialize, Serialize};

use dispatch_core::{Collection, Environment};
pub use dispatch_core::{PostmanDocumentKind, PostmanImportPreview, PostmanWarning};

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum PostmanSource {
    File {
        path: String,
    },
    Text {
        content: String,
        name: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct PostmanImportResult {
    pub kind: PostmanDocumentKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<Collection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Environment>,
    pub warnings: Vec<PostmanWarning>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PostmanExportResult {
    pub request_count: usize,
    pub variable_count: usize,
    pub warnings: Vec<PostmanWarning>,
    pub cancelled: bool,
}
