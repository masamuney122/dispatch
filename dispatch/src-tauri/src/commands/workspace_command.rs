use std::path::PathBuf;
use std::sync::Mutex;

use tauri::Manager;

use crate::models::environment::AppState;
use crate::models::workspace::{
    EnvironmentsDocument, GlobalHttpSettings, ProxyMode, RecentWorkspace, WorkspaceRuntimeState,
    WorkspaceSession, ENVIRONMENTS_FILE,
};
use crate::services::cookie_service::CookieRuntimeState;
use crate::services::storage_service::read_json;
use crate::services::workspace_service;

#[tauri::command]
pub fn create_workspace(
    app_handle: tauri::AppHandle,
    runtime: tauri::State<'_, Mutex<WorkspaceRuntimeState>>,
    environment_state: tauri::State<'_, Mutex<AppState>>,
    cookie_state: tauri::State<'_, Mutex<CookieRuntimeState>>,
    path: String,
    name: String,
) -> Result<WorkspaceSession, String> {
    let session = workspace_service::create_workspace(&PathBuf::from(path), &name)?;
    activate_workspace(
        &app_handle,
        &runtime,
        &environment_state,
        &cookie_state,
        session,
    )
}

#[tauri::command]
pub fn open_workspace(
    app_handle: tauri::AppHandle,
    runtime: tauri::State<'_, Mutex<WorkspaceRuntimeState>>,
    environment_state: tauri::State<'_, Mutex<AppState>>,
    cookie_state: tauri::State<'_, Mutex<CookieRuntimeState>>,
    path: String,
) -> Result<WorkspaceSession, String> {
    let session = workspace_service::open_workspace(&PathBuf::from(path))?;
    activate_workspace(
        &app_handle,
        &runtime,
        &environment_state,
        &cookie_state,
        session,
    )
}

#[tauri::command]
pub fn close_workspace(
    runtime: tauri::State<'_, Mutex<WorkspaceRuntimeState>>,
    environment_state: tauri::State<'_, Mutex<AppState>>,
    cookie_state: tauri::State<'_, Mutex<CookieRuntimeState>>,
) -> Result<(), String> {
    runtime
        .lock()
        .map_err(|error| error.to_string())?
        .current_workspace = None;
    *environment_state
        .lock()
        .map_err(|error| error.to_string())? = AppState::default();
    cookie_state
        .lock()
        .map_err(|error| error.to_string())?
        .deactivate();
    Ok(())
}

#[tauri::command]
pub fn get_current_workspace(
    runtime: tauri::State<'_, Mutex<WorkspaceRuntimeState>>,
) -> Result<Option<WorkspaceSession>, String> {
    Ok(runtime
        .lock()
        .map_err(|error| error.to_string())?
        .current_workspace
        .clone())
}

#[tauri::command]
pub fn list_recent_workspaces(
    runtime: tauri::State<'_, Mutex<WorkspaceRuntimeState>>,
) -> Result<Vec<RecentWorkspace>, String> {
    Ok(runtime
        .lock()
        .map_err(|error| error.to_string())?
        .settings
        .recent_workspaces
        .clone())
}

#[tauri::command]
pub fn get_http_settings(
    runtime: tauri::State<'_, Mutex<WorkspaceRuntimeState>>,
) -> Result<GlobalHttpSettings, String> {
    Ok(runtime
        .lock()
        .map_err(|error| error.to_string())?
        .settings
        .http
        .clone())
}

#[tauri::command]
pub fn update_http_settings(
    app_handle: tauri::AppHandle,
    runtime: tauri::State<'_, Mutex<WorkspaceRuntimeState>>,
    mut settings: GlobalHttpSettings,
    proxy_password: Option<String>,
) -> Result<GlobalHttpSettings, String> {
    settings.max_redirects = settings.max_redirects.clamp(1, 100);
    settings.request_timeout_ms = settings.request_timeout_ms.min(3_600_000);
    settings.max_response_size_mb = settings.max_response_size_mb.min(1_024);
    settings.proxy_url = settings.proxy_url.trim().to_string();
    settings.proxy_bypass = settings.proxy_bypass.trim().to_string();
    settings.proxy_username = settings.proxy_username.trim().to_string();

    if settings.proxy_mode == ProxyMode::Manual {
        if !settings.proxy_use_http && !settings.proxy_use_https {
            return Err("Enable the manual proxy for HTTP, HTTPS, or both.".into());
        }
        let parsed = url::Url::parse(&settings.proxy_url)
            .map_err(|error| format!("Enter a valid proxy server URL: {error}"))?;
        if !matches!(parsed.scheme(), "http" | "https") || parsed.host_str().is_none() {
            return Err("Proxy server must be an HTTP or HTTPS URL with a host.".into());
        }
    }

    if settings.proxy_mode == ProxyMode::Manual && settings.proxy_auth_enabled {
        if settings.proxy_username.is_empty() {
            return Err("Enter a username when enabling proxy authentication.".into());
        }
        if let Some(password) = proxy_password.filter(|value| !value.is_empty()) {
            crate::services::proxy_credential_service::store_password(&password)?;
        } else if crate::services::proxy_credential_service::load_password()?.is_none() {
            return Err("Enter a password when enabling proxy authentication.".into());
        }
    } else if settings.proxy_mode == ProxyMode::Manual {
        crate::services::proxy_credential_service::delete_password()?;
    }
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|error| format!("Failed to get app data directory: {error}"))?;
    let mut runtime = runtime.lock().map_err(|error| error.to_string())?;
    runtime.settings.http = settings.clone();
    workspace_service::save_settings(&app_data_dir, &runtime.settings)?;
    Ok(settings)
}

pub fn activate_workspace(
    app_handle: &tauri::AppHandle,
    runtime: &Mutex<WorkspaceRuntimeState>,
    environment_state: &Mutex<AppState>,
    cookie_state: &Mutex<CookieRuntimeState>,
    session: WorkspaceSession,
) -> Result<WorkspaceSession, String> {
    let environments: EnvironmentsDocument = read_json(
        &workspace_service::workspace_file(&session, ENVIRONMENTS_FILE),
        "environments document",
    )?;
    cookie_state
        .lock()
        .map_err(|error| error.to_string())?
        .activate(&session)?;

    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|error| format!("Failed to get app data directory: {error}"))?;

    let active_environment_id = {
        let mut runtime_state = runtime.lock().map_err(|error| error.to_string())?;
        let mut settings = runtime_state.settings.clone();
        workspace_service::remember_workspace(&mut settings, &session);
        workspace_service::save_settings(&app_data_dir, &settings)?;

        let active_id = settings
            .active_environment_by_workspace
            .get(&session.manifest.id)
            .filter(|id| {
                environments
                    .environments
                    .iter()
                    .any(|environment| &environment.id == *id)
            })
            .cloned();
        runtime_state.settings = settings;
        runtime_state.current_workspace = Some(session.clone());
        active_id
    };

    *environment_state
        .lock()
        .map_err(|error| error.to_string())? = AppState {
        schema_version: environments.schema_version,
        environments: environments.environments,
        active_environment_id,
    };

    Ok(session)
}
