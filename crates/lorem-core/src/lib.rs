//! Markov-chain lorem ipsum generation with multiple themes.
//!
//! Shared by the CLI, TUI, and Tauri GUI front ends. Module map (one
//! responsibility each):
//!
//! - [`text`] — pure tokenization and casing helpers
//! - [`interner`] — word ↔ `u32` id table
//! - [`chain`] — the order-2 Markov transition table
//! - [`model`] — corpus text → vocabulary + chain composition
//! - [`walk`] — sentence-walk policy (length, ender stops, comma seams)
//! - [`compose`] — word sequence → punctuated sentence string
//! - [`generator`] — orchestration: `Generator` (incremental) and `generate`
//! - [`types`] — serde option/output types
//! - [`themes`] — themed corpora
//! - [`settings`] — persisted defaults

mod chain;
mod compose;
mod generator;
mod interner;
mod model;
mod settings;
mod text;
mod themes;
mod types;
mod walk;

pub use generator::{generate, Generator};
pub use settings::{load_settings, save_settings, settings_path};
pub use themes::{list_themes, Theme};
pub use types::{GeneratedText, GeneratorOptions, Mode, ThemeInfo, LOREM_OPENER};
