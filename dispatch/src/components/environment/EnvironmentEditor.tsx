import { useMemo, useState } from "react";
import type { Environment, EnvironmentVariableMetadata } from "../../types/environment";
import { EnvironmentVariablesTable } from "./EnvironmentVariablesTable";
import type { EnvironmentVariableDraft } from "./EnvironmentVariableRow";
import { getDuplicateVariableNames } from "../../utils/environmentVariableUtils";

interface EnvironmentEditorProps {
  environment: Environment;
  saveError: string | null;
  onChange: (
    id: string,
    name: string,
    variables: Record<string, string>,
    variableMetadata?: Record<string, EnvironmentVariableMetadata>,
  ) => void;
}

const createDraftVariables = (environment: Environment): EnvironmentVariableDraft[] =>
  Object.entries(environment.variables)
    .map(([key, value], index) => ({
      key,
      value,
      order: environment.variable_metadata?.[key]?.order ?? index,
    }))
    .sort((left, right) => left.order - right.order)
    .map(({ key, value }) => ({
      id: crypto.randomUUID(),
      key,
      value,
      enabled: environment.variable_metadata?.[key]?.enabled ?? true,
      sensitive: environment.variable_metadata?.[key]?.sensitive ?? false,
      description: environment.variable_metadata?.[key]?.description ?? "",
    }));

const toVariableRecord = (variables: EnvironmentVariableDraft[]) =>
  variables.reduce<Record<string, string>>((record, variable) => {
    const key = variable.key.trim();
    if (key) record[key] = variable.value;
    return record;
  }, {});

const toVariableMetadata = (variables: EnvironmentVariableDraft[]) =>
  variables.reduce<Record<string, EnvironmentVariableMetadata>>((record, variable, order) => {
    const key = variable.key.trim();
    if (key) {
      record[key] = {
        enabled: variable.enabled,
        sensitive: variable.sensitive,
        description: variable.description,
        order,
      };
    }
    return record;
  }, {});

export const EnvironmentEditor: React.FC<EnvironmentEditorProps> = ({
  environment,
  saveError,
  onChange,
}) => {
  const [draftName, setDraftName] = useState(environment.name);
  const [editingName, setEditingName] = useState(false);
  const [draftVariables, setDraftVariables] = useState(() =>
    createDraftVariables(environment)
  );

  const duplicateNames = useMemo(
    () => getDuplicateVariableNames(draftVariables),
    [draftVariables]
  );

  const publishChange = (
    name: string,
    variables: EnvironmentVariableDraft[]
  ) => {
    if (!name.trim() || getDuplicateVariableNames(variables).size > 0) return;
    onChange(
      environment.id,
      name.trim(),
      toVariableRecord(variables),
      toVariableMetadata(variables),
    );
  };

  const handleNameChange = (name: string) => {
    setDraftName(name);
    if (!name.trim()) return;
    onChange(
      environment.id,
      name.trim(),
      duplicateNames.size > 0
        ? environment.variables
        : toVariableRecord(draftVariables),
      duplicateNames.size > 0
        ? environment.variable_metadata
        : toVariableMetadata(draftVariables),
    );
  };

  const finishNameEditing = () => {
    if (!draftName.trim()) setDraftName(environment.name);
    setEditingName(false);
  };

  const handleVariablesChange = (
    variables: EnvironmentVariableDraft[]
  ) => {
    setDraftVariables(variables);
    publishChange(editingName ? draftName : environment.name, variables);
  };

  return (
    <section
      className="flex min-h-full w-full flex-col text-sm"
      style={{
        paddingLeft: "48px",
        paddingRight: "48px",
        paddingTop: "32px",
        paddingBottom: "32px",
      }}
    >
      <div className="mb-6">
        <div className="flex min-h-9 items-center gap-3.5">
          <span className="flex h-8 w-8 items-center justify-center rounded-md border border-[#3a3a3a] bg-[#242424] text-emerald-400">
            <svg
              className="h-[18px] w-[18px]"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={1.8}
                d="M7 7h10M7 12h10M7 17h6M5 3h14a2 2 0 012 2v14a2 2 0 01-2 2H5a2 2 0 01-2-2V5a2 2 0 012-2z"
              />
            </svg>
          </span>
          {editingName ? (
            <input
              autoFocus
              value={draftName}
              onChange={(event) => handleNameChange(event.target.value)}
              onBlur={finishNameEditing}
              onKeyDown={(event) => {
                if (event.key === "Enter") event.currentTarget.blur();
                if (event.key === "Escape") {
                  setDraftName(environment.name);
                  setEditingName(false);
                }
              }}
              className="min-w-0 flex-1 border-b border-[#637083] bg-transparent px-1 py-1.5 text-lg font-semibold text-zinc-100 outline-none"
            />
          ) : (
            <button
              type="button"
              onClick={() => {
                setDraftName(environment.name);
                setEditingName(true);
              }}
              className="group flex min-w-0 items-center gap-2 text-left"
              title="Rename environment"
            >
              <h1 className="truncate text-lg font-semibold text-zinc-100">
                {environment.name}
              </h1>
              <svg
                className="h-3.5 w-3.5 text-zinc-600 transition-colors group-hover:text-zinc-300"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={1.8}
                  d="M15.232 5.232l3.536 3.536M9 11l6.768-6.768a2.5 2.5 0 013.536 3.536L12.536 14.536 8 16l1-5z"
                />
              </svg>
            </button>
          )}
        </div>
      </div>

      <div>
        <EnvironmentVariablesTable
          variables={draftVariables}
          onChange={handleVariablesChange}
        />
      </div>

      {duplicateNames.size > 0 && (
        <p className="mt-4 text-xs text-red-400">
          Resolve duplicate variable names before changes can be saved.
        </p>
      )}
      {saveError && (
        <p className="mt-4 text-xs text-red-400">{saveError}</p>
      )}
    </section>
  );
};
