export interface EnvironmentVariableMetadata {
    enabled: boolean;
    sensitive: boolean;
    description?: string;
    order: number;
}

export interface Environment {
    id: string;
    name: string;
    variables: Record<string, string>;
    variable_metadata?: Record<string, EnvironmentVariableMetadata>;
    workspace_id?: string | null;
    created_at?: string;
    updated_at?: string;
}
