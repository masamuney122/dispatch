import { saveCollectionsDocument, saveEnvironmentsDocument } from "../adapters/fileSystem/workspaceFileSystem";
import type { Collection } from "../../../types/collection";
import type { Environment } from "../../../types/environment";
import type { PostmanEnvironmentExportOptions, PostmanExportResult, PostmanImportPreview, PostmanImportResult, PostmanSource } from "../../../types/postman";
import { exportPostmanCore, exportPostmanEnvironmentCore, importPostmanCore, inspectPostmanCore, parsePostmanCore } from "./wasmClient";
import { getActiveWorkspace, updateActiveWorkspace } from "./workspaceRuntime";

export function choosePostmanFile(): Promise<PostmanSource | null> {
  return new Promise((resolve) => {
    const input = document.createElement("input");
    input.type = "file";
    input.accept = ".json,application/json";
    input.addEventListener("change", async () => {
      const file = input.files?.[0];
      resolve(file ? { name: file.name, content: await file.text() } : null);
    }, { once: true });
    input.addEventListener("cancel", () => resolve(null), { once: true });
    input.click();
  });
}

const fallbackName = (source: PostmanSource) => source.name.replace(/\.postman_(collection|environment)\.json$|\.json$/i, "");
const uniqueName = (name: string, names: string[]) => {
  if (!names.includes(name)) return name;
  for (let index = 1; ; index += 1) {
    const candidate = `${name} (Imported${index === 1 ? "" : ` ${index}`})`;
    if (!names.includes(candidate)) return candidate;
  }
};
const parseSource = (source: PostmanSource) => {
  if (source.content === undefined) throw new Error("The Postman file could not be read.");
  return parsePostmanCore(source.content);
};

export async function inspectPostman(source: PostmanSource): Promise<PostmanImportPreview> {
  return inspectPostmanCore(await parseSource(source), fallbackName(source));
}

export async function importPostman(source: PostmanSource): Promise<PostmanImportResult> {
  const workspace = getActiveWorkspace();
  const converted = await importPostmanCore(await parseSource(source), undefined);
  if (converted.kind === "collection" && converted.collection) {
    converted.collection.name = uniqueName(converted.collection.name, workspace.bundle.collections.collections.map((item) => item.name));
    const updated = await saveCollectionsDocument(workspace, {
      ...workspace.bundle.collections,
      collections: [...workspace.bundle.collections.collections, converted.collection],
    });
    updateActiveWorkspace(updated);
    return { kind: "collection", collection: converted.collection, warnings: converted.warnings };
  }
  const now = new Date().toISOString();
  const environment: Environment = {
    id: crypto.randomUUID(),
    name: uniqueName(converted.environment_name || fallbackName(source), workspace.bundle.environments.environments.map((item) => item.name)),
    variables: converted.environment_variables,
    variable_metadata: converted.environment_variable_metadata,
    workspace_id: workspace.bundle.manifest.id,
    created_at: now,
    updated_at: now,
  };
  const updated = await saveEnvironmentsDocument(workspace, {
    ...workspace.bundle.environments,
    environments: [...workspace.bundle.environments.environments, environment],
  });
  updateActiveWorkspace(updated);
  return { kind: "environment", environment, warnings: converted.warnings };
}

export async function exportCollectionPostman(collection: Collection): Promise<PostmanExportResult> {
  const result = await exportPostmanCore(collection);
  const safeName = collection.name.replace(/[^a-zA-Z0-9._-]+/g, "-") || "dispatch-collection";
  const url = URL.createObjectURL(new Blob([result.content], { type: `${result.mime_type};charset=utf-8` }));
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = `${safeName}.${result.extension}`;
  document.body.appendChild(anchor);
  anchor.click(); anchor.remove(); URL.revokeObjectURL(url);
  return { request_count: result.request_count, variable_count: result.variable_count, warnings: result.warnings };
}

export async function exportEnvironmentPostman(
  environment: Environment,
  options: PostmanEnvironmentExportOptions,
): Promise<PostmanExportResult> {
  const result = await exportPostmanEnvironmentCore(environment, options.includeSensitiveValues);
  const safeName = environment.name.replace(/[^a-zA-Z0-9._-]+/g, "-") || "dispatch-environment";
  const url = URL.createObjectURL(new Blob([result.content], { type: `${result.mime_type};charset=utf-8` }));
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = `${safeName}.${result.extension}`;
  document.body.appendChild(anchor);
  anchor.click(); anchor.remove(); URL.revokeObjectURL(url);
  return { request_count: 0, variable_count: result.variable_count, warnings: result.warnings };
}
