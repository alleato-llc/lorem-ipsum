//! Order-2 (bigram) Markov chain over words, built from a themed corpus.

use std::collections::{HashMap, HashSet};

use rand::rngs::StdRng;
use rand::Rng;

/// A word-level Markov chain. Words are interned to `u32` ids to keep the
/// transition table compact.
pub struct Chain {
    words: Vec<String>,
    transitions: HashMap<(u32, u32), Vec<u32>>,
    starts: Vec<(u32, u32)>,
    /// Words that ended a sentence in the corpus — natural stopping points.
    enders: HashSet<u32>,
}

fn intern(word: &str, words: &mut Vec<String>, index: &mut HashMap<String, u32>) -> u32 {
    if let Some(&id) = index.get(word) {
        return id;
    }
    let id = words.len() as u32;
    words.push(word.to_string());
    index.insert(word.to_string(), id);
    id
}

/// Lowercase a raw token and strip surrounding punctuation, keeping internal
/// apostrophes and hyphens (e.g. "ship's", "cross-functional").
fn normalize(raw: &str) -> Option<String> {
    let cleaned: String = raw
        .chars()
        .filter(|c| c.is_alphabetic() || *c == '\'' || *c == '-')
        .flat_map(|c| c.to_lowercase())
        .collect();
    let cleaned = cleaned.trim_matches(|c| c == '\'' || c == '-');
    if cleaned.is_empty() {
        None
    } else {
        Some(cleaned.to_string())
    }
}

impl Chain {
    pub fn from_corpus(corpus: &str) -> Self {
        let mut words = Vec::new();
        let mut index = HashMap::new();
        let mut transitions: HashMap<(u32, u32), Vec<u32>> = HashMap::new();
        let mut starts = Vec::new();
        let mut enders = HashSet::new();

        for sentence in corpus.split(|c| matches!(c, '.' | '!' | '?' | ';')) {
            let tokens: Vec<u32> = sentence
                .split_whitespace()
                .filter_map(normalize)
                .map(|w| intern(&w, &mut words, &mut index))
                .collect();
            if tokens.len() < 3 {
                continue;
            }
            starts.push((tokens[0], tokens[1]));
            enders.insert(*tokens.last().unwrap());
            for win in tokens.windows(3) {
                transitions
                    .entry((win[0], win[1]))
                    .or_default()
                    .push(win[2]);
            }
        }

        assert!(!starts.is_empty(), "corpus too small to build a chain");
        Chain {
            words,
            transitions,
            starts,
            enders,
        }
    }

    pub(crate) fn random_start(&self, rng: &mut StdRng) -> (u32, u32) {
        self.starts[rng.random_range(0..self.starts.len())]
    }

    /// One step of the walk; `None` means `pair` is a dead end and the
    /// caller should splice in a fresh start.
    pub(crate) fn next_id(&self, rng: &mut StdRng, pair: (u32, u32)) -> Option<u32> {
        self.transitions
            .get(&pair)
            .map(|nexts| nexts[rng.random_range(0..nexts.len())])
    }

    pub(crate) fn word_str(&self, id: u32) -> &str {
        &self.words[id as usize]
    }

    /// Locate a specific start pair by its words (used to begin classic
    /// word-mode output with "lorem ipsum").
    pub fn find_start(&self, first: &str, second: &str) -> Option<(u32, u32)> {
        self.starts
            .iter()
            .copied()
            .find(|&(a, b)| self.words[a as usize] == first && self.words[b as usize] == second)
    }

    /// Generate one sentence of roughly `min_words..=max_words` words.
    ///
    /// After reaching the target length the walk continues (up to a small
    /// overshoot) until it lands on a word that ended a sentence in the
    /// corpus, so sentences finish on a natural note instead of mid-phrase.
    pub fn sentence(&self, rng: &mut StdRng, min_words: usize, max_words: usize) -> String {
        let max_words = max_words.max(min_words);
        let target = rng.random_range(min_words..=max_words);
        let hard_max = max_words + 8;

        let (a, b) = self.random_start(rng);
        let mut ids = vec![a, b];
        // Positions after which a comma reads naturally (dead-end splices).
        let mut commas: HashSet<usize> = HashSet::new();

        loop {
            let len = ids.len();
            if len >= hard_max
                || (len >= target && self.enders.contains(ids.last().unwrap()))
            {
                break;
            }
            let key = (ids[len - 2], ids[len - 1]);
            match self.transitions.get(&key) {
                Some(nexts) => ids.push(nexts[rng.random_range(0..nexts.len())]),
                None => {
                    // Dead end: splice in a fresh start; the seam becomes a
                    // comma so the join reads as a clause break.
                    commas.insert(len - 1);
                    let (x, y) = self.random_start(rng);
                    ids.push(x);
                    ids.push(y);
                }
            }
        }

        // Occasional pause commas, only after corpus sentence-enders so they
        // never trail an article or preposition.
        let mut last_comma = 0usize;
        for i in 3..ids.len().saturating_sub(3) {
            if commas.contains(&i) {
                last_comma = i;
            } else if i - last_comma >= 4
                && self.enders.contains(&ids[i])
                && rng.random_range(0..100) < 20
            {
                commas.insert(i);
                last_comma = i;
            }
        }

        let mut out = String::new();
        for (i, &id) in ids.iter().enumerate() {
            let word = &self.words[id as usize];
            if i == 0 {
                let mut chars = word.chars();
                if let Some(first) = chars.next() {
                    out.extend(first.to_uppercase());
                    out.push_str(chars.as_str());
                }
            } else {
                out.push(' ');
                out.push_str(word);
            }
            if i + 1 < ids.len() && commas.contains(&i) {
                out.push(',');
            }
        }
        out.push('.');
        out
    }
}
