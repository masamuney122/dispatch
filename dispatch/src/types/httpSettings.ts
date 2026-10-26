import type { CookieCredentials } from "./cookie";

export type HttpVersionPreference = "auto" | "http1" | "http2";
export type ProxyMode = "system" | "manual" | "no-proxy";

export interface GlobalHttpSettings {
  http_version: HttpVersionPreference;
  verify_ssl: boolean;
  follow_redirects: boolean;
  remove_referer_on_redirect: boolean;
  max_redirects: number;
  request_timeout_ms: number;
  max_response_size_mb: number;
  cookies_enabled: boolean;
  cookie_credentials: CookieCredentials;
  proxy_mode: ProxyMode;
  proxy_url: string;
  proxy_use_http: boolean;
  proxy_use_https: boolean;
  proxy_bypass: string;
  proxy_auth_enabled: boolean;
  proxy_username: string;
}

export const REQUEST_HTTP_SETTING_KEYS = [
  "http_version",
  "verify_ssl",
  "follow_redirects",
  "remove_referer_on_redirect",
  "max_redirects",
  "cookie_credentials",
] as const satisfies ReadonlyArray<keyof GlobalHttpSettings>;

export type RequestHttpSettings = Partial<
  Pick<GlobalHttpSettings, (typeof REQUEST_HTTP_SETTING_KEYS)[number]>
>;

export const DEFAULT_HTTP_SETTINGS: GlobalHttpSettings = {
  http_version: "auto",
  verify_ssl: true,
  follow_redirects: true,
  remove_referer_on_redirect: false,
  max_redirects: 10,
  request_timeout_ms: 0,
  max_response_size_mb: 50,
  cookies_enabled: true,
  cookie_credentials: "same-origin",
  proxy_mode: "system",
  proxy_url: "",
  proxy_use_http: true,
  proxy_use_https: true,
  proxy_bypass: "localhost,127.0.0.1,::1",
  proxy_auth_enabled: false,
  proxy_username: "",
};

export function resolveHttpSettings(
  globalSettings: GlobalHttpSettings,
  requestSettings?: RequestHttpSettings,
): GlobalHttpSettings {
  const overrides = Object.fromEntries(
    Object.entries(requestSettings || {}).filter(([, value]) => value !== null && value !== undefined),
  ) as RequestHttpSettings;
  return { ...globalSettings, ...overrides };
}
