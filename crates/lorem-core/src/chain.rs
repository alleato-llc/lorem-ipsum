//! The order-2 Markov transition table — nothing but the statistics of
//! which word id follows a pair of word ids, where sentences start, and
//! which words ended a corpus sentence.

use std::collections::{HashMap, HashSet};

use rand::rngs::StdRng;
use rand::Rng;

pub(crate) struct Chain {
    transitions: HashMap<(u32, u32), Vec<u32>>,
    starts: Vec<(u32, u32)>,
    enders: HashSet<u32>,
}

impl Chain {
    /// Build from tokenized sentences. Sentences shorter than three tokens
    /// carry no order-2 information and are skipped.
    pub(crate) fn from_sentences(sentences: &[Vec<u32>]) -> Self {
        let mut transitions: HashMap<(u32, u32), Vec<u32>> = HashMap::new();
        let mut starts = Vec::new();
        let mut enders = HashSet::new();

        for tokens in sentences {
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
            transitions,
            starts,
            enders,
        }
    }

    /// A random sentence-start pair.
    pub(crate) fn random_start(&self, rng: &mut StdRng) -> (u32, u32) {
        self.starts[rng.random_range(0..self.starts.len())]
    }

    /// One step of the walk; `None` means `pair` is a dead end.
    pub(crate) fn next_id(&self, rng: &mut StdRng, pair: (u32, u32)) -> Option<u32> {
        self.transitions
            .get(&pair)
            .map(|nexts| nexts[rng.random_range(0..nexts.len())])
    }

    /// Did `id` end a sentence in the corpus?
    pub(crate) fn is_ender(&self, id: u32) -> bool {
        self.enders.contains(&id)
    }

    /// `pair` if it is a known sentence start.
    pub(crate) fn start_pair(&self, pair: (u32, u32)) -> Option<(u32, u32)> {
        self.starts.contains(&pair).then_some(pair)
    }
}

#[cfg(test)]
mod tests;
