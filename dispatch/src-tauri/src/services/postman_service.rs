use std::{fs, path::Path};

use chrono::Utc;
use dispatch_core::{
    export_collection_postman, export_environment_postman, import_postman, inspect_postman,
    parse_postman, Environment, PostmanDocumentKind, PostmanEnvironmentExportOptions,
    PostmanImportContext,
};

use crate::models::environment::AppState;
use crate::models::postman::{PostmanExportResult, PostmanImportResult, PostmanSource};
use crate::models::workspace::{
    CollectionsDocument, EnvironmentsDocument, WorkspaceSession, COLLECTIONS_FILE,
    ENVIRONMENTS_FILE,
};
use crate::services::storage_service::{read_json, write_bytes_atomic, write_json_atomic};
use crate::services::workspace_service::workspace_file;

fn read_source(source: &PostmanSource) -> Result<(String, String), String> {
    match source {
        PostmanSource::File { path } => {
            let content = fs::read_to_string(path)
                .map_err(|error| format!("Failed to read Postman file: {error}"))?;
            let fallback = Path::new(path)
                .file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("Imported Postman document")
                .to_string();
            Ok((content, fallback))
        }
        PostmanSource::Text { content, name } => Ok((
            content.clone(),
            name.clone()
                .unwrap_or_else(|| "Imported Postman document".into()),
        )),
    }
}

pub fn inspect(source: &PostmanSource) -> Result<dispatch_core::PostmanImportPreview, String> {
    let (content, fallback) = read_source(source)?;
    inspect_postman(&parse_postman(&content)?, &fallback)
}

pub fn import(
    session: &WorkspaceSession,
    app_state: &mut AppState,
    source: &PostmanSource,
) -> Result<PostmanImportResult, String> {
    let (content, _) = read_source(source)?;
    let document = parse_postman(&content)?;
    let collections_path = workspace_file(session, COLLECTIONS_FILE);
    let environments_path = workspace_file(session, ENVIRONMENTS_FILE);
    let mut collections: CollectionsDocument =
        read_json(&collections_path, "collections document")?;
    let mut environments: EnvironmentsDocument =
        read_json(&environments_path, "environments document")?;
    let imported = import_postman(
        &document,
        None,
        &PostmanImportContext {
            collection_id: uuid::Uuid::new_v4().to_string(),
            id_prefix: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now().to_rfc3339(),
        },
    )?;
    let now = Utc::now().to_rfc3339();
    match imported.kind {
        PostmanDocumentKind::Collection => {
            let mut collection = imported.collection.ok_or_else(|| {
                "Postman collection conversion returned no collection".to_string()
            })?;
            collection.name = unique_name(
                &collection.name,
                collections
                    .collections
                    .iter()
                    .map(|item| item.name.as_str()),
            );
            collections.revision = collections.revision.saturating_add(1);
            collections.updated_at = now;
            collections.collections.push(collection.clone());
            write_json_atomic(&collections_path, &collections, "collections document")?;
            Ok(PostmanImportResult {
                kind: PostmanDocumentKind::Collection,
                collection: Some(collection),
                environment: None,
                warnings: imported.warnings,
            })
        }
        PostmanDocumentKind::Environment => {
            let requested = imported
                .environment_name
                .unwrap_or_else(|| "Imported Environment".into());
            let environment = Environment {
                id: uuid::Uuid::new_v4().to_string(),
                name: unique_name(
                    &requested,
                    environments
                        .environments
                        .iter()
                        .map(|item| item.name.as_str()),
                ),
                variables: imported.environment_variables,
                variable_metadata: imported.environment_variable_metadata,
                workspace_id: Some(session.manifest.id.clone()),
                created_at: now.clone(),
                updated_at: now.clone(),
            };
            environments.revision = environments.revision.saturating_add(1);
            environments.updated_at = now;
            environments.environments.push(environment.clone());
            write_json_atomic(&environments_path, &environments, "environments document")?;
            app_state.environments = environments.environments;
            Ok(PostmanImportResult {
                kind: PostmanDocumentKind::Environment,
                collection: None,
                environment: Some(environment),
                warnings: imported.warnings,
            })
        }
    }
}

pub fn export_collection(
    session: &WorkspaceSession,
    collection_id: &str,
    destination: &Path,
) -> Result<PostmanExportResult, String> {
    let document: CollectionsDocument = read_json(
        &workspace_file(session, COLLECTIONS_FILE),
        "collections document",
    )?;
    let collection = document
        .collections
        .iter()
        .find(|item| item.id == collection_id)
        .ok_or_else(|| format!("Collection not found: {collection_id}"))?;
    let exported = export_collection_postman(collection)?;
    write_bytes_atomic(
        destination,
        exported.content.as_bytes(),
        "Postman collection",
    )?;
    Ok(PostmanExportResult {
        request_count: exported.request_count,
        variable_count: exported.variable_count,
        warnings: exported.warnings,
        cancelled: false,
    })
}

pub fn export_environment(
    session: &WorkspaceSession,
    environment_id: &str,
    include_sensitive_values: bool,
    destination: &Path,
) -> Result<PostmanExportResult, String> {
    let document: EnvironmentsDocument = read_json(
        &workspace_file(session, ENVIRONMENTS_FILE),
        "environments document",
    )?;
    let environment = document
        .environments
        .iter()
        .find(|item| item.id == environment_id)
        .ok_or_else(|| format!("Environment not found: {environment_id}"))?;
    let exported = export_environment_postman(
        environment,
        &PostmanEnvironmentExportOptions {
            include_sensitive_values,
            exported_at: Utc::now().to_rfc3339(),
        },
    )?;
    write_bytes_atomic(
        destination,
        exported.content.as_bytes(),
        "Postman environment",
    )?;
    Ok(PostmanExportResult {
        request_count: 0,
        variable_count: exported.variable_count,
        warnings: exported.warnings,
        cancelled: false,
    })
}

fn unique_name<'a>(requested: &str, existing: impl Iterator<Item = &'a str>) -> String {
    let existing = existing.collect::<Vec<_>>();
    if !existing.contains(&requested) {
        return requested.to_string();
    }
    for index in 1.. {
        let candidate = if index == 1 {
            format!("{requested} (Imported)")
        } else {
            format!("{requested} (Imported {index})")
        };
        if !existing.contains(&candidate.as_str()) {
            return candidate;
        }
    }
    unreachable!()
}
