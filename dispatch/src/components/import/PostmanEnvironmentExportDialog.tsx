import type { Environment } from "../../types/environment";
import { isSensitiveName } from "../../services/console/redaction";
import { DialogFrame, PrimaryButton, SecondaryButton } from "../openapi/DialogPrimitives";

interface Props {
  environment: Environment;
  submitting: boolean;
  error: string | null;
  onClose: () => void;
  onExport: (includeSensitiveValues: boolean) => Promise<void>;
}

export function PostmanEnvironmentExportDialog({
  environment,
  submitting,
  error,
  onClose,
  onExport,
}: Props) {
  const sensitiveCount = Object.keys(environment.variables).filter(
    (name) => environment.variable_metadata?.[name]?.sensitive || isSensitiveName(name),
  ).length;

  return (
    <DialogFrame title="Export Postman Environment" onClose={onClose} disabled={submitting}>
      <div className="space-y-4 text-sm text-zinc-300">
        <div className="rounded-lg border border-[#3a3a3a] bg-[#202020] p-4">
          <p className="font-medium text-zinc-100">{environment.name}</p>
          <p className="mt-1 text-xs text-zinc-400">
            {Object.keys(environment.variables).length} variables
            {sensitiveCount > 0 ? ` · ${sensitiveCount} sensitive` : ""}
          </p>
        </div>
        <p className="text-xs leading-5 text-zinc-400">
          Sensitive values are excluded by default. Their variable names and secret metadata remain in the exported file.
        </p>
        {error && <div className="rounded border border-red-500/40 bg-red-500/10 px-3 py-2 text-xs text-red-300">{error}</div>}
        <div className="flex flex-wrap justify-end gap-2">
          <SecondaryButton onClick={onClose} disabled={submitting}>Cancel</SecondaryButton>
          {sensitiveCount > 0 && (
            <SecondaryButton onClick={() => void onExport(true)} disabled={submitting}>
              Include sensitive values
            </SecondaryButton>
          )}
          <PrimaryButton onClick={() => void onExport(false)} disabled={submitting}>
            {submitting ? "Exporting..." : "Export safely"}
          </PrimaryButton>
        </div>
      </div>
    </DialogFrame>
  );
}
