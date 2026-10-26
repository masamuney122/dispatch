import { useState } from "react";
import {
  applyThemePreference,
  getStoredThemePreference,
  type AppTheme,
  type ThemePreference,
} from "../../services/themeService";
import type { GlobalHttpSettings } from "../../types/httpSettings";
import {
  GeneralHttpSettingsEditor,
  ProxySettingsEditor,
} from "./HttpSettingsEditor";

interface Props {
  settings: GlobalHttpSettings;
  platform: "desktop" | "web";
  saving: boolean;
  error: string | null;
  onClose: () => void;
  onSave: (settings: GlobalHttpSettings, proxyPassword?: string) => Promise<void>;
}

type SettingsPage = "general" | "proxy" | "theme";

const SettingsIcon = () => (
  <svg viewBox="0 0 24 24" aria-hidden="true" className="h-4 w-4 fill-none stroke-current" strokeWidth="1.8">
    <circle cx="12" cy="12" r="3" />
    <path d="M19.4 15a1.7 1.7 0 0 0 .34 1.88l.06.06-2.83 2.83-.06-.06A1.7 1.7 0 0 0 15 19.4a1.7 1.7 0 0 0-1 .6 1.7 1.7 0 0 0-.4 1.1V21h-4v-.09A1.7 1.7 0 0 0 8.55 19.4a1.7 1.7 0 0 0-1.88.34l-.06.06-2.83-2.83.06-.06A1.7 1.7 0 0 0 4.6 15a1.7 1.7 0 0 0-.6-1 1.7 1.7 0 0 0-1.1-.4H3v-4h.09A1.7 1.7 0 0 0 4.6 8.55a1.7 1.7 0 0 0-.34-1.88l-.06-.06 2.83-2.83.06.06A1.7 1.7 0 0 0 9 4.6a1.7 1.7 0 0 0 1-.6 1.7 1.7 0 0 0 .4-1.1V3h4v.09A1.7 1.7 0 0 0 15.45 4.6a1.7 1.7 0 0 0 1.88-.34l.06-.06 2.83 2.83-.06.06A1.7 1.7 0 0 0 19.4 9c.1.38.31.73.6 1 .3.27.68.4 1.1.4H21v4h-.09A1.7 1.7 0 0 0 19.4 15Z" />
  </svg>
);

const ProxyIcon = () => (
  <svg viewBox="0 0 24 24" aria-hidden="true" className="h-4 w-4 fill-none stroke-current" strokeWidth="1.8">
    <rect x="3" y="4" width="18" height="6" rx="1.5" />
    <rect x="3" y="14" width="18" height="6" rx="1.5" />
    <path d="M7 7h.01M7 17h.01M11 7h7M11 17h7" strokeLinecap="round" />
  </svg>
);

const ThemeIcon = () => (
  <svg viewBox="0 0 24 24" aria-hidden="true" className="h-4 w-4 fill-none stroke-current" strokeWidth="1.8">
    <path d="M12 3a9 9 0 1 0 0 18h1.2a1.8 1.8 0 0 0 0-3.6h-.7a1.5 1.5 0 0 1 0-3H15A6 6 0 0 0 15 3h-3Z" />
    <circle cx="7.5" cy="10" r=".8" fill="currentColor" stroke="none" />
    <circle cx="9.5" cy="6.7" r=".8" fill="currentColor" stroke="none" />
    <circle cx="14" cy="6.5" r=".8" fill="currentColor" stroke="none" />
  </svg>
);

const NAVIGATION: Array<{
  id: SettingsPage;
  label: string;
  icon: () => React.ReactNode;
}> = [
  { id: "general", label: "Global", icon: SettingsIcon },
  { id: "proxy", label: "Proxy", icon: ProxyIcon },
  { id: "theme", label: "Theme", icon: ThemeIcon },
];

function ThemePreview({ theme }: { theme: AppTheme }) {
  const dark = theme === "dark";
  const colors = dark
    ? { shell: "#171717", border: "#444", chrome: "#202020", muted: "#616161", text: "#a1a1aa", field: "#242424" }
    : { shell: "#f7f9fc", border: "#cbd5e1", chrome: "#e8edf4", muted: "#9aa9bb", text: "#52657d", field: "#ffffff" };

  return (
    <div className="h-32 overflow-hidden rounded-lg border" style={{ background: colors.shell, borderColor: colors.border }}>
      <div className="flex h-7 items-center gap-1.5 border-b px-3" style={{ background: colors.chrome, borderColor: colors.border }}>
        <span className="h-1.5 w-5 rounded-full" style={{ background: colors.muted }} />
        <span className="h-1.5 w-5 rounded-full" style={{ background: colors.muted }} />
        <span className="ml-auto h-2 w-2 rounded-full bg-[#ff6c37]" />
      </div>
      <div className="space-y-3 p-4">
        <span className="block h-2 w-16 rounded-full" style={{ background: colors.text }} />
        <div className="flex h-7 items-center rounded border px-2" style={{ background: colors.field, borderColor: colors.border }}>
          <span className="h-1.5 w-1/2 rounded-full" style={{ background: colors.muted }} />
          <span className="ml-auto h-4 w-8 rounded bg-[#ff6c37]" />
        </div>
        <div className="flex gap-2">
          <span className="h-1.5 w-8 rounded-full bg-[#ff6c37]" />
          <span className="h-1.5 w-10 rounded-full" style={{ background: colors.muted }} />
          <span className="h-1.5 w-10 rounded-full" style={{ background: colors.muted }} />
        </div>
      </div>
    </div>
  );
}

