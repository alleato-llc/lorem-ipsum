//! The serde-facing option and output types shared by every front end.
//! Data only — generation logic lives in `generator`.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::themes::Theme;

/// The canonical opening sentence, used when `start_with_lorem` is set on the
/// classic theme.
pub const LOREM_OPENER: &str =
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod \
     tempor incididunt ut labore et dolore magna aliqua.";

/// What `count` counts: bare words, full sentences, or paragraphs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Words,
    Sentences,
    Paragraphs,
}

impl Mode {
    pub const ALL: [Mode; 3] = [Mode::Paragraphs, Mode::Sentences, Mode::Words];

    pub fn id(&self) -> &'static str {
        match self {
            Mode::Words => "words",
            Mode::Sentences => "sentences",
            Mode::Paragraphs => "paragraphs",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Mode::Words => "Words",
            Mode::Sentences => "Sentences",
            Mode::Paragraphs => "Paragraphs",
        }
    }

    /// Singular label for one generated item (TUI table header etc.).
    pub fn item_label(&self) -> &'static str {
        match self {
            Mode::Words => "Words",
            Mode::Sentences => "Sentence",
            Mode::Paragraphs => "Paragraph",
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.id())
    }
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "word" | "words" => Ok(Mode::Words),
            "sentence" | "sentences" => Ok(Mode::Sentences),
            "paragraph" | "paragraphs" => Ok(Mode::Paragraphs),
            other => Err(format!(
                "unknown mode '{other}' (expected: words, sentences, paragraphs)"
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GeneratorOptions {
    pub theme: Theme,
    pub mode: Mode,
    /// How many units of `mode` to generate.
    pub count: usize,
    /// Sentences per paragraph (paragraphs mode only).
    pub min_sentences: usize,
    pub max_sentences: usize,
    /// Words per sentence (sentences and paragraphs modes).
    pub min_words: usize,
    pub max_words: usize,
    /// Reproducible output when set; a random seed is drawn (and reported)
    /// otherwise.
    pub seed: Option<u64>,
    /// Begin output with the canonical "Lorem ipsum…" (classic theme only).
    pub start_with_lorem: bool,
}

impl Default for GeneratorOptions {
    fn default() -> Self {
        GeneratorOptions {
            theme: Theme::Classic,
            mode: Mode::Paragraphs,
            count: 3,
            min_sentences: 3,
            max_sentences: 6,
            min_words: 8,
            max_words: 16,
            seed: None,
            start_with_lorem: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedText {
    pub theme: String,
    pub theme_name: String,
    pub mode: Mode,
    /// One entry per paragraph or sentence; a single entry in words mode.
    pub items: Vec<String>,
    pub word_count: usize,
    pub sentence_count: usize,
    /// The seed actually used; feed it back in to reproduce this output.
    pub seed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeInfo {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[cfg(test)]
mod tests;
