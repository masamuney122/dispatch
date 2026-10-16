import {
  export_collection_postman,
  export_environment_postman,
  import_postman,
  inspect_postman,
  parse_postman,
} from "../../wasm/dispatch_web_wasm";
import type { Collection } from "../../../../types/collection";
import type { Environment, EnvironmentVariableMetadata } from "../../../../types/environment";
import type { PostmanImportPreview, PostmanWarning } from "../../../../types/postman";
import { ensureWasmInitialized, wasmError } from "./wasmRuntime";

export interface CorePostmanImportDocument {
  kind: "collection" | "environment";
  collection?: Collection;
  environment_name?: string;
  environment_variables: Record<string, string>;
  environment_variable_metadata: Record<string, EnvironmentVariableMetadata>;
  warnings: PostmanWarning[];
}

export interface CorePostmanExportDocument {
  content: string;
  mime_type: string;
  extension: string;
  request_count: number;
  variable_count: number;
  warnings: PostmanWarning[];
}

export async function parsePostmanCore(content: string): Promise<Record<string, unknown>> {
  await ensureWasmInitialized();
  try { return parse_postman(content) as Record<string, unknown>; }
  catch (error) { throw wasmError(error, "The Postman document could not be parsed"); }
}

export async function inspectPostmanCore(document: Record<string, unknown>, fallback: string): Promise<PostmanImportPreview> {
  await ensureWasmInitialized();
  try { return inspect_postman(document, fallback) as PostmanImportPreview; }
  catch (error) { throw wasmError(error, "The Postman document could not be inspected"); }
}

export async function importPostmanCore(document: Record<string, unknown>, name: string | undefined): Promise<CorePostmanImportDocument> {
  await ensureWasmInitialized();
  try {
    return import_postman(document, name, {
      collection_id: crypto.randomUUID(),
      id_prefix: crypto.randomUUID(),
      timestamp: new Date().toISOString(),
    }) as CorePostmanImportDocument;
  } catch (error) { throw wasmError(error, "The Postman document could not be converted"); }
}

export async function exportPostmanCore(collection: Collection): Promise<CorePostmanExportDocument> {
  await ensureWasmInitialized();
  try { return export_collection_postman(collection) as CorePostmanExportDocument; }
  catch (error) { throw wasmError(error, "The Postman collection could not be generated"); }
}

export async function exportPostmanEnvironmentCore(
  environment: Environment,
  includeSensitiveValues: boolean,
): Promise<CorePostmanExportDocument> {
  await ensureWasmInitialized();
  try {
    return export_environment_postman(environment, {
      include_sensitive_values: includeSensitiveValues,
      exported_at: new Date().toISOString(),
    }) as CorePostmanExportDocument;
  } catch (error) {
    throw wasmError(error, "The Postman environment could not be generated");
  }
}
