//! The TUI palette. Royal purple, mirroring the GUI's CSS variables
//! (gui/src/styles.css) — keep them in sync when retheming.

use ratatui::style::Color;

/// #7851a9 — fills, borders, selection background.
pub(crate) const ACCENT: Color = Color::Rgb(120, 81, 169);

/// #a78bdb — text accents on dark backgrounds.
pub(crate) const ACCENT_LIGHT: Color = Color::Rgb(167, 139, 219);

/// Muted purple row-selection background.
pub(crate) const ROW_HIGHLIGHT: Color = Color::Rgb(46, 38, 66);
