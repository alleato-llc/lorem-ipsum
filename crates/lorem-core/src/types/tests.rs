use super::*;

#[test]
fn mode_parses_singular_and_plural() {
    assert_eq!("word".parse::<Mode>(), Ok(Mode::Words));
    assert_eq!("words".parse::<Mode>(), Ok(Mode::Words));
    assert_eq!("Sentence".parse::<Mode>(), Ok(Mode::Sentences));
    assert_eq!("PARAGRAPHS".parse::<Mode>(), Ok(Mode::Paragraphs));
}

#[test]
fn mode_rejects_unknown_values() {
    assert!("stanza".parse::<Mode>().is_err());
}

#[test]
fn mode_display_matches_id() {
    for mode in Mode::ALL {
        assert_eq!(mode.to_string(), mode.id());
    }
}

#[test]
fn options_default_to_classic_paragraphs() {
    let opts = GeneratorOptions::default();
    assert_eq!(opts.theme, Theme::Classic);
    assert_eq!(opts.mode, Mode::Paragraphs);
    assert_eq!(opts.seed, None);
    assert!(opts.start_with_lorem);
}

#[test]
fn options_deserialize_from_partial_json() {
    // The GUI sends partial options; everything else must default.
    let opts: GeneratorOptions = serde_json::from_str(r#"{"theme":"pirate"}"#).unwrap();
    assert_eq!(opts.theme, Theme::Pirate);
    assert_eq!(opts.count, GeneratorOptions::default().count);
}
