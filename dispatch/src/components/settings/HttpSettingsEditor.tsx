import { useState } from "react";
import type {
  GlobalHttpSettings,
  HttpVersionPreference,
  RequestHttpSettings,
} from "../../types/httpSettings";
import { resolveHttpSettings } from "../../types/httpSettings";
import type { CookieCredentials } from "../../types/cookie";

type Platform = "desktop" | "web";

const versionLabel = (value: HttpVersionPreference) =>
  value === "http1" ? "HTTP/1.1" : value === "http2" ? "HTTP/2" : "Auto";

const SettingRow = ({
  title,
  description,
  children,
}: {
  title: string;
  description: string;
  children: React.ReactNode;
}) => (
  <div className="grid grid-cols-[minmax(0,1fr)_210px] gap-8 border-b border-[#343434] py-2.5 last:border-0">
    <div className="min-w-0">
      <p className="text-[13px] font-semibold text-zinc-200">{title}</p>
      <p className="mt-0.5 text-xs leading-4 text-zinc-500">{description}</p>
    </div>
    <div className="flex min-h-8 flex-col items-end justify-center gap-1">{children}</div>
  </div>
);

export const SettingsToggle = ({ checked, onChange, disabled = false, label }: { checked: boolean; onChange: () => void; disabled?: boolean; label?: string }) => (
  <button
    type="button"
    role="switch"
    aria-label={label}
    aria-checked={checked}
    disabled={disabled}
    onClick={onChange}
    className={`relative h-5 w-9 rounded-full transition ${checked ? "bg-[#ff6c37]" : "bg-[#454545]"} disabled:cursor-not-allowed disabled:opacity-40`}
  >
    <span className={`absolute top-0.5 h-4 w-4 rounded-full bg-white transition-all ${checked ? "left-[18px]" : "left-0.5"}`} />
  </button>
);

const BrowserManaged = () => <span className="text-[11px] font-medium text-zinc-500">Browser managed</span>;
const WorkspaceManaged = () => <span className="text-[11px] font-medium text-zinc-500">Workspace cookie jar</span>;

const NumberSetting = ({
  value,
  unit,
  min,
  max,
  placeholder,
  disabled = false,
  onChange,
}: {
  value: number | "";
  unit: string;
  min: number;
  max: number;
  placeholder?: string;
  disabled?: boolean;
  onChange: (value: string) => void;
}) => {
  const [draft, setDraft] = useState<string | null>(null);
  const displayedValue = draft ?? String(value);

  return (
    <label className="flex h-8 overflow-hidden rounded-lg border border-[#484848] bg-[#202020] focus-within:border-[#ff6c37]">
      <input
        type="number"
        min={min}
        max={max}
        disabled={disabled}
        value={displayedValue}
        placeholder={placeholder}
        onFocus={() => setDraft(String(value))}
        onChange={(event) => {
          const next = event.target.value;
          setDraft(next);
          if (next !== "") onChange(next);
        }}
        onBlur={() => {
          onChange(draft === "" || draft == null ? String(min) : draft);
          setDraft(null);
        }}
        className="w-24 bg-transparent px-3 text-right text-xs text-zinc-200 outline-none disabled:opacity-40"
      />
      <span className="flex w-11 items-center justify-center border-l border-[#383838] text-[11px] text-zinc-500">
        {unit}
      </span>
    </label>
  );
};

const clampedNumber = (value: string, min: number, max: number, fallback: number) =>
  Math.min(max, Math.max(min, Number(value) || fallback));

