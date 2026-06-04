import { invoke } from "@tauri-apps/api/core";

interface ThemeInfo {
  id: string;
  name: string;
  description: string;
}

interface GeneratedText {
  theme: string;
  theme_name: string;
  paragraphs: string[];
  word_count: number;
  sentence_count: number;
  seed: number;
}

interface GeneratorOptions {
  theme: string;
  paragraphs: number;
  min_sentences: number;
  max_sentences: number;
  min_words: number;
  max_words: number;
  seed: number | null;
  start_with_lorem: boolean;
}

const $ = <T extends HTMLElement>(id: string): T =>
  document.getElementById(id) as T;

const themeSelect = $<HTMLSelectElement>("theme");
const themeDescription = $<HTMLElement>("theme-description");
const paragraphsInput = $<HTMLInputElement>("paragraphs");
const paragraphsOut = $<HTMLOutputElement>("paragraphs-out");
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

function currentOptions(): GeneratorOptions {
  const seed = seedInput.value.trim();
  return {
    theme: themeSelect.value,
    paragraphs: Number(paragraphsInput.value),
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
  output.replaceChildren(
    ...result.paragraphs.map((text) => {
      const p = document.createElement("p");
      p.textContent = text;
      return p;
    }),
  );
  stats.textContent =
    `${result.theme_name} · ${result.paragraphs.length} paragraphs · ` +
    `${result.word_count} words · ${result.sentence_count} sentences · seed ${result.seed}`;
  copyButton.disabled = false;
}

async function generate(): Promise<void> {
  const result = await invoke<GeneratedText>("generate", {
    options: currentOptions(),
  });
  render(result);
}

async function init(): Promise<void> {
  themes = await invoke<ThemeInfo[]>("themes");
  themeSelect.replaceChildren(
    ...themes.map((t) => {
      const option = document.createElement("option");
      option.value = t.id;
      option.textContent = t.name;
      return option;
    }),
  );
  updateThemeDescription();
  await generate();
}

function updateThemeDescription(): void {
  const theme = themes.find((t) => t.id === themeSelect.value);
  themeDescription.textContent = theme?.description ?? "";
  // "Start with Lorem ipsum…" only applies to the classic theme.
  startWithLorem.disabled = themeSelect.value !== "classic";
}

themeSelect.addEventListener("change", updateThemeDescription);

paragraphsInput.addEventListener("input", () => {
  paragraphsOut.value = paragraphsInput.value;
});

form.addEventListener("submit", (e) => {
  e.preventDefault();
  void generate();
});

copyButton.addEventListener("click", async () => {
  if (!lastResult) return;
  await navigator.clipboard.writeText(lastResult.paragraphs.join("\n\n"));
  copyButton.textContent = "Copied!";
  setTimeout(() => (copyButton.textContent = "Copy text"), 1200);
});

void init();
