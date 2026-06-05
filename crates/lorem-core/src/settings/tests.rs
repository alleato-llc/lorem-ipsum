use super::*;
use crate::{Mode, Theme};

#[test]
fn settings_round_trip_without_seed() {
    let dir = std::env::temp_dir().join("lorem-ipsum-test-settings");
    let path = dir.join("settings.json");
    let opts = GeneratorOptions {
        theme: Theme::Pirate,
        mode: Mode::Sentences,
        count: 9,
        start_with_lorem: false,
        seed: Some(42),
        ..Default::default()
    };
    save_settings_to(&path, &opts).unwrap();
    let loaded = load_settings_from(&path);
    assert_eq!(loaded.theme, Theme::Pirate);
    assert_eq!(loaded.mode, Mode::Sentences);
    assert_eq!(loaded.count, 9);
    assert!(!loaded.start_with_lorem);
    assert_eq!(loaded.seed, None, "seed must never be persisted");
    fs::remove_dir_all(&dir).ok();
}

#[test]
fn missing_settings_file_yields_defaults() {
    let loaded = load_settings_from(Path::new("/nonexistent/lorem/settings.json"));
    assert_eq!(loaded.count, GeneratorOptions::default().count);
}
