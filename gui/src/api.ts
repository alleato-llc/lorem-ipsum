// The Tauri boundary: backend types and invoke wrappers, nothing else.

import { invoke } from "@tauri-apps/api/core";

export interface ThemeInfo {
  id: string;
  name: string;
  description: string;
}

export type Mode = "words" | "sentences" | "paragraphs";

export interface GeneratedText {
  theme: string;
  theme_name: string;
  mode: Mode;
  items: string[];
  word_count: number;
  sentence_count: number;
  seed: number;
}

export interface GeneratorOptions {
  theme: string;
  mode: Mode;
  count: number;
  min_sentences: number;
  max_sentences: number;
  min_words: number;
  max_words: number;
  seed: number | null;
  start_with_lorem: boolean;
}

export const fetchThemes = (): Promise<ThemeInfo[]> => invoke<ThemeInfo[]>("themes");

export const fetchSettings = (): Promise<GeneratorOptions> =>
  invoke<GeneratorOptions>("settings");

export const requestGeneration = (options: GeneratorOptions): Promise<GeneratedText> =>
  invoke<GeneratedText>("generate", { options });
