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
mod tests;
