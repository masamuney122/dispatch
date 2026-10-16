import { describe, expect, it } from "vitest";
import type { Collection } from "../types/collection";
import { collectionExecutionContext } from "./collectionExecutionContext";

describe("collectionExecutionContext", () => {
  it("combines scripts and resolves nearest inherited auth", () => {
    const collection: Collection = {
      id: "c", name: "Imported", created_at: "now", updated_at: "now",
      variables: { base: "collection", shared: "collection" },
      auth: { type: "Bearer", token: "collection-token" },
      scripts: { pre_request: "collectionPre();", post_response: "collectionPost();" },
      folders: [{
        id: "f", name: "Folder", collection_id: "c", order: 0,
        created_at: "now", updated_at: "now",
        auth: { type: "Basic", username: "folder", password: "secret" },
        scripts: { pre_request: "folderPre();", post_response: "folderPost();" },
      }],
      requests: [{
        id: "r", name: "Request", folder_id: "f", order: 0,
        created_at: "now", updated_at: "now",
        request: {
          method: "GET", url: "{{base}}/users", body: "", body_type: "none", headers: {}, form_fields: [],
          scripts: { pre_request: "requestPre();", post_response: "requestPost();" },
        },
      }],
    };
    const result = collectionExecutionContext(collection, collection.requests[0]);
    expect(result.request.auth).toEqual({ type: "Basic", username: "folder", password: "secret" });
    expect(result.scripts.pre_request).toBe("collectionPre();\n\nfolderPre();\n\nrequestPre();");
    expect(result.scripts.post_response).toBe("collectionPost();\n\nfolderPost();\n\nrequestPost();");
    expect(result.variables).toEqual(collection.variables);
  });
});
