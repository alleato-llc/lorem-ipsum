// The options form: DOM glue only. Option-building rules live in
// `options.ts`; this module reads/writes form elements and keeps fields
// consistent with the selected theme/mode.

import { byId } from "./dom";
import { buildOptions, clampCount, MODE_CONFIG } from "./options";
import type { GeneratorOptions, Mode, ThemeInfo } from "./types";

let themes: ThemeInfo[] = [];

function currentMode(): Mode {
  return byId<HTMLSelectElement>("mode").value as Mode;
}

/** The form's current state as backend options. */
export function currentOptions(): GeneratorOptions {
  return buildOptions({
    theme: byId<HTMLSelectElement>("theme").value,
    mode: currentMode(),
    count: byId<HTMLInputElement>("count").value,
    minSentences: byId<HTMLInputElement>("min-sentences").value,
    maxSentences: byId<HTMLInputElement>("max-sentences").value,
    minWords: byId<HTMLInputElement>("min-words").value,
    maxWords: byId<HTMLInputElement>("max-words").value,
    seed: byId<HTMLInputElement>("seed").value,
    startWithLorem: byId<HTMLInputElement>("start-with-lorem").checked,
  });
}

function updateThemeDescription(): void {
  const theme = themes.find((t) => t.id === byId<HTMLSelectElement>("theme").value);
  byId("theme-description").textContent = theme?.description ?? "";
  // "Start with Lorem ipsum…" only applies to the classic theme.
  byId<HTMLInputElement>("start-with-lorem").disabled =
    byId<HTMLSelectElement>("theme").value !== "classic";
}

function setCount(value: number): void {
  const count = byId<HTMLInputElement>("count");
  count.value = String(value);
  byId<HTMLOutputElement>("count-out").value = count.value;
}

/** Retune the count slider and hide irrelevant range fields for the mode. */
function updateModeFields(): void {
  const mode = currentMode();
  const config = MODE_CONFIG[mode];
  const count = byId<HTMLInputElement>("count");
  byId("count-label").textContent = config.label;
  count.min = String(config.min);
  count.max = String(config.max);
  setCount(config.initial);
  byId("sentence-range").hidden = mode !== "paragraphs";
  byId("word-range").hidden = mode === "words";
}

/** Populate the form from saved defaults. */
function applySettings(saved: GeneratorOptions): void {
  byId<HTMLSelectElement>("theme").value = saved.theme;
  byId<HTMLSelectElement>("mode").value = saved.mode;
  updateThemeDescription();
  updateModeFields(); // sets slider bounds for the mode, then override count:
  setCount(clampCount(saved.count, saved.mode));
  byId<HTMLInputElement>("min-sentences").value = String(saved.min_sentences);
  byId<HTMLInputElement>("max-sentences").value = String(saved.max_sentences);
  byId<HTMLInputElement>("min-words").value = String(saved.min_words);
  byId<HTMLInputElement>("max-words").value = String(saved.max_words);
  byId<HTMLInputElement>("start-with-lorem").checked = saved.start_with_lorem;
}

/** Wire the form up: theme list, saved defaults, and the generate action. */
export function initForm(
  themeList: ThemeInfo[],
  saved: GeneratorOptions,
  onGenerate: () => void,
): void {
  themes = themeList;
  byId<HTMLSelectElement>("theme").replaceChildren(
    ...themes.map((t) => {
      const option = document.createElement("option");
      option.value = t.id;
      option.textContent = t.name;
      return option;
    }),
  );
  applySettings(saved);

  byId("theme").addEventListener("change", updateThemeDescription);
  byId("mode").addEventListener("change", updateModeFields);
  byId("count").addEventListener("input", () => {
    byId<HTMLOutputElement>("count-out").value = byId<HTMLInputElement>("count").value;
  });
  byId<HTMLFormElement>("options-form").addEventListener("submit", (e) => {
    e.preventDefault();
    onGenerate();
  });
}
