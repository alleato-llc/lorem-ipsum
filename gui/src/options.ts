// Pure form logic: per-mode count configuration and turning raw form
// values into backend options. No DOM access — fully unit-testable.

import type { GeneratorOptions, Mode } from "./types";

export interface ModeConfig {
  label: string;
  min: number;
  max: number;
  initial: number;
}

/** Per-mode count slider bounds and label. */
export const MODE_CONFIG: Record<Mode, ModeConfig> = {
  paragraphs: { label: "Paragraphs", min: 1, max: 12, initial: 3 },
  sentences: { label: "Sentences", min: 1, max: 30, initial: 5 },
  words: { label: "Words", min: 5, max: 200, initial: 50 },
};

/** Clamp a count into the mode's slider bounds. */
export function clampCount(value: number, mode: Mode): number {
  const { min, max } = MODE_CONFIG[mode];
  return Math.min(max, Math.max(min, value));
}

/** Raw form field values, as the DOM holds them. */
export interface RawFormValues {
  theme: string;
  mode: Mode;
  count: string;
  minSentences: string;
  maxSentences: string;
  minWords: string;
  maxWords: string;
  seed: string;
  startWithLorem: boolean;
}

/** Digits-only seed text parses to a number; anything else means random. */
export function parseSeed(raw: string): number | null {
  const trimmed = raw.trim();
  return /^\d+$/.test(trimmed) ? Number(trimmed) : null;
}

/** Build backend options from raw form values, clamping min/max ordering. */
export function buildOptions(raw: RawFormValues): GeneratorOptions {
  const minSentences = Number(raw.minSentences);
  const minWords = Number(raw.minWords);
  return {
    theme: raw.theme,
    mode: raw.mode,
    count: Number(raw.count),
    min_sentences: minSentences,
    max_sentences: Math.max(Number(raw.maxSentences), minSentences),
    min_words: minWords,
    max_words: Math.max(Number(raw.maxWords), minWords),
    seed: parseSeed(raw.seed),
    start_with_lorem: raw.startWithLorem,
  };
}