function ThemeSettings({
  preference,
  onChange,
}: {
  preference: ThemePreference;
  onChange: (preference: ThemePreference) => void;
}) {
  return (
    <div>
      <h3 className="text-sm font-bold text-zinc-100">Theme selection</h3>

      <div className="mt-5 grid grid-cols-2 gap-5">
        {(["light", "dark"] as const).map((theme) => {
          const selected = preference === theme;
          return (
            <button key={theme} type="button" onClick={() => onChange(theme)} className="text-left">
              <div className={`rounded-xl border-2 p-1 transition ${selected ? "border-[#ff6c37]" : "border-transparent hover:border-[#555]"}`}>
                <ThemePreview theme={theme} />
              </div>
              <div className="mt-2 flex items-center gap-2 px-1">
                <span className={`flex h-4 w-4 items-center justify-center rounded-full border ${selected ? "border-[#ff6c37]" : "border-zinc-600"}`}>
                  {selected && <span className="h-2 w-2 rounded-full bg-[#ff6c37]" />}
                </span>
                <span className="text-xs font-medium text-zinc-300">{theme === "light" ? "Light" : "Dark"}</span>
              </div>
            </button>
          );
        })}
      </div>
    </div>
  );
}

export function GlobalSettingsDialog({ settings, platform, saving, error, onClose, onSave }: Props) {
  const [draft, setDraft] = useState(settings);
  const [proxyPassword, setProxyPassword] = useState("");
  const [page, setPage] = useState<SettingsPage>("general");
  const [initialTheme] = useState(getStoredThemePreference);
  const [themePreference, setThemePreference] = useState(initialTheme);

  const changeTheme = (preference: ThemePreference) => {
    setThemePreference(preference);
    applyThemePreference(preference);
  };

  const cancel = () => {
    applyThemePreference(initialTheme);
    onClose();
  };

  const title = page === "general" ? "Global" : page === "proxy" ? "Proxy" : "Theme";

  return (
    <div className="fixed inset-0 z-[350] flex items-center justify-center bg-black/65 p-6" onMouseDown={() => !saving && cancel()}>
      <div className="flex h-[min(620px,82vh)] w-full max-w-[900px] overflow-hidden rounded-xl border border-[#414141] bg-[#242424] shadow-2xl" onMouseDown={(event) => event.stopPropagation()}>
        <aside className="w-44 shrink-0 border-r border-[#393939] bg-[#202020] p-3">
          <p className="px-3 pb-3 pt-1 text-[11px] font-bold tracking-[0.12em] text-zinc-600">SETTINGS</p>
          <nav className="space-y-1" aria-label="Settings pages">
            {NAVIGATION.map((item) => {
              const Icon = item.icon;
              const active = page === item.id;
              return (
                <button
                  key={item.id}
                  type="button"
                  onClick={() => setPage(item.id)}
                  className={`flex h-9 w-full items-center gap-3 rounded-lg px-3 text-left text-sm transition ${active ? "dispatch-settings-nav-active bg-[#353535] font-semibold text-zinc-100" : "text-zinc-500 hover:bg-[#2b2b2b] hover:text-zinc-300"}`}
                >
                  <Icon />
                  {item.label}
                </button>
              );
            })}
          </nav>
        </aside>

        <div className="flex min-w-0 flex-1 flex-col">
          <header className="flex h-14 shrink-0 items-center justify-between border-b border-[#393939] px-6">
            <h2 className="text-lg font-bold text-zinc-100">{title}</h2>
            <button type="button" disabled={saving} onClick={cancel} className="flex h-8 w-8 items-center justify-center rounded text-2xl font-light text-zinc-500 hover:bg-[#333] hover:text-white disabled:opacity-40" aria-label="Close settings">×</button>
          </header>

          <main className="min-h-0 flex-1 overflow-y-auto px-6 py-4">
            {page === "general" && (
              <section>
                <h3 className="mb-2 text-sm font-bold text-zinc-100">Request</h3>
                <GeneralHttpSettingsEditor value={draft} onChange={setDraft} platform={platform} />
              </section>
            )}
            {page === "proxy" && (
              <ProxySettingsEditor value={draft} onChange={setDraft} platform={platform} proxyPassword={proxyPassword} onProxyPasswordChange={setProxyPassword} />
            )}
            {page === "theme" && <ThemeSettings preference={themePreference} onChange={changeTheme} />}
          </main>

          <footer className="shrink-0 border-t border-[#393939] px-6 py-3">
            {error && <p className="mb-3 rounded-lg border border-red-900/60 bg-red-950/20 p-3 text-xs text-red-300">{error}</p>}
            <div className="flex justify-end gap-2">
              <button type="button" disabled={saving} onClick={cancel} className="h-9 rounded-lg border border-[#464646] bg-[#2c2c2c] px-4 text-xs font-semibold text-zinc-300 hover:bg-[#333] disabled:opacity-40">Cancel</button>
              <button type="button" disabled={saving} onClick={() => void onSave(draft, proxyPassword)} className="h-9 rounded-lg bg-[#ff6c37] px-5 text-xs font-bold text-white hover:bg-[#ff7a47] disabled:opacity-40">{saving ? "Saving..." : "Save"}</button>
            </div>
          </footer>
        </div>
      </div>
    </div>
  );
}
