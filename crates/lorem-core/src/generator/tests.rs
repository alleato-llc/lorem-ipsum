use super::*;

fn opts(mode: Mode, seed: u64) -> GeneratorOptions {
    GeneratorOptions {
        mode,
        seed: Some(seed),
        ..Default::default()
    }
}

#[test]
fn opener_fires_exactly_once_across_paragraphs() {
    let mut generator = Generator::new(&opts(Mode::Paragraphs, 2));
    let first = generator.next_item();
    let second = generator.next_item();
    assert!(first.starts_with("Lorem ipsum dolor sit amet"));
    assert!(!second.contains("consectetur adipiscing elit, sed do eiusmod"));
}

#[test]
fn word_walk_keeps_continuity_after_queued_start() {
    let mut generator = Generator::new(&opts(Mode::Words, 4));
    // First two words come from the forced "lorem ipsum" start pair.
    assert_eq!(generator.next_item(), "lorem");
    assert_eq!(generator.next_item(), "ipsum");
    // The third word must be a real corpus continuation of "lorem ipsum".
    assert_eq!(generator.next_item(), "dolor");
}

#[test]
fn sentence_counter_tracks_every_sentence() {
    let options = GeneratorOptions {
        mode: Mode::Paragraphs,
        min_sentences: 2,
        max_sentences: 2,
        seed: Some(5),
        ..Default::default()
    };
    let mut generator = Generator::new(&options);
    generator.next_item();
    generator.next_item();
    assert_eq!(generator.sentences_generated(), 4);
}
