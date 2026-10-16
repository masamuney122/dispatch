
import {
  EnvironmentVariableRow,
  type EnvironmentVariableDraft,
} from "./EnvironmentVariableRow";
import { getDuplicateVariableNames } from "../../utils/environmentVariableUtils";

interface EnvironmentVariablesTableProps {
  variables: EnvironmentVariableDraft[];
  onChange: (variables: EnvironmentVariableDraft[]) => void;
}

const createEmptyVariable = (): EnvironmentVariableDraft => ({
  id: crypto.randomUUID(),
  key: "",
  value: "",
  enabled: true,
  sensitive: false,
  description: "",
});

export const EnvironmentVariablesTable: React.FC<
  EnvironmentVariablesTableProps
> = ({ variables, onChange }) => {
  const duplicateNames = getDuplicateVariableNames(variables);

  const updateVariable = (
    index: number,
    field: "key" | "value" | "enabled" | "sensitive" | "description",
    value: string | boolean,
  ) => {
    const next = variables.map((variable, variableIndex) =>
      variableIndex === index ? { ...variable, [field]: value } : variable
    );
    onChange(next);
  };

  const addVariable = () => {
    onChange([...variables, createEmptyVariable()]);
  };

  const deleteVariable = (index: number) => {
    onChange(
      variables.filter((_, variableIndex) => variableIndex !== index)
    );
  };

  return (
    <div className="w-full overflow-hidden rounded-md border border-[#3a3a3a] bg-[#212121]">
      <table className="w-full table-fixed border-collapse text-left text-xs">
        <thead>
          <tr className="select-none border-b border-[#343434] bg-[#242424] text-xs font-semibold text-zinc-400">
            <th className="w-9 px-2" aria-label="Enabled" />
            <th className="w-[28%]" style={{ padding: "8px 12px" }}>
              Variable
            </th>
            <th className="w-[34%] border-l border-[#2e2e2e]" style={{ padding: "8px 12px" }}>
              Value
            </th>
            <th className="border-l border-[#2e2e2e]" style={{ padding: "8px 12px" }}>
              <div className="flex items-center justify-between"><span>Description</span><button type="button" onClick={addVariable} className="cursor-pointer font-medium text-sky-400 hover:underline">+ Add</button></div>
            </th>
          </tr>
        </thead>
        <tbody className="divide-y divide-[#282828]">
          {variables.length === 0 ? (
            <tr
              onClick={addVariable}
              className="h-9 cursor-text transition-colors hover:bg-[#242424]"
            >
              <td className="w-9 px-2" />
              <td className="font-mono text-zinc-600" style={{ padding: "8px 12px" }}>
                Variable
              </td>
              <td className="border-l border-[#2e2e2e] font-mono text-zinc-600" style={{ padding: "8px 12px" }}>
                Value
              </td>
              <td className="border-l border-[#2e2e2e] text-zinc-600" style={{ padding: "8px 12px" }}>Description</td>
            </tr>
          ) : (
            variables.map((variable, index) => {
              const key = variable.key.trim();
              return (
                <EnvironmentVariableRow
                  key={variable.id}
                  variable={variable}
                  isDuplicate={Boolean(key && duplicateNames.has(key))}
                  onChange={(field, value) =>
                    updateVariable(index, field, value)
                  }
                  onDelete={() => deleteVariable(index)}
                />
              );
            })
          )}
        </tbody>
      </table>
    </div>
  );
};
