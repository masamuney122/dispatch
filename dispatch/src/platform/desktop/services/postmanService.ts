import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import type { Collection } from "../../../types/collection";
import type { Environment } from "../../../types/environment";
import type { PostmanEnvironmentExportOptions, PostmanExportResult, PostmanImportPreview, PostmanImportResult, PostmanSource } from "../../../types/postman";

const nativeSource = (source: PostmanSource) => source.path
  ? { kind: "file", path: source.path }
  : { kind: "text", content: source.content || "", name: source.name };

export async function choosePostmanFile(): Promise<PostmanSource | null> {
  const selected = await open({ multiple: false, directory: false, title: "Select a Postman collection or environment", filters: [{ name: "Postman JSON", extensions: ["json"] }] });
  return typeof selected === "string" ? { name: selected.split(/[\\/]/).pop() || selected, path: selected } : null;
}

export const inspectPostman = (source: PostmanSource) => invoke<PostmanImportPreview>("inspect_postman", { source: nativeSource(source) });
export const importPostman = (source: PostmanSource) => invoke<PostmanImportResult>("import_postman", { source: nativeSource(source) });

export async function exportCollectionPostman(collection: Collection): Promise<PostmanExportResult> {
  const path = await save({ title: "Save Postman collection", defaultPath: `${collection.name.replace(/[^a-zA-Z0-9._-]+/g, "-")}.postman_collection.json`, filters: [{ name: "Postman Collection", extensions: ["json"] }] });
  if (!path) return { request_count: 0, variable_count: 0, warnings: [], cancelled: true };
  return invoke<PostmanExportResult>("export_collection_postman", { collectionId: collection.id, path });
}

export async function exportEnvironmentPostman(
  environment: Environment,
  options: PostmanEnvironmentExportOptions,
): Promise<PostmanExportResult> {
  const path = await save({
    title: "Save Postman environment",
    defaultPath: `${environment.name.replace(/[^a-zA-Z0-9._-]+/g, "-")}.postman_environment.json`,
    filters: [{ name: "Postman Environment", extensions: ["json"] }],
  });
  if (!path) return { request_count: 0, variable_count: 0, warnings: [], cancelled: true };
  return invoke<PostmanExportResult>("export_environment_postman", {
    environmentId: environment.id,
    includeSensitiveValues: options.includeSensitiveValues,
    path,
  });
}
