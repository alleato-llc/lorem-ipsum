//! Option resolution: merge command-line arguments over saved defaults.
//! Precedence is flag → saved default → built-in.

use std::time::Duration;

use lorem_core::{GeneratorOptions, Mode, Theme};

use crate::args::Args;

pub(crate) fn options(args: &Args, defaults: GeneratorOptions) -> Result<GeneratorOptions, String> {
    let theme: Theme = match &args.theme {
        Some(s) => s.parse()?,
        None => defaults.theme,
    };
    let mode: Mode = match &args.mode {
        Some(s) => s.parse()?,
        None => defaults.mode,
    };
    let start_with_lorem = if args.lorem_start {
        true
    } else if args.no_lorem_start {
        false
    } else {
        defaults.start_with_lorem
    };

    Ok(GeneratorOptions {
        theme,
        mode,
        count: args.count.unwrap_or(defaults.count),
        min_sentences: args.min_sentences.unwrap_or(defaults.min_sentences),
        max_sentences: args.max_sentences.unwrap_or(defaults.max_sentences),
        min_words: args.min_words.unwrap_or(defaults.min_words),
        max_words: args.max_words.unwrap_or(defaults.max_words),
        seed: args.seed,
        start_with_lorem,
    })
}

/// Streaming delay: explicit flag, or a per-mode default.
pub(crate) fn interval(mode: Mode, flag: Option<u64>) -> Duration {
    Duration::from_millis(flag.unwrap_or(match mode {
        Mode::Words => 100,
        Mode::Sentences => 400,
        Mode::Paragraphs => 900,
    }))
}

#[cfg(test)]
mod tests;
