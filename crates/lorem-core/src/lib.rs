//! Markov-chain lorem ipsum generation with multiple themes.
//!
//! Shared by the CLI, TUI, and Tauri GUI front ends.

mod markov;
mod themes;

pub use markov::Chain;
pub use themes::Theme;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};

/// The canonical opening sentence, used when `start_with_lorem` is set on the
/// classic theme.
pub const LOREM_OPENER: &str =
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod \
     tempor incididunt ut labore et dolore magna aliqua.";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GeneratorOptions {
    pub theme: Theme,
    pub paragraphs: usize,
    pub min_sentences: usize,
    pub max_sentences: usize,
    pub min_words: usize,
    pub max_words: usize,
    /// Reproducible output when set; a random seed is drawn (and reported)
    /// otherwise.
    pub seed: Option<u64>,
    /// Begin the first paragraph with the canonical "Lorem ipsum…" sentence
    /// (classic theme only).
    pub start_with_lorem: bool,
}

impl Default for GeneratorOptions {
    fn default() -> Self {
        GeneratorOptions {
            theme: Theme::Classic,
            paragraphs: 3,
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
    pub paragraphs: Vec<String>,
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

    let max_sentences = opts.max_sentences.max(opts.min_sentences);
    let mut paragraphs = Vec::with_capacity(opts.paragraphs);
    let mut sentence_count = 0usize;

    for p in 0..opts.paragraphs {
        let n = rng.random_range(opts.min_sentences.max(1)..=max_sentences.max(1));
        let mut sentences = Vec::with_capacity(n);
        for s in 0..n {
            if p == 0 && s == 0 && opts.start_with_lorem && opts.theme == Theme::Classic {
                sentences.push(LOREM_OPENER.to_string());
            } else {
                sentences.push(chain.sentence(&mut rng, opts.min_words, opts.max_words));
            }
        }
        sentence_count += sentences.len();
        paragraphs.push(sentences.join(" "));
    }

    let word_count = paragraphs
        .iter()
        .map(|p| p.split_whitespace().count())
        .sum();

    GeneratedText {
        theme: opts.theme.id().to_string(),
        theme_name: opts.theme.display_name().to_string(),
        paragraphs,
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
        assert_eq!(a.paragraphs, b.paragraphs);
        assert_eq!(a.seed, 42);
    }

    #[test]
    fn all_themes_produce_text() {
        for theme in Theme::ALL {
            let opts = GeneratorOptions {
                theme,
                paragraphs: 2,
                seed: Some(7),
                ..Default::default()
            };
            let out = generate(&opts);
            assert_eq!(out.paragraphs.len(), 2);
            assert!(out.word_count > 0, "theme {theme} produced no words");
        }
    }

    #[test]
    fn respects_paragraph_and_sentence_bounds() {
        let opts = GeneratorOptions {
            theme: Theme::Tech,
            paragraphs: 4,
            min_sentences: 2,
            max_sentences: 2,
            min_words: 5,
            max_words: 10,
            seed: Some(123),
            start_with_lorem: false,
        };
        let out = generate(&opts);
        assert_eq!(out.paragraphs.len(), 4);
        assert_eq!(out.sentence_count, 8);
        for p in &out.paragraphs {
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
        assert!(out.paragraphs[0].starts_with("Lorem ipsum dolor sit amet"));
    }
}
