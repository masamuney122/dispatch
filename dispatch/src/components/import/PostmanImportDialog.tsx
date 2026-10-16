import { useState } from "react";
import { choosePostmanFile } from "../../services/postmanService";
import type { PostmanImportPreview, PostmanSource } from "../../types/postman";
import { DialogFrame, PrimaryButton, SecondaryButton, inputClass } from "../openapi/DialogPrimitives";

interface Props {
  submitting: boolean;
  error: string | null;
  onInspect: (source: PostmanSource) => Promise<PostmanImportPreview>;
  onResetError: () => void;
  onClose: () => void;
  onConfirm: (source: PostmanSource, setActiveAfterImport: boolean) => Promise<void>;
}

export function PostmanImportDialog({ submitting, error, onInspect, onResetError, onClose, onConfirm }: Props) {
  const [source, setSource] = useState<PostmanSource | null>(null);
  const [preview, setPreview] = useState<PostmanImportPreview | null>(null);
  const [inspecting, setInspecting] = useState(false);
  const [inspectError, setInspectError] = useState<string | null>(null);
  const [setActiveAfterImport, setSetActiveAfterImport] = useState(false);
  const disabled = submitting || inspecting;

  const choose = async () => {
    const selected = await choosePostmanFile();
    if (!selected) return;
    setSource(selected); setPreview(null); setInspectError(null); setSetActiveAfterImport(false); onResetError();
    setInspecting(true);
    try { setPreview(await onInspect(selected)); }
    catch (reason) { setInspectError(reason instanceof Error ? reason.message : String(reason)); }
    finally { setInspecting(false); }
  };

  return (
    <DialogFrame title="Import Postman" onClose={onClose} disabled={disabled} wide>
      <div className="space-y-4">
        <div className="flex gap-2">
          <div className={`${inputClass} flex min-w-0 flex-1 items-center font-mono text-zinc-400`}>
            <span className="truncate">{source?.name || "No Postman JSON file selected"}</span>
          </div>
          <SecondaryButton onClick={() => void choose()} disabled={disabled}>Select file</SecondaryButton>
        </div>
        {inspecting && <p className="text-xs text-zinc-400">Analyzing Postman document...</p>}
        {preview && (
          <div className="space-y-3 rounded-lg border border-[#3a3a3a] bg-[#202020] p-4 text-xs">
            <div className="flex items-center justify-between"><span className="text-zinc-400">Type</span><span className="capitalize text-zinc-100">{preview.kind}</span></div>
            <div className="flex items-center justify-between"><span className="text-zinc-400">Name</span><span className="text-zinc-100">{preview.title}</span></div>
            {preview.kind === "collection" && <>
              <div className="grid grid-cols-3 gap-3 border-t border-[#3a3a3a] pt-3 text-center">
                <Metric label="Requests" value={preview.request_count} /><Metric label="Folders" value={preview.folder_count} /><Metric label="Scripts" value={preview.script_count} />
              </div>
            </>}
            {preview.kind === "environment" ? (
              <>
                <div className="grid grid-cols-4 gap-2 border-t border-[#3a3a3a] pt-3 text-center">
                  <Metric label="Variables" value={preview.variable_count} />
                  <Metric label="Enabled" value={preview.enabled_variable_count} />
                  <Metric label="Disabled" value={preview.disabled_variable_count} />
                  <Metric label="Sensitive" value={preview.sensitive_variable_count} />
                </div>
                <label className="flex items-center gap-2 border-t border-[#3a3a3a] pt-3 text-zinc-300">
                  <input type="checkbox" checked={setActiveAfterImport} onChange={(event) => setSetActiveAfterImport(event.target.checked)} className="accent-sky-500" />
                  Set as active environment after import
                </label>
                {preview.empty_variable_count > 0 && (
                  <p className="rounded border border-amber-500/30 bg-amber-500/10 px-3 py-2 text-amber-300">
                    {preview.empty_variable_count} variable value(s) are already empty in the selected Postman file.
                  </p>
                )}
              </>
            ) : (
              <div className="flex items-center justify-between"><span className="text-zinc-400">Variables</span><span>{preview.variable_count}</span></div>
            )}
            {preview.warnings.length > 0 && <p className="border-t border-[#3a3a3a] pt-3 text-amber-400">{preview.warnings.length} compatibility warning(s) will be reported after import.</p>}
          </div>
        )}
        {(inspectError || error) && <div className="rounded border border-red-500/40 bg-red-500/10 px-3 py-2 text-xs text-red-300">{inspectError || error}</div>}
        <div className="flex justify-end gap-2">
          <SecondaryButton onClick={onClose} disabled={disabled}>Cancel</SecondaryButton>
          <PrimaryButton onClick={() => source && void onConfirm(source, setActiveAfterImport)} disabled={disabled || !source || !preview}>{submitting ? "Importing..." : "Import"}</PrimaryButton>
        </div>
      </div>
    </DialogFrame>
  );
}

function Metric({ label, value }: { label: string; value: number }) {
  return <div><div className="text-lg font-semibold text-zinc-100">{value}</div><div className="text-zinc-500">{label}</div></div>;
}
