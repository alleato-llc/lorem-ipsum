// Unit tests: pure form logic, no DOM.

import { describe, expect, it } from "vitest";

import { buildOptions, clampCount, parseSeed, MODE_CONFIG, RawFormValues } from "../../src/options";

const raw = (overrides: Partial<RawFormValues> = {}): RawFormValues => ({
  theme: "classic",
  mode: "paragraphs",
  count: "3",
  minSentences: "3",
  maxSentences: "6",
  minWords: "8",
  maxWords: "16",
  seed: "",
  startWithLorem: true,
  ...overrides,
});

describe("parseSeed", () => {
  it("parses digit strings", () => {
    expect(parseSeed("42")).toBe(42);
    expect(parseSeed("  7 ")).toBe(7);
  });

  it("treats anything else as random", () => {
    expect(parseSeed("")).toBeNull();
    expect(parseSeed("abc")).toBeNull();
    expect(parseSeed("4x2")).toBeNull();
    expect(parseSeed("-5")).toBeNull();
  });
});

describe("clampCount", () => {
  it("clamps into the mode's bounds", () => {
    expect(clampCount(0, "paragraphs")).toBe(MODE_CONFIG.paragraphs.min);
    expect(clampCount(999, "paragraphs")).toBe(MODE_CONFIG.paragraphs.max);
    expect(clampCount(999, "words")).toBe(MODE_CONFIG.words.max);
    expect(clampCount(50, "words")).toBe(50);
  });
});

describe("buildOptions", () => {
  it("maps raw values onto backend field names", () => {
    const opts = buildOptions(raw({ theme: "pirate", mode: "sentences", count: "5" }));
    expect(opts.theme).toBe("pirate");
    expect(opts.mode).toBe("sentences");
    expect(opts.count).toBe(5);
    expect(opts.min_sentences).toBe(3);
    expect(opts.start_with_lorem).toBe(true);
  });

  it("clamps max below min up to min", () => {
    const opts = buildOptions(raw({ minSentences: "9", maxSentences: "2", minWords: "30", maxWords: "10" }));
    expect(opts.max_sentences).toBe(9);
    expect(opts.max_words).toBe(30);
  });

  it("passes seed through parseSeed", () => {
    expect(buildOptions(raw({ seed: "123" })).seed).toBe(123);
    expect(buildOptions(raw({ seed: "random please" })).seed).toBeNull();
  });
});
