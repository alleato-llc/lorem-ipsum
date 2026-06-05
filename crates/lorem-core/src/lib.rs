//! Markov-chain lorem ipsum generation with multiple themes.
//!
//! Shared by the CLI, TUI, and Tauri GUI front ends.

mod markov;
mod settings;
mod themes;

pub use markov::Chain;
pub use settings::{load_settings, save_settings, settings_path};
pub use themes::Theme;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

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

pub fn list_themes() -> Vec<ThemeInfo> {
    Theme::ALL
        .iter()
        .map(|t| ThemeInfo {
            id: t.id().to_string(),
            name: t.display_name().to_string(),
            description: t.description().to_string(),
        })
        .collect()
}

pub fn generate(opts: &GeneratorOptions) -> GeneratedText {
    // Keep auto-drawn seeds within f64-safe integer range so they survive a
    // round trip through JSON.
    let seed = opts.seed.unwrap_or_else(|| rand::rng().random::<u32>() as u64);
    let mut rng = StdRng::seed_from_u64(seed);
    let chain = Chain::from_corpus(opts.theme.corpus());

    let count = opts.count.max(1);
    let classic_opener = opts.start_with_lorem && opts.theme == Theme::Classic;
    let max_sentences = opts.max_sentences.max(opts.min_sentences).max(1);
    let min_sentences = opts.min_sentences.max(1);

    let (items, sentence_count) = match opts.mode {
        Mode::Words => {
            let start = classic_opener
                .then(|| chain.find_start("lorem", "ipsum"))
                .flatten();
            (vec![chain.words(&mut rng, count, start)], 0)
        }
        Mode::Sentences => {
            let mut items = Vec::with_capacity(count);
            for i in 0..count {
                if i == 0 && classic_opener {
                    items.push(LOREM_OPENER.to_string());
                } else {
                    items.push(chain.sentence(&mut rng, opts.min_words, opts.max_words));
                }
            }
            (items, count)
        }
        Mode::Paragraphs => {
            let mut items = Vec::with_capacity(count);
            let mut total = 0usize;
            for p in 0..count {
                let n = rng.random_range(min_sentences..=max_sentences);
                let mut sentences = Vec::with_capacity(n);
                for s in 0..n {
                    if p == 0 && s == 0 && classic_opener {
                        sentences.push(LOREM_OPENER.to_string());
                    } else {
                        sentences.push(chain.sentence(&mut rng, opts.min_words, opts.max_words));
                    }
                }
                total += sentences.len();
                items.push(sentences.join(" "));
            }
            (items, total)
        }
    };

    let word_count = items.iter().map(|p| p.split_whitespace().count()).sum();

    GeneratedText {
        theme: opts.theme.id().to_string(),
        theme_name: opts.theme.display_name().to_string(),
        mode: opts.mode,
        items,
        word_count,
        sentence_count,
        seed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_is_deterministic() {
        let opts = GeneratorOptions {
            seed: Some(42),
            ..Default::default()
        };
        let a = generate(&opts);
        let b = generate(&opts);
        assert_eq!(a.items, b.items);
        assert_eq!(a.seed, 42);
    }

    #[test]
    fn all_themes_produce_text() {
        for theme in Theme::ALL {
            let opts = GeneratorOptions {
                theme,
                count: 2,
                seed: Some(7),
                ..Default::default()
            };
            let out = generate(&opts);
            assert_eq!(out.items.len(), 2);
            assert!(out.word_count > 0, "theme {theme} produced no words");
        }
    }

    #[test]
    fn respects_paragraph_and_sentence_bounds() {
        let opts = GeneratorOptions {
            theme: Theme::Tech,
            count: 4,
            min_sentences: 2,
            max_sentences: 2,
            min_words: 5,
            max_words: 10,
            seed: Some(123),
            start_with_lorem: false,
            ..Default::default()
        };
        let out = generate(&opts);
        assert_eq!(out.items.len(), 4);
        assert_eq!(out.sentence_count, 8);
        for p in &out.items {
            assert_eq!(p.matches('.').count(), 2);
        }
    }

    #[test]
    fn classic_starts_with_lorem_when_asked() {
        let opts = GeneratorOptions {
            seed: Some(1),
            ..Default::default()
        };
        let out = generate(&opts);
        assert!(out.items[0].starts_with("Lorem ipsum dolor sit amet"));
    }

    #[test]
    fn words_mode_yields_exact_count() {
        for theme in Theme::ALL {
            let opts = GeneratorOptions {
                theme,
                mode: Mode::Words,
                count: 37,
                seed: Some(5),
                ..Default::default()
            };
            let out = generate(&opts);
            assert_eq!(out.items.len(), 1);
            assert_eq!(out.word_count, 37);
            assert_eq!(out.sentence_count, 0);
            // raw word run: no terminal punctuation
            assert!(!out.items[0].ends_with('.'));
        }
    }

    #[test]
    fn words_mode_classic_starts_with_lorem_ipsum() {
        let opts = GeneratorOptions {
            mode: Mode::Words,
            count: 10,
            seed: Some(9),
            ..Default::default()
        };
        let out = generate(&opts);
        assert!(out.items[0].starts_with("lorem ipsum"));
    }

    #[test]
    fn sentences_mode_yields_one_item_per_sentence() {
        let opts = GeneratorOptions {
            theme: Theme::Cosmic,
            mode: Mode::Sentences,
            count: 5,
            seed: Some(11),
            ..Default::default()
        };
        let out = generate(&opts);
        assert_eq!(out.items.len(), 5);
        assert_eq!(out.sentence_count, 5);
        for s in &out.items {
            assert!(s.ends_with('.'));
        }
    }
}
