import { useState } from "react";
import { exportCollectionPostman, exportEnvironmentPostman, importPostman, inspectPostman } from "../services/postmanService";
import { flushWorkspaceChanges } from "../services/workspaceLifecycle";
import type { Collection } from "../types/collection";
import type { Environment } from "../types/environment";
import type { PostmanSource } from "../types/postman";

interface Options {
  refreshCollections: () => Promise<void>;
  refreshEnvironments: () => Promise<void>;
  showCollections: () => void;
  openEnvironment: (environment: Environment) => void;
  activateEnvironment: (environmentId: string) => Promise<void>;
}

export function usePostmanWorkflow({ refreshCollections, refreshEnvironments, showCollections, openEnvironment, activateEnvironment }: Options) {
  const [importOpen, setImportOpen] = useState(false);
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [environmentToExport, setEnvironmentToExport] = useState<Environment | null>(null);

  const confirmImport = async (source: PostmanSource, setActiveAfterImport: boolean) => {
    setSubmitting(true); setError(null);
    try {
      await flushWorkspaceChanges();
      const result = await importPostman(source);
      await Promise.all([refreshCollections(), refreshEnvironments()]);
      if (result.kind === "environment" && result.environment) {
        if (setActiveAfterImport) await activateEnvironment(result.environment.id);
        openEnvironment(result.environment);
      }
      else showCollections();
      setImportOpen(false);
      if (result.warnings.length) window.alert(warningMessage("Postman import completed", result.warnings));
    } catch (reason) { setError(reason instanceof Error ? reason.message : String(reason)); }
    finally { setSubmitting(false); }
  };

  const exportCollection = async (collection: Collection) => {
    try {
      await flushWorkspaceChanges();
      const result = await exportCollectionPostman(collection);
      if (!result.cancelled && result.warnings.length) window.alert(warningMessage("Postman export completed", result.warnings));
    } catch (reason) { window.alert(reason instanceof Error ? reason.message : String(reason)); }
  };

  const exportEnvironment = async (includeSensitiveValues: boolean) => {
    if (!environmentToExport) return;
    setSubmitting(true); setError(null);
    try {
      await flushWorkspaceChanges();
      const result = await exportEnvironmentPostman(environmentToExport, { includeSensitiveValues });
      if (!result.cancelled) {
        setEnvironmentToExport(null);
        if (result.warnings.length) window.alert(warningMessage("Postman environment export completed", result.warnings));
      }
    } catch (reason) { setError(reason instanceof Error ? reason.message : String(reason)); }
    finally { setSubmitting(false); }
  };

  return {
    importOpen, submitting, error, inspect: inspectPostman,
    openImport: () => { setError(null); setImportOpen(true); },
    closeImport: () => { if (!submitting) setImportOpen(false); },
    resetError: () => setError(null), confirmImport, exportCollection,
    environmentToExport,
    openEnvironmentExport: (environment: Environment) => { setError(null); setEnvironmentToExport(environment); },
    closeEnvironmentExport: () => { if (!submitting) setEnvironmentToExport(null); },
    exportEnvironment,
  };
}

function warningMessage(title: string, warnings: Array<{ message: string; location?: string }>): string {
  const visible = warnings.slice(0, 5).map((warning) => `• ${warning.message}${warning.location ? ` (${warning.location})` : ""}`).join("\n");
  return `${title} with ${warnings.length} warning(s):\n\n${visible}${warnings.length > 5 ? `\n• +${warnings.length - 5} more` : ""}`;
}
