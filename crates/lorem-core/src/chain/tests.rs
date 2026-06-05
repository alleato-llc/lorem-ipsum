use super::*;
use rand::SeedableRng;

/// ids: 0 1 2 3 / 0 1 4 — gives (0,1) two continuations.
fn chain() -> Chain {
    Chain::from_sentences(&[vec![0, 1, 2, 3], vec![0, 1, 4]])
}

fn rng() -> StdRng {
    StdRng::seed_from_u64(1)
}

#[test]
fn next_id_follows_corpus_transitions() {
    let chain = chain();
    let mut rng = rng();
    for _ in 0..20 {
        let next = chain.next_id(&mut rng, (0, 1)).unwrap();
        assert!(next == 2 || next == 4, "unexpected transition to {next}");
    }
    assert_eq!(chain.next_id(&mut rng, (1, 2)), Some(3));
}

#[test]
fn dead_end_pair_yields_none() {
    let chain = chain();
    assert_eq!(chain.next_id(&mut rng(), (2, 3)), None);
    assert_eq!(chain.next_id(&mut rng(), (9, 9)), None);
}

#[test]
fn enders_are_sentence_final_words() {
    let chain = chain();
    assert!(chain.is_ender(3));
    assert!(chain.is_ender(4));
    assert!(!chain.is_ender(0));
    assert!(!chain.is_ender(1));
}

#[test]
fn random_start_returns_a_corpus_start() {
    let chain = chain();
    assert_eq!(chain.random_start(&mut rng()), (0, 1));
}

#[test]
fn start_pair_only_matches_known_starts() {
    let chain = chain();
    assert_eq!(chain.start_pair((0, 1)), Some((0, 1)));
    assert_eq!(chain.start_pair((1, 2)), None);
}

#[test]
fn short_sentences_are_skipped() {
    // Only the 3-token sentence contributes; (5,6) is not a start.
    let chain = Chain::from_sentences(&[vec![5, 6], vec![0, 1, 2]]);
    assert_eq!(chain.start_pair((5, 6)), None);
    assert_eq!(chain.start_pair((0, 1)), Some((0, 1)));
}
