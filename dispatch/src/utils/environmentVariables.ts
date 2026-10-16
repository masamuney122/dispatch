import type {
  Environment,
  EnvironmentVariableMetadata,
} from "../types/environment";

export const defaultEnvironmentVariableMetadata = (
  order = 0,
): EnvironmentVariableMetadata => ({
  enabled: true,
  sensitive: false,
  description: "",
  order,
});

export function environmentVariableMetadata(
  environment: Environment,
  name: string,
  order = 0,
): EnvironmentVariableMetadata {
  return {
    ...defaultEnvironmentVariableMetadata(order),
    ...environment.variable_metadata?.[name],
  };
}

export function getEnabledEnvironmentVariables(
  environment: Environment | null | undefined,
): Record<string, string> {
  if (!environment) return {};
  return Object.fromEntries(
    Object.entries(environment.variables).filter(
      ([name]) => environment.variable_metadata?.[name]?.enabled !== false,
    ),
  );
}

export function mergeEnabledEnvironmentVariables(
  environment: Environment,
  enabledVariables: Record<string, string>,
): Record<string, string> {
  const disabled = Object.fromEntries(
    Object.entries(environment.variables).filter(
      ([name]) => environment.variable_metadata?.[name]?.enabled === false,
    ),
  );
  return { ...disabled, ...enabledVariables };
}
