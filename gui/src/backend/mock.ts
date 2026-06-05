// Deterministic in-browser backend: lets the frontend run, demo, and be
// end-to-end tested in a plain browser where Tauri isn't available. Same
// seed → same output, mirroring the real core's contract.

import type { Backend } from "../backend";
import type { GeneratedText, GeneratorOptions, Mode, ThemeInfo } from "../types";

const THEMES: ThemeInfo[] = [
  { id: "classic", name: "Classic Latin", description: "Traditional lorem ipsum pseudo-Latin" },
  { id: "tech", name: "Tech Startup", description: "Startup and developer jargon" },
  { id: "pirate", name: "Pirate", description: "High-seas adventure speak" },
  { id: "corporate", name: "Corporate Buzzword", description: "Boardroom buzzword bingo" },
  { id: "cosmic", name: "Cosmic", description: "Spacefaring nebula prose" },
];

const WORDS: Record<string, string[]> = {
  classic: ["lorem", "ipsum", "dolor", "sit", "amet", "consectetur", "adipiscing", "elit", "tempor", "magna"],
  tech: ["scalable", "platform", "deploy", "pipeline", "cloud", "sprint", "telemetry", "latency", "cluster", "shipped"],
  pirate: ["crew", "galleon", "treasure", "horizon", "anchor", "lagoon", "doubloons", "spyglass", "shanty", "cove"],
  corporate: ["synergy", "stakeholder", "deliverable", "roadmap", "alignment", "pipeline", "quarter", "initiative", "leverage", "offsite"],
  cosmic: ["nebula", "starlight", "orbit", "comet", "void", "pulsar", "aurora", "galaxy", "telescope", "dawn"],
};

const DEFAULTS: GeneratorOptions = {
  theme: "classic",
  mode: "paragraphs",
  count: 3,
  min_sentences: 3,
  max_sentences: 6,
  min_words: 8,
  max_words: 16,
  seed: null,
  start_with_lorem: true,
};

/** Tiny deterministic PRNG (mulberry32). */
function rng(seed: number): () => number {
  let state = seed | 0;
  return () => {
    state = (state + 0x6d2b79f5) | 0;
    let t = Math.imul(state ^ (state >>> 15), 1 | state);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

function intBetween(next: () => number, min: number, max: number): number {
  return min + Math.floor(next() * (max - min + 1));
}

function words(next: () => number, vocabulary: string[], count: number): string[] {
  return Array.from({ length: count }, () => vocabulary[Math.floor(next() * vocabulary.length)]);
}

function sentence(next: () => number, vocabulary: string[], minWords: number, maxWords: number): string {
  const body = words(next, vocabulary, intBetween(next, minWords, Math.max(minWords, maxWords)));
  const text = body.join(" ");
  return text.charAt(0).toUpperCase() + text.slice(1) + ".";
}

function generateItems(options: GeneratorOptions, next: () => number): { items: string[]; sentences: number } {
  const vocabulary = WORDS[options.theme] ?? WORDS.classic;
  const count = Math.max(1, options.count);
  const opener = options.start_with_lorem && options.theme === "classic";

  const mode: Mode = options.mode;
  if (mode === "words") {
    const run = words(next, vocabulary, count);
    if (opener) {
      run.splice(0, Math.min(2, count), ...["lorem", "ipsum"].slice(0, Math.min(2, count)));
    }
    return { items: [run.join(" ")], sentences: 0 };
  }

  const makeSentence = (isFirst: boolean): string =>
    isFirst && opener
      ? "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
      : sentence(next, vocabulary, options.min_words, Math.max(options.min_words, options.max_words));

  if (mode === "sentences") {
    return {
      items: Array.from({ length: count }, (_, i) => makeSentence(i === 0)),
      sentences: count,
    };
  }

  let sentences = 0;
  const items = Array.from({ length: count }, (_, p) => {
    const min = Math.max(1, options.min_sentences);
    const n = intBetween(next, min, Math.max(min, options.max_sentences));
    sentences += n;
    return Array.from({ length: n }, (_, s) => makeSentence(p === 0 && s === 0)).join(" ");
  });
  return { items, sentences };
}

export const mockBackend: Backend = {
  themes: () => Promise.resolve(THEMES),

  settings: () => Promise.resolve({ ...DEFAULTS }),

  generate: (options: GeneratorOptions): Promise<GeneratedText> => {
    const seed = options.seed ?? Math.floor(Math.random() * 0xffffffff);
    const next = rng(seed);
    const { items, sentences } = generateItems(options, next);
    const theme = THEMES.find((t) => t.id === options.theme) ?? THEMES[0];
    return Promise.resolve({
      theme: theme.id,
      theme_name: theme.name,
      mode: options.mode,
      items,
      word_count: items.reduce((n, item) => n + item.split(/\s+/).length, 0),
      sentence_count: sentences,
      seed,
    });
  },
};
