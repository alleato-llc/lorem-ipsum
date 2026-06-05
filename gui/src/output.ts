// The output pane: rendering generated text, the stats line, and the
// copy-to-clipboard control.

import { byId } from "./dom";
import type { GeneratedText } from "./types";

let lastResult: GeneratedText | null = null;

export function render(result: GeneratedText): void {
  lastResult = result;
  const output = byId("output");
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
  byId("stats").textContent =
    `${result.theme_name} · ${result.mode} · ` +
    `${result.word_count} words${sentences} · seed ${result.seed}`;
  byId<HTMLButtonElement>("copy").disabled = false;
}

export function initOutput(): void {
  byId("copy").addEventListener("click", async () => {
    if (!lastResult) return;
    const copyButton = byId<HTMLButtonElement>("copy");
    await navigator.clipboard.writeText(lastResult.items.join("\n\n"));
    copyButton.textContent = "Copied!";
    setTimeout(() => (copyButton.textContent = "Copy text"), 1200);
  });
}
