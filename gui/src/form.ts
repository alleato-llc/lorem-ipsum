// The options form: reading it, populating it from saved settings, and
// keeping its fields consistent with the selected theme/mode.

import type { GeneratorOptions, Mode, ThemeInfo } from "./api";

/** Per-mode count slider bounds and label. */
const MODE_CONFIG: Record<Mode, { label: string; min: number; max: number; initial: number }> = {
  paragraphs: { label: "Paragraphs", min: 1, max: 12, initial: 3 },
  sentences: { label: "Sentences", min: 1, max: 30, initial: 5 },
  words: { label: "Words", min: 5, max: 200, initial: 50 },
};

const $ = <T extends HTMLElement>(id: string): T =>
  document.getElementById(id) as T;

const themeSelect = $<HTMLSelectElement>("theme");
const themeDescription = $<HTMLElement>("theme-description");
const modeSelect = $<HTMLSelectElement>("mode");
const countInput = $<HTMLInputElement>("count");
const countLabel = $<HTMLElement>("count-label");
const countOut = $<HTMLOutputElement>("count-out");
const sentenceRange = $<HTMLElement>("sentence-range");
const wordRange = $<HTMLElement>("word-range");
const minSentences = $<HTMLInputElement>("min-sentences");
const maxSentences = $<HTMLInputElement>("max-sentences");
const minWords = $<HTMLInputElement>("min-words");
const maxWords = $<HTMLInputElement>("max-words");
const seedInput = $<HTMLInputElement>("seed");
const startWithLorem = $<HTMLInputElement>("start-with-lorem");
const form = $<HTMLFormElement>("options-form");

let themes: ThemeInfo[] = [];

function currentMode(): Mode {
  return modeSelect.value as Mode;
}

/** The form's current state as backend options. */
export function currentOptions(): GeneratorOptions {
  const seed = seedInput.value.trim();
  return {
    theme: themeSelect.value,
    mode: currentMode(),
    count: Number(countInput.value),
    min_sentences: Number(minSentences.value),
    max_sentences: Math.max(Number(maxSentences.value), Number(minSentences.value)),
    min_words: Number(minWords.value),
    max_words: Math.max(Number(maxWords.value), Number(minWords.value)),
    seed: /^\d+$/.test(seed) ? Number(seed) : null,
    start_with_lorem: startWithLorem.checked,
  };
}

function updateThemeDescription(): void {
  const theme = themes.find((t) => t.id === themeSelect.value);
  themeDescription.textContent = theme?.description ?? "";
  // "Start with Lorem ipsum…" only applies to the classic theme.
  startWithLorem.disabled = themeSelect.value !== "classic";
}

/** Retune the count slider and hide irrelevant range fields for the mode. */
function updateModeFields(): void {
  const config = MODE_CONFIG[currentMode()];
  countLabel.textContent = config.label;
  countInput.min = String(config.min);
  countInput.max = String(config.max);
  countInput.value = String(config.initial);
  countOut.value = countInput.value;
  sentenceRange.hidden = currentMode() !== "paragraphs";
  wordRange.hidden = currentMode() === "words";
}

/** Populate the form from saved defaults. */
function applySettings(saved: GeneratorOptions): void {
  themeSelect.value = saved.theme;
  modeSelect.value = saved.mode;
  updateThemeDescription();
  updateModeFields(); // sets slider bounds for the mode, then override count:
  countInput.value = String(
    Math.min(Number(countInput.max), Math.max(Number(countInput.min), saved.count)),
  );
  countOut.value = countInput.value;
  minSentences.value = String(saved.min_sentences);
  maxSentences.value = String(saved.max_sentences);
  minWords.value = String(saved.min_words);
  maxWords.value = String(saved.max_words);
  startWithLorem.checked = saved.start_with_lorem;
}

/** Wire the form up: theme list, saved defaults, and the generate action. */
export function initForm(
  themeList: ThemeInfo[],
  saved: GeneratorOptions,
  onGenerate: () => void,
): void {
  themes = themeList;
  themeSelect.replaceChildren(
    ...themes.map((t) => {
      const option = document.createElement("option");
      option.value = t.id;
      option.textContent = t.name;
      return option;
    }),
  );
  applySettings(saved);

  themeSelect.addEventListener("change", updateThemeDescription);
  modeSelect.addEventListener("change", updateModeFields);
  countInput.addEventListener("input", () => {
    countOut.value = countInput.value;
  });
  form.addEventListener("submit", (e) => {
    e.preventDefault();
    onGenerate();
  });
}
