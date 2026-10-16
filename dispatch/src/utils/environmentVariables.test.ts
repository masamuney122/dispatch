import { describe, expect, it } from "vitest";
import type { Environment } from "../types/environment";
import {
  getEnabledEnvironmentVariables,
  mergeEnabledEnvironmentVariables,
} from "./environmentVariables";

const environment: Environment = {
  id: "environment-1",
  name: "Local",
  variables: { enabled: "one", disabled: "two" },
  variable_metadata: {
    disabled: {
      enabled: false,
      sensitive: true,
      description: "Kept but not resolved",
      order: 1,
    },
  },
};

describe("environment variable resolution", () => {
  it("excludes disabled variables while treating legacy variables as enabled", () => {
    expect(getEnabledEnvironmentVariables(environment)).toEqual({ enabled: "one" });
  });

  it("preserves disabled values when scripts update the effective environment", () => {
    expect(mergeEnabledEnvironmentVariables(environment, { enabled: "changed", added: "three" })).toEqual({
      disabled: "two",
      enabled: "changed",
      added: "three",
    });
  });
});
