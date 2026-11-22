import {emit, listen} from "@tauri-apps/api/event";

export type ColorPalette = string;
export type AppearanceMode = "light" | "dark";

export interface ColorPaletteOption {
  value: ColorPalette;
  label: string;
  className: string;
}

export const colorPalettes: ColorPaletteOption[] = [
  {value: "default", label: "Default", className: ""},
  {value: "labeq", label: "LabEq", className: "theme-labeq"},
];

const themeChangeEvent = "active-theme-change";
const storageKey = "active-color-palette";
const modeStorageKey = "active-appearance-mode";

export function getStoredColorPalette(): ColorPalette {
  const stored = localStorage.getItem(storageKey);
  if (stored && isRegisteredPalette(stored)) {
    return stored;
  }
  return colorPalettes[0].value;
}

function isRegisteredPalette(value: string): value is ColorPalette {
  return colorPalettes.some(palette => palette.value === value);
}

export function applyColorPalette(palette: ColorPalette, notify = true): void {
  const option = colorPalettes.find(item => item.value === palette)
      ?? colorPalettes[0];

  document.documentElement.classList.remove(
      ...colorPalettes
          .map(item => item.className)
          .filter(className => className.length > 0),
  );

  if (option.className) {
    document.documentElement.classList.add(option.className);
  }

  localStorage.setItem(storageKey, option.value);
  if (notify) {
    window.dispatchEvent(new CustomEvent(themeChangeEvent, {
      detail: {
        palette: option.value,
        mode: getStoredAppearanceMode(),
      },
    }));
    void emit(themeChangeEvent, {
      palette: option.value,
      mode: getStoredAppearanceMode(),
    });
  }
}

export function getStoredAppearanceMode(): AppearanceMode {
  return localStorage.getItem(modeStorageKey) === "dark" ? "dark" : "light";
}

export function applyAppearanceMode(mode: AppearanceMode, notify = true): void {
  document.documentElement.classList.toggle("dark", mode === "dark");
  localStorage.setItem(modeStorageKey, mode);
  if (notify) {
    window.dispatchEvent(new CustomEvent(themeChangeEvent, {
      detail: {
        palette: getStoredColorPalette(),
        mode,
      },
    }));
    void emit(themeChangeEvent, {
      palette: getStoredColorPalette(),
      mode,
    });
  }
}

export function applyActiveTheme(
  palette: ColorPalette = getStoredColorPalette(),
  mode: AppearanceMode = getStoredAppearanceMode(),
  notify = true,
): void {
  applyColorPalette(palette, notify);
  applyAppearanceMode(mode, notify);
}

export function initColorPalette(): ColorPalette {
  const palette = getStoredColorPalette();
  applyColorPalette(palette, false);
  return palette;
}

export function watchColorPalette(onChange: (palette: ColorPalette) => void): () => void {
  const handler = (event: StorageEvent) => {
    if (event.key === storageKey) {
      const palette = getStoredColorPalette();
      applyColorPalette(palette);
      onChange(palette);
    }
    if (event.key === modeStorageKey) {
      applyAppearanceMode(getStoredAppearanceMode());
    }
  };
  window.addEventListener("storage", handler);
  let unlistenTheme: (() => void) | undefined;
  void listen<{palette: ColorPalette; mode: AppearanceMode}>(
      "active-theme-change",
      event => {
        applyActiveTheme(event.payload.palette, event.payload.mode, false);
        onChange(event.payload.palette);
      },
  ).then(unlisten => {
    unlistenTheme = unlisten;
  });
  return () => {
    window.removeEventListener("storage", handler);
    unlistenTheme?.();
  };
}

export function watchActiveTheme(
  onChange: (palette: ColorPalette, mode: AppearanceMode) => void,
): () => void {
  const localHandler = (event: Event) => {
    const detail = (event as CustomEvent<{
      palette: ColorPalette;
      mode: AppearanceMode;
    }>).detail;
    onChange(detail.palette, detail.mode);
  };
  window.addEventListener(themeChangeEvent, localHandler);
  const stopWatchingPalette = watchColorPalette(() => {
    onChange(getStoredColorPalette(), getStoredAppearanceMode());
  });
  return () => {
    window.removeEventListener(themeChangeEvent, localHandler);
    stopWatchingPalette();
  };
}
