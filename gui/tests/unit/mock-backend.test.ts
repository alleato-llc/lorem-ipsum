// Unit tests: the in-browser mock backend honors the core's contract.

import { describe, expect, it } from "vitest";

import { mockBackend } from "../../src/backend/mock";
import type { GeneratorOptions } from "../../src/types";

const opts = (overrides: Partial<GeneratorOptions> = {}): GeneratorOptions => ({
  theme: "classic",
  mode: "paragraphs",
  count: 3,
  min_sentences: 3,
  max_sentences: 6,
  min_words: 8,
  max_words: 16,
  seed: 42,
  start_with_lorem: true,
  ...overrides,
});

describe("mockBackend", () => {
  it("lists five themes", async () => {
    expect(await mockBackend.themes()).toHaveLength(5);
  });

  it("is deterministic for the same seed", async () => {
    const a = await mockBackend.generate(opts());
    const b = await mockBackend.generate(opts());
    expect(a.items).toEqual(b.items);
    expect(a.seed).toBe(42);
  });

  it("draws and reports a seed when none is given", async () => {
    const result = await mockBackend.generate(opts({ seed: null }));
    expect(Number.isInteger(result.seed)).toBe(true);
  });

  it("words mode yields one flat run of exactly count words", async () => {
    const result = await mockBackend.generate(opts({ mode: "words", count: 37 }));
    expect(result.items).toHaveLength(1);
    expect(result.items[0].split(" ")).toHaveLength(37);
    expect(result.sentence_count).toBe(0);
    expect(result.items[0].startsWith("lorem ipsum")).toBe(true);
  });

  it("sentences mode yields one item per sentence", async () => {
    const result = await mockBackend.generate(opts({ mode: "sentences", count: 4 }));
    expect(result.items).toHaveLength(4);
    expect(result.sentence_count).toBe(4);
    for (const item of result.items) {
      expect(item.endsWith(".")).toBe(true);
    }
  });

  it("classic paragraphs start with the canonical opener", async () => {
    const result = await mockBackend.generate(opts());
    expect(result.items[0].startsWith("Lorem ipsum dolor sit amet")).toBe(true);
  });
});
