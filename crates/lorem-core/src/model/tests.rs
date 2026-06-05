use super::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

const CORPUS: &str = "Lorem ipsum dolor sit. Lorem ipsum amet est. Dolor sit amet.";

#[test]
fn builds_vocabulary_and_chain_from_corpus() {
    let model = Model::from_corpus(CORPUS);
    let mut rng = StdRng::seed_from_u64(1);
    let (a, b) = model.chain().random_start(&mut rng);
    let starts = (model.word(a).to_string(), model.word(b).to_string());
    assert!(
        starts == ("lorem".into(), "ipsum".into()) || starts == ("dolor".into(), "sit".into()),
        "unexpected start {starts:?}"
    );
}

#[test]
fn find_start_locates_known_pairs() {
    let model = Model::from_corpus(CORPUS);
    let pair = model.find_start("lorem", "ipsum").unwrap();
    assert_eq!(model.word(pair.0), "lorem");
    assert_eq!(model.word(pair.1), "ipsum");
}

#[test]
fn find_start_rejects_non_start_pairs_and_unknown_words() {
    let model = Model::from_corpus(CORPUS);
    // "ipsum dolor" occurs mid-sentence, never as a start.
    assert_eq!(model.find_start("ipsum", "dolor"), None);
    assert_eq!(model.find_start("nope", "ipsum"), None);
}

#[test]
fn resolve_all_maps_ids_back_to_words() {
    let model = Model::from_corpus(CORPUS);
    let pair = model.find_start("dolor", "sit").unwrap();
    assert_eq!(model.resolve_all(&[pair.0, pair.1]), ["dolor", "sit"]);
}
