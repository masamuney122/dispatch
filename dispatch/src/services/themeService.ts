import { setPlatformTheme } from "./platformService";

export type AppTheme = "dark" | "light";
export type ThemePreference = AppTheme;

const THEME_STORAGE_KEY = "dispatch.theme";

const isThemePreference = (value: string | null): value is ThemePreference =>
  value === "dark" || value === "light";

function getSystemTheme(): AppTheme {
  return window.matchMedia?.("(prefers-color-scheme: light)").matches ? "light" : "dark";
}

export function getStoredThemePreference(): ThemePreference {
  try {
    const storedTheme = window.localStorage.getItem(THEME_STORAGE_KEY);
    if (storedTheme === "system") return getSystemTheme();
    return isThemePreference(storedTheme) ? storedTheme : "dark";
  } catch {
    return "dark";
  }
}

function setResolvedTheme(theme: AppTheme): void {
  document.documentElement.dataset.theme = theme;
  document.documentElement.style.colorScheme = theme;
  void setPlatformTheme(theme).catch(() => undefined);
}

export function applyThemePreference(preference: ThemePreference): void {
  setResolvedTheme(preference);

  try {
    window.localStorage.setItem(THEME_STORAGE_KEY, preference);
  } catch {
    // The selected theme still applies for the current session when storage is unavailable.
  }
}

export function initializeTheme(): ThemePreference {
  const preference = getStoredThemePreference();
  applyThemePreference(preference);
  return preference;
}
