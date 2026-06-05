//! Generation orchestration: drives the model, walk, and compose stages to
//! produce items (words / sentences / paragraphs) per the options.

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::compose::compose_sentence;
use crate::model::Model;
use crate::themes::Theme;
use crate::types::{GeneratedText, GeneratorOptions, Mode, LOREM_OPENER};
use crate::walk::sentence_walk;

/// Incremental generator: each `next_item` call yields one unit of `mode` —
/// a single word, sentence, or paragraph. Words mode is one continuous chain
/// walk, so consecutive words connect like running text. `generate` is built
/// on this, so batch and streamed output are identical for the same seed.
pub struct Generator {
    model: Model,
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
            model: Model::from_corpus(opts.theme.corpus()),
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
            (Some(pair), None) => match self.model.chain().next_id(&mut self.rng, pair) {
                Some(id) => id,
                None => {
                    // Dead end: splice in a fresh start pair so the walk
                    // resumes on a real corpus transition.
                    let (x, y) = self.model.chain().random_start(&mut self.rng);
                    self.queued = Some(y);
                    x
                }
            },
            (None, _) => {
                // First word: classic + opener starts at "lorem ipsum".
                let start = self
                    .classic_opener()
                    .then(|| self.model.find_start("lorem", "ipsum"))
                    .flatten()
                    .unwrap_or_else(|| self.model.chain().random_start(&mut self.rng));
                self.queued = Some(start.1);
                start.0
            }
        };
        self.walk = Some(match self.walk {
            Some((_, prev)) => (prev, id),
            None => (id, id), // placeholder until the queued word lands
        });
        self.model.word(id).to_string()
    }

    fn next_sentence(&mut self) -> String {
        self.sentences_generated += 1;
        if !self.opener_done && self.classic_opener() {
            self.opener_done = true;
            return LOREM_OPENER.to_string();
        }
        self.opener_done = true;
        let walk = sentence_walk(
            self.model.chain(),
            &mut self.rng,
            self.opts.min_words,
            self.opts.max_words,
        );
        compose_sentence(&self.model.resolve_all(&walk.ids), &walk.commas)
    }

    fn next_paragraph(&mut self) -> String {
        let min = self.opts.min_sentences.max(1);
        let max = self.opts.max_sentences.max(min);
        let n = self.rng.random_range(min..=max);
        let sentences: Vec<String> = (0..n).map(|_| self.next_sentence()).collect();
        sentences.join(" ")
    }

    /// Yield the next word, sentence, or paragraph depending on `mode`.
    pub fn next_item(&mut self) -> String {
        match self.opts.mode {
            Mode::Words => self.next_word(),
            Mode::Sentences => self.next_sentence(),
            Mode::Paragraphs => self.next_paragraph(),
        }
    }

    pub(crate) fn sentences_generated(&self) -> usize {
        self.sentences_generated
    }
}

/// Batch generation: `count` items collected into a `GeneratedText`.
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
        sentence_count: generator.sentences_generated(),
        seed: generator.seed(),
    }
}

#[cfg(test)]
mod tests;
