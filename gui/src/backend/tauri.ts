// The real backend: thin invoke wrappers around the Tauri commands.

import { invoke } from "@tauri-apps/api/core";

import type { Backend } from "../backend";
import type { GeneratedText, GeneratorOptions, ThemeInfo } from "../types";

export const tauriBackend: Backend = {
  themes: () => invoke<ThemeInfo[]>("themes"),
  settings: () => invoke<GeneratorOptions>("settings"),
  generate: (options: GeneratorOptions) =>
    invoke<GeneratedText>("generate", { options }),
};
