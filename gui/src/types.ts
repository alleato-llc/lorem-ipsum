// Backend data types, mirroring lorem-core's serde types.

export interface ThemeInfo {
  id: string;
  name: string;
  description: string;
}

export type Mode = "words" | "sentences" | "paragraphs";

export interface GeneratedText {
  theme: string;
  theme_name: string;
  mode: Mode;
  items: string[];
  word_count: number;
  sentence_count: number;
  seed: number;
}

export interface GeneratorOptions {
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