export function GeneralHttpSettingsEditor({
  value,
  onChange,
  platform,
}: {
  value: GlobalHttpSettings;
  onChange: (value: GlobalHttpSettings) => void;
  platform: Platform;
}) {
  const desktop = platform === "desktop";
  return (
    <div>
      <SettingRow title="HTTP version" description="Selects the HTTP protocol used to send requests.">
        {desktop ? (
          <select value={value.http_version} onChange={(event) => onChange({ ...value, http_version: event.target.value as HttpVersionPreference })} className="h-8 w-40 rounded-lg border border-[#484848] bg-[#202020] px-3 text-xs text-zinc-200 outline-none focus:border-[#ff6c37]">
            <option value="auto">Auto</option><option value="http1">HTTP/1.1</option><option value="http2">HTTP/2</option>
          </select>
        ) : <BrowserManaged />}
      </SettingRow>
      <SettingRow title="Request timeout" description="The maximum time to wait for a response. Use 0 for no timeout.">
        <NumberSetting
          value={value.request_timeout_ms}
          unit="ms"
          min={0}
          max={3_600_000}
          onChange={(next) => onChange({ ...value, request_timeout_ms: clampedNumber(next, 0, 3_600_000, 0) })}
        />
      </SettingRow>
      <SettingRow title="Maximum response size" description="Limits the response body size to download. Use 0 for no limit.">
        <NumberSetting
          value={value.max_response_size_mb}
          unit="MB"
          min={0}
          max={1_024}
          onChange={(next) => onChange({ ...value, max_response_size_mb: clampedNumber(next, 0, 1_024, 0) })}
        />
      </SettingRow>
      <SettingRow title="SSL certificate verification" description="Stops requests with invalid or untrusted TLS certificates.">
        {desktop ? <SettingsToggle checked={value.verify_ssl} onChange={() => onChange({ ...value, verify_ssl: !value.verify_ssl })} /> : <BrowserManaged />}
      </SettingRow>
      <SettingRow title="Automatically follow redirects" description="Automatically follows HTTP 3xx responses to their new destination.">
        <SettingsToggle checked={value.follow_redirects} onChange={() => onChange({ ...value, follow_redirects: !value.follow_redirects })} />
      </SettingRow>
      <SettingRow title="Remove Referer on redirect" description="Prevents the Referer header from being sent through the redirect chain.">
        <SettingsToggle checked={value.remove_referer_on_redirect} onChange={() => onChange({ ...value, remove_referer_on_redirect: !value.remove_referer_on_redirect })} />
      </SettingRow>
      <SettingRow title="Maximum number of redirects" description="Sets the maximum number of redirects to follow.">
        {desktop ? (
          <input type="number" min={1} max={100} disabled={!value.follow_redirects} value={value.max_redirects} onChange={(event) => onChange({ ...value, max_redirects: Math.min(100, Math.max(1, Number(event.target.value) || 1)) })} className="h-8 w-24 rounded-lg border border-[#484848] bg-[#202020] px-3 text-right text-xs text-zinc-200 outline-none focus:border-[#ff6c37] disabled:opacity-40" />
        ) : <BrowserManaged />}
      </SettingRow>
      <SettingRow title="Cookies" description="When disabled, cookies are neither stored nor sent with requests.">
        <SettingsToggle
          checked={value.cookies_enabled}
          onChange={() => onChange({ ...value, cookies_enabled: !value.cookies_enabled })}
          label="Enable cookies"
        />
      </SettingRow>
      <SettingRow title="Cookie credentials" description="Controls the policy for sending cookies with requests.">
        {desktop ? <WorkspaceManaged /> : (
          <select disabled={!value.cookies_enabled} value={value.cookie_credentials} onChange={(event) => onChange({ ...value, cookie_credentials: event.target.value as CookieCredentials })} className="h-8 w-40 rounded-lg border border-[#484848] bg-[#202020] px-3 text-xs text-zinc-200 outline-none focus:border-[#ff6c37] disabled:opacity-40">
            <option value="omit">Omit</option><option value="same-origin">Same origin</option><option value="include">Include</option>
          </select>
        )}
      </SettingRow>
    </div>
  );
}

