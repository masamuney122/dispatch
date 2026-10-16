use std::{path::PathBuf, sync::Mutex};

use crate::models::environment::AppState;
use crate::models::postman::{PostmanExportResult, PostmanImportResult, PostmanSource};
use crate::models::workspace::WorkspaceRuntimeState;
use crate::services::postman_service;

#[tauri::command]
pub fn inspect_postman(
    source: PostmanSource,
) -> Result<dispatch_core::PostmanImportPreview, String> {
    postman_service::inspect(&source)
}

#[tauri::command]
pub fn import_postman(
    runtime: tauri::State<'_, Mutex<WorkspaceRuntimeState>>,
    environment_state: tauri::State<'_, Mutex<AppState>>,
    source: PostmanSource,
) -> Result<PostmanImportResult, String> {
    let session = runtime
        .lock()
        .map_err(|error| error.to_string())?
        .current_workspace
        .clone()
        .ok_or_else(|| "No workspace is currently open".to_string())?;
    let mut state = environment_state
        .lock()
        .map_err(|error| error.to_string())?;
    postman_service::import(&session, &mut state, &source)
}

#[tauri::command]
pub fn export_collection_postman(
    runtime: tauri::State<'_, Mutex<WorkspaceRuntimeState>>,
    collection_id: String,
    path: String,
) -> Result<PostmanExportResult, String> {
    let session = runtime
        .lock()
        .map_err(|error| error.to_string())?
        .current_workspace
        .clone()
        .ok_or_else(|| "No workspace is currently open".to_string())?;
    postman_service::export_collection(&session, &collection_id, &PathBuf::from(path))
}

#[tauri::command]
pub fn export_environment_postman(
    runtime: tauri::State<'_, Mutex<WorkspaceRuntimeState>>,
    environment_id: String,
    include_sensitive_values: bool,
    path: String,
) -> Result<PostmanExportResult, String> {
    let session = runtime
        .lock()
        .map_err(|error| error.to_string())?
        .current_workspace
        .clone()
        .ok_or_else(|| "No workspace is currently open".to_string())?;
    postman_service::export_environment(
        &session,
        &environment_id,
        include_sensitive_values,
        &PathBuf::from(path),
    )
}
