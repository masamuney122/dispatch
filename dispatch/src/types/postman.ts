import type { Collection } from "./collection";
import type { Environment } from "./environment";

export type PostmanDocumentKind = "collection" | "environment";

export interface PostmanWarning {
  code: string;
  message: string;
  location?: string;
}

export interface PostmanSource {
  name: string;
  content?: string;
  path?: string;
}

export interface PostmanImportPreview {
  kind: PostmanDocumentKind;
  title: string;
  schema: string;
  request_count: number;
  folder_count: number;
  variable_count: number;
  enabled_variable_count: number;
  disabled_variable_count: number;
  sensitive_variable_count: number;
  empty_variable_count: number;
  script_count: number;
  warnings: PostmanWarning[];
}

export interface PostmanImportResult {
  kind: PostmanDocumentKind;
  collection?: Collection;
  environment?: Environment;
  warnings: PostmanWarning[];
}

export interface PostmanExportResult {
  request_count: number;
  variable_count: number;
  warnings: PostmanWarning[];
  cancelled?: boolean;
}

export interface PostmanEnvironmentExportOptions {
  includeSensitiveValues: boolean;
}
