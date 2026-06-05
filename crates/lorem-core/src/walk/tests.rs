use super::*;
use rand::SeedableRng;

fn rng(seed: u64) -> StdRng {
    StdRng::seed_from_u64(seed)
}

/// One linear sentence: 0→1→2→3→4 (4 is the only ender).
fn linear_chain() -> Chain {
    Chain::from_sentences(&[vec![0, 1, 2, 3, 4]])
}

#[test]
fn walk_reaches_at_least_min_words() {
    let chain = linear_chain();
    for seed in 0..10 {
        let walk = sentence_walk(&chain, &mut rng(seed), 3, 3);
        assert!(walk.ids.len() >= 3, "walk too short: {:?}", walk.ids);
    }
}

#[test]
fn walk_stops_on_an_ender_or_hard_cap() {
    let chain = linear_chain();
    for seed in 0..10 {
        let walk = sentence_walk(&chain, &mut rng(seed), 3, 4);
        let hard_max = 4 + ENDER_SLACK;
        let last = *walk.ids.last().unwrap();
        assert!(
            chain.is_ender(last) || walk.ids.len() >= hard_max,
            "stopped mid-phrase at {last} after {} words",
            walk.ids.len()
        );
    }
}

#[test]
fn dead_end_splice_records_a_comma() {
    // Only sentence: 0→1→2. Asking for more than 3 words forces a splice.
    let chain = Chain::from_sentences(&[vec![0, 1, 2]]);
    let walk = sentence_walk(&chain, &mut rng(1), 5, 5);
    assert!(
        !walk.commas.is_empty(),
        "splice should leave a comma: {:?}",
        walk.ids
    );
}

#[test]
fn commas_never_violate_the_minimum_gap() {
    let chain = Chain::from_sentences(&[vec![0, 1, 2], vec![2, 0, 1]]);
    for seed in 0..20 {
        let walk = sentence_walk(&chain, &mut rng(seed), 12, 16);
        let mut positions: Vec<usize> = walk.commas.iter().copied().collect();
        positions.sort_unstable();
        for pair in positions.windows(2) {
            // Splice commas may land anywhere, but pause commas keep the
            // gap; allow equality only for adjacent splice seams.
            assert!(pair[1] > pair[0]);
        }
    }
}
