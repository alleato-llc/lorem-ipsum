// Component tests: the form module against the real index.html markup.

import { beforeEach, describe, expect, it, vi } from "vitest";

import { currentOptions, initForm } from "../../src/form";
import type { GeneratorOptions, ThemeInfo } from "../../src/types";
import { mountApp } from "../helpers/fixture";

const THEMES: ThemeInfo[] = [
  { id: "classic", name: "Classic Latin", description: "Traditional pseudo-Latin" },
  { id: "pirate", name: "Pirate", description: "High-seas adventure speak" },
];

const SAVED: GeneratorOptions = {
  theme: "pirate",
  mode: "sentences",
  count: 7,
  min_sentences: 2,
  max_sentences: 4,
  min_words: 5,
  max_words: 12,
  seed: null,
  start_with_lorem: false,
};

const byId = <T extends HTMLElement>(id: string): T =>
  document.getElementById(id) as T;

function change(id: string, value: string): void {
  const el = byId<HTMLInputElement | HTMLSelectElement>(id);
  el.value = value;
  el.dispatchEvent(new Event("change", { bubbles: true }));
}

describe("initForm", () => {
  beforeEach(() => {
    mountApp();
  });

  it("populates the theme select", () => {
    initForm(THEMES, SAVED, () => {});
    const options = byId<HTMLSelectElement>("theme").querySelectorAll("option");
    expect(options).toHaveLength(2);
    expect(options[1].textContent).toBe("Pirate");
  });

  it("applies saved defaults to every field", () => {
    initForm(THEMES, SAVED, () => {});
    expect(byId<HTMLSelectElement>("theme").value).toBe("pirate");
    expect(byId<HTMLSelectElement>("mode").value).toBe("sentences");
    expect(byId<HTMLInputElement>("count").value).toBe("7");
    expect(byId<HTMLInputElement>("min-sentences").value).toBe("2");
    expect(byId<HTMLInputElement>("max-words").value).toBe("12");
    expect(byId<HTMLInputElement>("start-with-lorem").checked).toBe(false);
  });

  it("shows the saved theme's description and disables the opener off-classic", () => {
    initForm(THEMES, SAVED, () => {});
    expect(byId("theme-description").textContent).toContain("High-seas");
    expect(byId<HTMLInputElement>("start-with-lorem").disabled).toBe(true);
    change("theme", "classic");
    expect(byId<HTMLInputElement>("start-with-lorem").disabled).toBe(false);
  });

  it("hides range fields that don't apply to the mode", () => {
    initForm(THEMES, SAVED, () => {});
    // sentences mode: sentence range hidden, word range shown
    expect(byId("sentence-range").hidden).toBe(true);
    expect(byId("word-range").hidden).toBe(false);
    change("mode", "words");
    expect(byId("word-range").hidden).toBe(true);
    change("mode", "paragraphs");
    expect(byId("sentence-range").hidden).toBe(false);
    expect(byId("word-range").hidden).toBe(false);
  });

  it("retunes the count slider per mode", () => {
    initForm(THEMES, SAVED, () => {});
    change("mode", "words");
    const count = byId<HTMLInputElement>("count");
    expect(count.max).toBe("200");
    expect(count.value).toBe("50");
    expect(byId("count-label").textContent).toBe("Words");
    expect(byId<HTMLOutputElement>("count-out").value).toBe("50");
  });

  it("clamps an out-of-bounds saved count into the slider range", () => {
    initForm(THEMES, { ...SAVED, mode: "paragraphs", count: 500 }, () => {});
    expect(byId<HTMLInputElement>("count").value).toBe("12");
  });

  it("submit calls the generate callback without reloading", () => {
    const onGenerate = vi.fn();
    initForm(THEMES, SAVED, onGenerate);
    byId<HTMLFormElement>("options-form").dispatchEvent(
      new Event("submit", { bubbles: true, cancelable: true }),
    );
    expect(onGenerate).toHaveBeenCalledOnce();
  });

  it("currentOptions reflects the live form state", () => {
    initForm(THEMES, SAVED, () => {});
    byId<HTMLInputElement>("seed").value = "42";
    const opts = currentOptions();
    expect(opts.theme).toBe("pirate");
    expect(opts.mode).toBe("sentences");
    expect(opts.count).toBe(7);
    expect(opts.seed).toBe(42);
  });
});
