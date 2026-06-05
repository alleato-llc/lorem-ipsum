//! Persisted generator defaults, shared by all front ends.
//!
//! Settings are a `GeneratorOptions` serialized as JSON in the platform
//! config dir. The seed is deliberately never persisted — a saved seed would
//! make every run produce identical output.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::GeneratorOptions;

/// Where settings live (e.g. `~/Library/Application Support/lorem-ipsum/`
/// on macOS, `~/.config/lorem-ipsum/` on Linux).
pub fn settings_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("lorem-ipsum").join("settings.json"))
}

/// Load saved defaults, falling back to built-in defaults if there's no
/// settings file (or it doesn't parse).
pub fn load_settings() -> GeneratorOptions {
    settings_path()
        .map(|p| load_settings_from(&p))
        .unwrap_or_default()
}

pub fn load_settings_from(path: &Path) -> GeneratorOptions {
    fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

/// Persist options as the new defaults. Returns the path written.
pub fn save_settings(opts: &GeneratorOptions) -> io::Result<PathBuf> {
    let path = settings_path()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no config directory"))?;
    save_settings_to(&path, opts)?;
    Ok(path)
}

pub fn save_settings_to(path: &Path, opts: &GeneratorOptions) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut to_save = opts.clone();
    to_save.seed = None; // never persist seeds
    let json = serde_json::to_string_pretty(&to_save).map_err(io::Error::other)?;
    fs::write(path, json)
}

#[cfg(test)]
mod tests {
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
}
