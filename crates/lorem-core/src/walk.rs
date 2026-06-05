//! Sentence-walk policy: how a sentence-length id sequence is produced from
//! the chain — target length, natural ender stops, and where commas land.
//! Produces ids only; rendering to text is `compose`'s job.

use std::collections::HashSet;

use rand::rngs::StdRng;
use rand::Rng;

use crate::chain::Chain;

/// Overshoot allowed past `max_words` while hunting for an ender word.
const ENDER_SLACK: usize = 8;

/// Odds (percent) of a pause comma landing on an eligible word.
const PAUSE_COMMA_PCT: u32 = 20;

/// Minimum words between commas.
const COMMA_GAP: usize = 4;

pub(crate) struct SentenceWalk {
    pub(crate) ids: Vec<u32>,
    /// Positions after which a comma reads naturally.
    pub(crate) commas: HashSet<usize>,
}

/// Walk one sentence of roughly `min_words..=max_words` words. After the
/// target length the walk continues (up to `ENDER_SLACK` overshoot) until it
/// lands on a corpus sentence-ender, so sentences don't stop mid-phrase.
/// Dead ends splice in a fresh start; the seam becomes a comma.
pub(crate) fn sentence_walk(
    chain: &Chain,
    rng: &mut StdRng,
    min_words: usize,
    max_words: usize,
) -> SentenceWalk {
    let max_words = max_words.max(min_words);
    let target = rng.random_range(min_words..=max_words);
    let hard_max = max_words + ENDER_SLACK;

    let (a, b) = chain.random_start(rng);
    let mut ids = vec![a, b];
    let mut commas = HashSet::new();

    loop {
        let len = ids.len();
        if len >= hard_max || (len >= target && chain.is_ender(*ids.last().unwrap())) {
            break;
        }
        let pair = (ids[len - 2], ids[len - 1]);
        match chain.next_id(rng, pair) {
            Some(id) => ids.push(id),
            None => {
                commas.insert(len - 1);
                let (x, y) = chain.random_start(rng);
                ids.push(x);
                ids.push(y);
            }
        }
    }

    add_pause_commas(chain, rng, &ids, &mut commas);
    SentenceWalk { ids, commas }
}

/// Sprinkle occasional pause commas, only after corpus sentence-enders so
/// they never trail an article or preposition, and never near the ends.
fn add_pause_commas(chain: &Chain, rng: &mut StdRng, ids: &[u32], commas: &mut HashSet<usize>) {
    let mut last_comma = 0usize;
    for i in 3..ids.len().saturating_sub(3) {
        if commas.contains(&i) {
            last_comma = i;
        } else if i - last_comma >= COMMA_GAP
            && chain.is_ender(ids[i])
            && rng.random_range(0..100) < PAUSE_COMMA_PCT
        {
            commas.insert(i);
            last_comma = i;
        }
    }
}

#[cfg(test)]
mod tests;
