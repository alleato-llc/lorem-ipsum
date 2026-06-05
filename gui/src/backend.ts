// The backend seam: one interface, selected at runtime. Inside Tauri the
// real commands run; in a plain browser (vite dev, tests, Playwright) the
// deterministic mock stands in.

import type { GeneratedText, GeneratorOptions, ThemeInfo } from "./types";
import { mockBackend } from "./backend/mock";
import { tauriBackend } from "./backend/tauri";

export interface Backend {
  themes(): Promise<ThemeInfo[]>;
  settings(): Promise<GeneratorOptions>;
  generate(options: GeneratorOptions): Promise<GeneratedText>;
}

const isTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

export const backend: Backend = isTauri ? tauriBackend : mockBackend;