export function ProxySettingsEditor({
  value,
  onChange,
  platform,
  proxyPassword = "",
  onProxyPasswordChange,
}: {
  value: GlobalHttpSettings;
  onChange: (value: GlobalHttpSettings) => void;
  platform: Platform;
  proxyPassword?: string;
  onProxyPasswordChange?: (value: string) => void;
}) {
  const desktop = platform === "desktop";

  return (
    <div>
      {desktop ? (
        <div className="space-y-5">
          <section>
            <h3 className="text-sm font-bold text-zinc-100">Proxy mode</h3>
            <p className="mt-1 text-xs leading-5 text-zinc-500">
              Choose how Dispatch connects when sending API requests.
            </p>
            <div className="mt-4 grid grid-cols-3 gap-2 rounded-lg bg-[#1d1d1d] p-1">
              {([
                ["system", "System proxy"],
                ["manual", "Manual proxy"],
                ["no-proxy", "No proxy"],
              ] as const).map(([mode, label]) => (
                <button
                  key={mode}
                  type="button"
                  onClick={() => onChange({ ...value, proxy_mode: mode })}
                  className={`h-9 rounded-md text-xs font-semibold transition ${
                    value.proxy_mode === mode
                      ? "bg-[#343434] text-white shadow"
                      : "text-zinc-500 hover:text-zinc-300"
                  }`}
                >
                  {label}
                </button>
              ))}
            </div>
          </section>

          {value.proxy_mode === "system" && (
            <section className="rounded-lg border border-[#383838] bg-[#202020] p-5">
              <div>
                <p className="text-xs font-semibold text-zinc-200">Use operating system proxy settings</p>
                <p className="mt-1 text-[11px] leading-4 text-zinc-500">
                  Dispatch uses the current user&apos;s system proxy configuration and the standard HTTP_PROXY,
                  HTTPS_PROXY, ALL_PROXY, and NO_PROXY environment variables.
                </p>
              </div>
            </section>
          )}

          {value.proxy_mode === "no-proxy" && (
            <section className="rounded-lg border border-[#383838] bg-[#202020] p-5">
              <p className="text-xs font-semibold text-zinc-200">Connect directly</p>
              <p className="mt-1 text-[11px] leading-4 text-zinc-500">
                Requests connect directly to their destination. System and environment proxy settings are ignored.
              </p>
            </section>
          )}

          {value.proxy_mode === "manual" && (
            <section className="divide-y divide-[#343434] rounded-lg border border-[#383838] bg-[#202020] px-5">
              <div className="flex items-center justify-between gap-6 py-5">
                <div>
                  <p className="text-xs font-semibold text-zinc-200">Use proxy for</p>
                  <p className="mt-0.5 text-[11px] text-zinc-500">Choose which request protocols use this proxy.</p>
                </div>
                <div className="flex gap-4 text-xs text-zinc-300">
                  <label className="flex items-center gap-2">
                    <input type="checkbox" checked={value.proxy_use_http} onChange={(event) => onChange({ ...value, proxy_use_http: event.target.checked })} className="accent-[#ff6c37]" /> HTTP
                  </label>
                  <label className="flex items-center gap-2">
                    <input type="checkbox" checked={value.proxy_use_https} onChange={(event) => onChange({ ...value, proxy_use_https: event.target.checked })} className="accent-[#ff6c37]" /> HTTPS
                  </label>
                </div>
              </div>

              <label className="grid grid-cols-[minmax(0,1fr)_minmax(260px,1fr)] items-center gap-8 py-5">
                <div className="flex items-center justify-between gap-6">
                  <div>
                    <span className="text-xs font-semibold text-zinc-200">Proxy server</span>
                    <p className="mt-0.5 text-[11px] text-zinc-500">Include the protocol and port.</p>
                  </div>
                </div>
                <input
                  type="url"
                  value={value.proxy_url}
                  onChange={(event) => onChange({ ...value, proxy_url: event.target.value })}
                  placeholder="http://proxy.company.com:8080"
                  className="h-9 w-full rounded-lg border border-[#484848] bg-[#1d1d1d] px-3 text-xs text-zinc-200 outline-none placeholder:text-zinc-600 focus:border-[#ff6c37]"
                />
              </label>

              <div className="py-5">
                <div className="flex items-center justify-between gap-6">
                  <div>
                    <p className="text-xs font-semibold text-zinc-200">This proxy requires authentication</p>
                    <p className="mt-0.5 text-[11px] text-zinc-500">Uses HTTP Basic proxy authentication.</p>
                  </div>
                  <SettingsToggle checked={value.proxy_auth_enabled} onChange={() => onChange({ ...value, proxy_auth_enabled: !value.proxy_auth_enabled })} label="Proxy authentication" />
                </div>

                {value.proxy_auth_enabled && (
                  <div className="mt-4 grid grid-cols-2 gap-3">
                    <label>
                      <span className="text-[11px] font-semibold text-zinc-400">Username</span>
                      <input type="text" autoComplete="username" value={value.proxy_username} onChange={(event) => onChange({ ...value, proxy_username: event.target.value })} className="mt-1.5 h-9 w-full rounded-lg border border-[#484848] bg-[#1d1d1d] px-3 text-xs text-zinc-200 outline-none focus:border-[#ff6c37]" />
                    </label>
                    <label>
                      <span className="text-[11px] font-semibold text-zinc-400">Password</span>
                      <input type="password" autoComplete="current-password" value={proxyPassword} onChange={(event) => onProxyPasswordChange?.(event.target.value)} placeholder="Leave blank to keep the saved password" className="mt-1.5 h-9 w-full rounded-lg border border-[#484848] bg-[#1d1d1d] px-3 text-xs text-zinc-200 outline-none placeholder:text-zinc-600 focus:border-[#ff6c37]" />
                    </label>
                    <p className="col-span-2 text-[10px] leading-4 text-zinc-600">
                      The password is stored in the operating system credential vault, not in the workspace or settings JSON.
                    </p>
                  </div>
                )}
              </div>
              <label className="grid grid-cols-[minmax(0,1fr)_minmax(260px,1fr)] gap-8 py-5">
                <div>
                  <span className="text-xs font-semibold text-zinc-200">Proxy bypass</span>
                  <p className="mt-0.5 text-[11px] leading-4 text-zinc-500">Comma-separated hosts, domains, IP addresses, or CIDR ranges.</p>
                </div>
                <textarea
                  value={value.proxy_bypass}
                  onChange={(event) => onChange({ ...value, proxy_bypass: event.target.value })}
                  placeholder="localhost, 127.0.0.1, ::1, .company.local"
                  rows={3}
                  className="w-full resize-y rounded-lg border border-[#484848] bg-[#1d1d1d] px-3 py-2 text-xs text-zinc-200 outline-none placeholder:text-zinc-600 focus:border-[#ff6c37]"
                />
              </label>
            </section>
          )}
        </div>
      ) : <BrowserManaged />}
    </div>
  );
}

