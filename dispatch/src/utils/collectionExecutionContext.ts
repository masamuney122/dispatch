import type { Collection, SavedRequest } from "../types/collection";
import type { ApiRequest } from "../types/request";
import type { RequestScripts } from "../types/script";

export interface CollectionExecutionContext {
  request: ApiRequest;
  scripts: RequestScripts;
  variables: Record<string, string>;
}

const joinScripts = (parts: Array<string | undefined>) => parts.filter((part) => part?.trim()).join("\n\n");

export function collectionExecutionContext(collection: Collection, saved: SavedRequest): CollectionExecutionContext {
  const folders = [];
  let folderId = saved.folder_id || null;
  const visited = new Set<string>();
  while (folderId && !visited.has(folderId)) {
    visited.add(folderId);
    const folder = collection.folders.find((item) => item.id === folderId);
    if (!folder) break;
    folders.unshift(folder);
    folderId = folder.parent_folder_id || null;
  }
  const inheritedAuth = [...folders].reverse().find((folder) => folder.auth)?.auth ?? collection.auth;
  const request = structuredClone(saved.request);
  if (request.auth === undefined && inheritedAuth !== undefined) request.auth = structuredClone(inheritedAuth);
  const scriptLayers = [collection.scripts, ...folders.map((folder) => folder.scripts), request.scripts];
  return {
    request,
    scripts: {
      pre_request: joinScripts(scriptLayers.map((scripts) => scripts?.pre_request)),
      post_response: joinScripts(scriptLayers.map((scripts) => scripts?.post_response)),
    },
    variables: { ...(collection.variables || {}) },
  };
}
