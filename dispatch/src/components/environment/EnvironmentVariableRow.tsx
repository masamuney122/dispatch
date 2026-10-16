export interface EnvironmentVariableDraft {
  id: string;
  key: string;
  value: string;
  enabled: boolean;
  sensitive: boolean;
  description: string;
}

interface EnvironmentVariableRowProps {
  variable: EnvironmentVariableDraft;
  isDuplicate: boolean;
  onChange: (
    field: "key" | "value" | "enabled" | "sensitive" | "description",
    value: string | boolean,
  ) => void;
  onDelete: () => void;
}

export const EnvironmentVariableRow: React.FC<EnvironmentVariableRowProps> = ({
  variable,
  isDuplicate,
  onChange,
  onDelete,
}) => (
  <tr className={`group transition-colors hover:bg-[#202020] ${variable.enabled ? "" : "opacity-60"}`}>
    <td className="w-9 px-2 text-center align-middle">
      <input
        type="checkbox"
        checked={variable.enabled}
        onChange={(event) => onChange("enabled", event.target.checked)}
        aria-label={`${variable.enabled ? "Disable" : "Enable"} ${variable.key || "variable"}`}
        className="h-3.5 w-3.5 accent-sky-500"
      />
    </td>
    <td className="w-[28%] align-top" style={{ padding: "4px 12px" }}>
      <input
        value={variable.key}
        onChange={(event) => onChange("key", event.target.value)}
        placeholder="Variable"
        className={`w-full bg-transparent px-1 py-1 font-mono text-xs outline-none ${
          isDuplicate ? "text-red-400" : "text-zinc-200"
        }`}
        aria-invalid={isDuplicate}
        spellCheck={false}
      />
      {isDuplicate && (
        <p className="px-1 pb-1 text-xs text-red-400">Variable names must be unique.</p>
      )}
    </td>
    <td className="w-[34%] border-l border-[#2e2e2e] align-top" style={{ padding: "4px 12px" }}>
      <div className="flex items-center gap-1">
        <input
          type={variable.sensitive ? "password" : "text"}
          value={variable.value}
          onChange={(event) => onChange("value", event.target.value)}
          placeholder="Value"
          className="min-w-0 flex-1 bg-transparent px-1 py-1 font-mono text-xs text-zinc-200 outline-none"
          spellCheck={false}
        />
        <button
          type="button"
          onClick={() => onChange("sensitive", !variable.sensitive)}
          className={`shrink-0 rounded px-1.5 py-1 text-[10px] ${
            variable.sensitive ? "bg-amber-500/15 text-amber-400" : "text-zinc-500 hover:text-zinc-200"
          }`}
          title={variable.sensitive ? "Sensitive value" : "Mark as sensitive"}
          aria-pressed={variable.sensitive}
        >
          {variable.sensitive ? "Locked" : "Lock"}
        </button>
      </div>
    </td>
    <td className="border-l border-[#2e2e2e] align-top" style={{ padding: "4px 12px" }}>
      <div className="flex items-center">
        <input
          value={variable.description}
          onChange={(event) => onChange("description", event.target.value)}
          placeholder="Description"
          className="min-w-0 flex-1 bg-transparent px-1 py-1 text-xs text-zinc-300 outline-none"
        />
        <button
          type="button"
          onClick={onDelete}
          className="ml-2 shrink-0 p-1 text-sm font-bold text-zinc-500 hover:text-red-400"
          title="Delete variable"
          aria-label="Delete variable"
        >
          ×
        </button>
      </div>
    </td>
  </tr>
);
