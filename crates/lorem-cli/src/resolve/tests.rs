use super::*;

fn saved_defaults() -> GeneratorOptions {
    GeneratorOptions {
        theme: Theme::Pirate,
        mode: Mode::Sentences,
        count: 9,
        start_with_lorem: false,
        ..Default::default()
    }
}

#[test]
fn unset_args_fall_back_to_saved_defaults() {
    let opts = options(&Args::default(), saved_defaults()).unwrap();
    assert_eq!(opts.theme, Theme::Pirate);
    assert_eq!(opts.mode, Mode::Sentences);
    assert_eq!(opts.count, 9);
    assert!(!opts.start_with_lorem);
}

#[test]
fn flags_override_saved_defaults() {
    let args = Args {
        theme: Some("cosmic".into()),
        mode: Some("words".into()),
        count: Some(2),
        ..Default::default()
    };
    let opts = options(&args, saved_defaults()).unwrap();
    assert_eq!(opts.theme, Theme::Cosmic);
    assert_eq!(opts.mode, Mode::Words);
    assert_eq!(opts.count, 2);
}

#[test]
fn lorem_start_flag_overrides_a_saved_off() {
    let args = Args {
        lorem_start: true,
        ..Default::default()
    };
    let opts = options(&args, saved_defaults()).unwrap();
    assert!(opts.start_with_lorem);
}

#[test]
fn no_lorem_start_flag_overrides_a_saved_on() {
    let args = Args {
        no_lorem_start: true,
        ..Default::default()
    };
    let opts = options(&args, GeneratorOptions::default()).unwrap();
    assert!(!opts.start_with_lorem);
}

#[test]
fn invalid_theme_or_mode_is_an_error() {
    let bad_theme = Args {
        theme: Some("haiku".into()),
        ..Default::default()
    };
    assert!(options(&bad_theme, GeneratorOptions::default()).is_err());

    let bad_mode = Args {
        mode: Some("stanzas".into()),
        ..Default::default()
    };
    assert!(options(&bad_mode, GeneratorOptions::default()).is_err());
}

#[test]
fn seed_comes_only_from_the_flag_never_from_defaults() {
    // Saved defaults never carry a seed, but belt-and-braces: resolution
    // must take the seed from args alone.
    let opts = options(&Args::default(), saved_defaults()).unwrap();
    assert_eq!(opts.seed, None);

    let args = Args {
        seed: Some(7),
        ..Default::default()
    };
    assert_eq!(options(&args, saved_defaults()).unwrap().seed, Some(7));
}

#[test]
fn interval_uses_per_mode_defaults() {
    assert_eq!(interval(Mode::Words, None), Duration::from_millis(100));
    assert_eq!(interval(Mode::Sentences, None), Duration::from_millis(400));
    assert_eq!(interval(Mode::Paragraphs, None), Duration::from_millis(900));
    assert_eq!(interval(Mode::Words, Some(5)), Duration::from_millis(5));
}
