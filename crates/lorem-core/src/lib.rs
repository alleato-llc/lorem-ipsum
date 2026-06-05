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

/// Incremental generator: each `next_item` call yields one unit of `mode` —
/// a single word, sentence, or paragraph. Words mode is one continuous chain
/// walk, so consecutive words connect like running text. Used by `generate`
/// and by streaming front ends.
pub struct Generator {
    chain: Chain,
    rng: StdRng,
    opts: GeneratorOptions,
    seed: u64,
    /// Last two emitted word ids (words mode walk state).
    walk: Option<(u32, u32)>,
    /// Second word of a freshly spliced start pair, emitted next.
    queued: Option<u32>,
    opener_done: bool,
    sentences_generated: usize,
}

impl Generator {
    pub fn new(opts: &GeneratorOptions) -> Self {
        // Keep auto-drawn seeds within f64-safe integer range so they
        // survive a round trip through JSON.
        let seed = opts.seed.unwrap_or_else(|| rand::rng().random::<u32>() as u64);
        Generator {
            chain: Chain::from_corpus(opts.theme.corpus()),
            rng: StdRng::seed_from_u64(seed),
            opts: opts.clone(),
            seed,
            walk: None,
            queued: None,
            opener_done: false,
            sentences_generated: 0,
        }
    }

    /// The seed in use; feed it back in to reproduce the stream.
    pub fn seed(&self) -> u64 {
        self.seed
    }

    fn classic_opener(&self) -> bool {
        self.opts.start_with_lorem && self.opts.theme == Theme::Classic
    }

    fn next_word(&mut self) -> String {
        let id = match (self.walk, self.queued.take()) {
            (Some(_), Some(queued)) => queued,
            (Some(pair), None) => match self.chain.next_id(&mut self.rng, pair) {
                Some(id) => id,
                None => {
                    // Dead end: splice in a fresh start pair so the walk
                    // resumes on a real corpus transition.
                    let (x, y) = self.chain.random_start(&mut self.rng);
                    self.queued = Some(y);
                    x
                }
            },
            (None, _) => {
                // First word: classic + opener starts at "lorem ipsum".
                let start = self
                    .classic_opener()
                    .then(|| self.chain.find_start("lorem", "ipsum"))
                    .flatten()
                    .unwrap_or_else(|| self.chain.random_start(&mut self.rng));
                self.queued = Some(start.1);
                start.0
            }
        };
        self.walk = Some(match self.walk {
            Some((_, prev)) => (prev, id),
            None => (id, id), // placeholder until the queued word lands
        });
        self.chain.word_str(id).to_string()
    }

    fn next_sentence(&mut self) -> String {
        self.sentences_generated += 1;
        if !self.opener_done && self.classic_opener() {
            self.opener_done = true;
            return LOREM_OPENER.to_string();
        }
        self.opener_done = true;
        self.chain
            .sentence(&mut self.rng, self.opts.min_words, self.opts.max_words)
    }

    /// Yield the next word, sentence, or paragraph depending on `mode`.
    pub fn next_item(&mut self) -> String {
        match self.opts.mode {
            Mode::Words => self.next_word(),
            Mode::Sentences => self.next_sentence(),
            Mode::Paragraphs => {
                let min = self.opts.min_sentences.max(1);
                let max = self.opts.max_sentences.max(min);
                let n = self.rng.random_range(min..=max);
                let sentences: Vec<String> = (0..n).map(|_| self.next_sentence()).collect();
                sentences.join(" ")
            }
        }
    }
}

pub fn generate(opts: &GeneratorOptions) -> GeneratedText {
    let mut generator = Generator::new(opts);
    let count = opts.count.max(1);

    let items = match opts.mode {
        // A single flat run of `count` words.
        Mode::Words => {
            let words: Vec<String> = (0..count).map(|_| generator.next_item()).collect();
            vec![words.join(" ")]
        }
        Mode::Sentences | Mode::Paragraphs => {
            (0..count).map(|_| generator.next_item()).collect()
        }
    };

    let word_count = items.iter().map(|p| p.split_whitespace().count()).sum();

    GeneratedText {
        theme: opts.theme.id().to_string(),
        theme_name: opts.theme.display_name().to_string(),
        mode: opts.mode,
        items,
        word_count,
        sentence_count: generator.sentences_generated,
        seed: generator.seed,
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
    fn generator_streams_single_words() {
        let opts = GeneratorOptions {
            mode: Mode::Words,
            seed: Some(21),
            ..Default::default()
        };
        let mut generator = Generator::new(&opts);
        assert_eq!(generator.next_item(), "lorem");
        assert_eq!(generator.next_item(), "ipsum");
        for _ in 0..100 {
            let word = generator.next_item();
            assert!(!word.is_empty());
            assert!(
                !word.contains(char::is_whitespace),
                "stream item should be a single word: {word:?}"
            );
        }
    }

    #[test]
    fn generator_stream_matches_generate() {
        let opts = GeneratorOptions {
            theme: Theme::Pirate,
            mode: Mode::Sentences,
            count: 4,
            seed: Some(33),
            start_with_lorem: false,
            ..Default::default()
        };
        let batch = generate(&opts);
        let mut generator = Generator::new(&opts);
        let streamed: Vec<String> = (0..4).map(|_| generator.next_item()).collect();
        assert_eq!(batch.items, streamed);
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
