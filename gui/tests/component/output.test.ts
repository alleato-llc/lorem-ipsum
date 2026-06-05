// Component tests: the output pane against the real index.html markup.

import { beforeEach, describe, expect, it, vi } from "vitest";

import { initOutput, render } from "../../src/output";
import type { GeneratedText } from "../../src/types";
import { mountApp } from "../helpers/fixture";

const RESULT: GeneratedText = {
  theme: "pirate",
  theme_name: "Pirate",
  mode: "paragraphs",
  items: ["The crew hoisted the flag.", "Gold doubloons spilled from the chest."],
  word_count: 11,
  sentence_count: 2,
  seed: 42,
};

const byId = <T extends HTMLElement>(id: string): T =>
  document.getElementById(id) as T;

describe("render", () => {
  beforeEach(() => {
    mountApp();
  });

  it("renders one paragraph element per item", () => {
    render(RESULT);
    const paragraphs = byId("output").querySelectorAll("p");
    expect(paragraphs).toHaveLength(2);
    expect(paragraphs[0].textContent).toBe("The crew hoisted the flag.");
  });

  it("shows stats including the seed", () => {
    render(RESULT);
    const stats = byId("stats").textContent ?? "";
    expect(stats).toContain("Pirate");
    expect(stats).toContain("11 words");
    expect(stats).toContain("2 sentences");
    expect(stats).toContain("seed 42");
  });

  it("omits the sentence count in words mode and flags the pane", () => {
    render({ ...RESULT, mode: "words", sentence_count: 0 });
    expect(byId("stats").textContent).not.toContain("sentences");
    expect(byId("output").classList.contains("words-mode")).toBe(true);
    render(RESULT);
    expect(byId("output").classList.contains("words-mode")).toBe(false);
  });

  it("enables the copy button once there is output", () => {
    expect(byId<HTMLButtonElement>("copy").disabled).toBe(true);
    render(RESULT);
    expect(byId<HTMLButtonElement>("copy").disabled).toBe(false);
  });
});

describe("copy", () => {
  beforeEach(() => {
    mountApp();
  });

  it("writes all items to the clipboard separated by blank lines", async () => {
    const writeText = vi.fn().mockResolvedValue(undefined);
    vi.stubGlobal("navigator", { clipboard: { writeText } });

    render(RESULT);
    initOutput();
    byId("copy").click();
    await vi.waitFor(() => expect(writeText).toHaveBeenCalled());

    expect(writeText).toHaveBeenCalledWith(
      "The crew hoisted the flag.\n\nGold doubloons spilled from the chest.",
    );
    expect(byId("copy").textContent).toBe("Copied!");
    vi.unstubAllGlobals();
  });
});
