// The output pane: rendering generated text, the stats line, and the
// copy-to-clipboard control.

import type { GeneratedText } from "./api";

const output = document.getElementById("output") as HTMLElement;
const stats = document.getElementById("stats") as HTMLElement;
const copyButton = document.getElementById("copy") as HTMLButtonElement;

let lastResult: GeneratedText | null = null;

export function render(result: GeneratedText): void {
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

export function initOutput(): void {
  copyButton.addEventListener("click", async () => {
    if (!lastResult) return;
    await navigator.clipboard.writeText(lastResult.items.join("\n\n"));
    copyButton.textContent = "Copied!";
    setTimeout(() => (copyButton.textContent = "Copy text"), 1200);
  });
}
