interface SecretVisibilityButtonProps {
  revealed: boolean;
  onToggle: () => void;
  label: string;
}

export function SecretVisibilityButton({
  revealed,
  onToggle,
  label,
}: SecretVisibilityButtonProps) {
  return (
    <button
      type="button"
      onClick={onToggle}
      className="flex h-6 w-6 shrink-0 items-center justify-center rounded text-zinc-600 transition-colors hover:bg-[#333] hover:text-zinc-300"
      aria-label={`${revealed ? "Hide" : "Show"} ${label}`}
      title={`${revealed ? "Hide" : "Show"} ${label}`}
    >
      {revealed ? (
        <svg viewBox="0 0 24 24" aria-hidden="true" className="h-3.5 w-3.5 fill-none stroke-current" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
          <path d="M9.88 9.88a3 3 0 1 0 4.24 4.24" />
          <path d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68M6.61 6.61A13.53 13.53 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61M2 2l20 20" />
        </svg>
      ) : (
        <svg viewBox="0 0 24 24" aria-hidden="true" className="h-3.5 w-3.5 fill-none stroke-current" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
          <path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7S2 12 2 12Z" />
          <circle cx="12" cy="12" r="3" />
        </svg>
      )}
    </button>
  );
}
