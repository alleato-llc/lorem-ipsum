//! A theme's language model: the composition of an interned vocabulary and
//! the Markov chain built over it. This is the only place corpus text meets
//! the chain.

use crate::chain::Chain;
use crate::interner::Interner;
use crate::text::{normalize_token, split_sentences};

pub(crate) struct Model {
    words: Interner,
    chain: Chain,
}

impl Model {
    pub(crate) fn from_corpus(corpus: &str) -> Self {
        let mut words = Interner::new();
        let sentences: Vec<Vec<u32>> = split_sentences(corpus)
            .map(|sentence| {
                sentence
                    .split_whitespace()
                    .filter_map(normalize_token)
                    .map(|w| words.intern(&w))
                    .collect()
            })
            .collect();
        Model {
            chain: Chain::from_sentences(&sentences),
            words,
        }
    }

    pub(crate) fn word(&self, id: u32) -> &str {
        self.words.resolve(id)
    }

    pub(crate) fn chain(&self) -> &Chain {
        &self.chain
    }

    /// A sentence-start pair by its words (e.g. "lorem", "ipsum").
    pub(crate) fn find_start(&self, first: &str, second: &str) -> Option<(u32, u32)> {
        let pair = (self.words.lookup(first)?, self.words.lookup(second)?);
        self.chain.start_pair(pair)
    }

    /// Resolve a walked id sequence to its words.
    pub(crate) fn resolve_all<'a>(&'a self, ids: &[u32]) -> Vec<&'a str> {
        ids.iter().map(|&id| self.words.resolve(id)).collect()
    }
}

#[cfg(test)]
mod tests;
