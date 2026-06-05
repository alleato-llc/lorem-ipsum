//! Integration tests: end-to-end generation behavior through the public API.

use lorem_core::{generate, Generator, GeneratorOptions, Mode, Theme};

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
