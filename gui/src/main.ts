import { invoke } from "@tauri-apps/api/core";

interface ThemeInfo {
  id: string;
  name: string;
  description: string;
}

type Mode = "words" | "sentences" | "paragraphs";

interface GeneratedText {
  theme: string;
  theme_name: string;
  mode: Mode;
  items: string[];
  word_count: number;
  sentence_count: number;
  seed: number;
}

interface GeneratorOptions {
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
const output = $<HTMLElement>("output");
const stats = $<HTMLElement>("stats");
const copyButton = $<HTMLButtonElement>("copy");

let themes: ThemeInfo[] = [];
let lastResult: GeneratedText | null = null;

function currentMode(): Mode {
  return modeSelect.value as Mode;
}

function currentOptions(): GeneratorOptions {
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

function render(result: GeneratedText): void {
  lastResult = result;
  output.classList.toggle("words-mode", result.mode === "words");
  output.replaceChildren(
    ...result.items.map((text) => {
      const p = document.createElement("p");
      p.textContent = text;
      return p;
    }),
  );
  const sentences =
    result.sentence_count > 0 ? ` · ${result.sentence_count} sentences` : "";
  stats.textContent =
    `${result.theme_name} · ${result.mode} · ` +
    `${result.word_count} words${sentences} · seed ${result.seed}`;
  copyButton.disabled = false;
}

async function generate(): Promise<void> {
  const result = await invoke<GeneratedText>("generate", {
    options: currentOptions(),
  });
  render(result);
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

async function init(): Promise<void> {
  const [themeList, saved] = await Promise.all([
    invoke<ThemeInfo[]>("themes"),
    invoke<GeneratorOptions>("settings"),
  ]);
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
  await generate();
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

themeSelect.addEventListener("change", updateThemeDescription);
modeSelect.addEventListener("change", updateModeFields);

countInput.addEventListener("input", () => {
  countOut.value = countInput.value;
});

form.addEventListener("submit", (e) => {
  e.preventDefault();
  void generate();
});

copyButton.addEventListener("click", async () => {
  if (!lastResult) return;
  await navigator.clipboard.writeText(lastResult.items.join("\n\n"));
  copyButton.textContent = "Copied!";
  setTimeout(() => (copyButton.textContent = "Copy text"), 1200);
});

void init();