export function RequestHttpSettingsEditor({
  value,
  globalSettings,
  onChange,
  platform,
}: {
  value: RequestHttpSettings;
  globalSettings: GlobalHttpSettings;
  onChange: (value: RequestHttpSettings) => void;
  platform: Platform;
}) {
  const resolved = resolveHttpSettings(globalSettings, value);
  const desktop = platform === "desktop";
  const clear = (key: keyof RequestHttpSettings) => {
    const next = { ...value };
    delete next[key];
    onChange(next);
  };
  const footer = (key: keyof RequestHttpSettings, label: string) =>
    value[key] == null ? (
      <span className="text-[10px] text-zinc-600">Global: {label}</span>
    ) : (
      <button type="button" onClick={() => clear(key)} className="text-[10px] text-[#ff8a61] hover:underline">Use global setting</button>
    );

  return (
    <div className="rounded-lg border border-[#383838] bg-[#222] px-4">
      <SettingRow title="HTTP version" description="Changes the HTTP protocol for this request only.">
        {desktop ? <>
          <select value={value.http_version ?? ""} onChange={(event) => event.target.value ? onChange({ ...value, http_version: event.target.value as HttpVersionPreference }) : clear("http_version")} className="h-8 w-44 rounded-lg border border-[#484848] bg-[#202020] px-3 text-xs text-zinc-200 outline-none focus:border-[#ff6c37]">
            <option value="">Global ({versionLabel(globalSettings.http_version)})</option><option value="auto">Auto</option><option value="http1">HTTP/1.1</option><option value="http2">HTTP/2</option>
          </select>
        </> : <BrowserManaged />}
      </SettingRow>
      <SettingRow title="SSL certificate verification" description="When disabled, self-signed certificates are allowed; use only in development.">
        {desktop ? <><SettingsToggle checked={resolved.verify_ssl} onChange={() => onChange({ ...value, verify_ssl: !resolved.verify_ssl })} />{footer("verify_ssl", globalSettings.verify_ssl ? "On" : "Off")}</> : <BrowserManaged />}
      </SettingRow>
      <SettingRow title="Automatically follow redirects" description="Controls HTTP 3xx redirect behavior for this request.">
        <SettingsToggle checked={resolved.follow_redirects} onChange={() => onChange({ ...value, follow_redirects: !resolved.follow_redirects })} />
        {footer("follow_redirects", globalSettings.follow_redirects ? "On" : "Off")}
      </SettingRow>
      <SettingRow title="Remove Referer on redirect" description="Prevents the Referer header from being sent to the destination server during redirects.">
        <SettingsToggle checked={resolved.remove_referer_on_redirect} onChange={() => onChange({ ...value, remove_referer_on_redirect: !resolved.remove_referer_on_redirect })} />
        {footer("remove_referer_on_redirect", globalSettings.remove_referer_on_redirect ? "On" : "Off")}
      </SettingRow>
      <SettingRow title="Maximum number of redirects" description="Limits the number of redirects this request can follow.">
        {desktop ? <>
          <input type="number" min={1} max={100} disabled={!resolved.follow_redirects} value={value.max_redirects ?? ""} placeholder={String(globalSettings.max_redirects)} onChange={(event) => event.target.value ? onChange({ ...value, max_redirects: Math.min(100, Math.max(1, Number(event.target.value) || 1)) }) : clear("max_redirects")} className="h-8 w-24 rounded-lg border border-[#484848] bg-[#202020] px-3 text-right text-xs text-zinc-200 outline-none focus:border-[#ff6c37] disabled:opacity-40" />
          {footer("max_redirects", String(globalSettings.max_redirects))}
        </> : <BrowserManaged />}
      </SettingRow>
      <SettingRow title="Cookie credentials" description="Changes the cookie-sending policy for the web fetch request.">
        {desktop ? <WorkspaceManaged /> : <>
          <select disabled={!resolved.cookies_enabled} value={value.cookie_credentials ?? ""} onChange={(event) => event.target.value ? onChange({ ...value, cookie_credentials: event.target.value as CookieCredentials }) : clear("cookie_credentials")} className="h-8 w-44 rounded-lg border border-[#484848] bg-[#202020] px-3 text-xs text-zinc-200 outline-none focus:border-[#ff6c37] disabled:opacity-40">
            <option value="">Global ({globalSettings.cookie_credentials})</option><option value="omit">Omit</option><option value="same-origin">Same origin</option><option value="include">Include</option>
          </select>
          {footer("cookie_credentials", globalSettings.cookie_credentials)}
        </>}
      </SettingRow>
    </div>
  );
}
