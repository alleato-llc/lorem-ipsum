//! Command-line surface: the clap definition and nothing else.

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(
    name = "lorem",
    about = "Markov-chain lorem ipsum generator (JSON output)",
    after_help = "Options left unset fall back to saved defaults \
                  (`lorem --save-defaults`), then built-ins.",
    version
)]
pub(crate) struct Args {
    /// Theme: classic, tech, pirate, corporate, cosmic
    #[arg(short, long)]
    pub(crate) theme: Option<String>,

    /// What to generate: paragraphs, sentences, or words
    #[arg(short, long)]
    pub(crate) mode: Option<String>,

    /// How many units of --mode to generate
    #[arg(short = 'n', long)]
    pub(crate) count: Option<usize>,

    /// Minimum sentences per paragraph (paragraphs mode)
    #[arg(long)]
    pub(crate) min_sentences: Option<usize>,

    /// Maximum sentences per paragraph (paragraphs mode)
    #[arg(long)]
    pub(crate) max_sentences: Option<usize>,

    /// Minimum words per sentence
    #[arg(long)]
    pub(crate) min_words: Option<usize>,

    /// Maximum words per sentence
    #[arg(long)]
    pub(crate) max_words: Option<usize>,

    /// Seed for reproducible output (random if omitted; reported in output)
    #[arg(short, long)]
    pub(crate) seed: Option<u64>,

    /// Begin with the canonical "Lorem ipsum…" (classic theme only)
    #[arg(long, conflicts_with = "no_lorem_start")]
    pub(crate) lorem_start: bool,

    /// Don't begin with the canonical "Lorem ipsum…"
    #[arg(long)]
    pub(crate) no_lorem_start: bool,

    /// Save the resolved options as defaults for future runs
    #[arg(long)]
    pub(crate) save_defaults: bool,

    /// Stream forever, ignoring --count, one word/sentence/paragraph at a
    /// time (per --mode)
    #[arg(long)]
    pub(crate) infinite: bool,

    /// Format for --infinite: pipe-friendly plain text (seed on stderr),
    /// or JSON Lines (meta line first, then one object per item)
    #[arg(long, value_enum, default_value_t = OutputFormat::Txt)]
    pub(crate) output_format: OutputFormat,

    /// Milliseconds between streamed items
    /// (default: 100 words / 400 sentences / 900 paragraphs)
    #[arg(long, value_name = "MS")]
    pub(crate) interval: Option<u64>,

    /// Emit compact JSON instead of pretty-printed
    #[arg(long)]
    pub(crate) compact: bool,

    /// List available themes as JSON and exit
    #[arg(long)]
    pub(crate) list_themes: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
pub(crate) enum OutputFormat {
    Txt,
    Json,
}

#[cfg(test)]
impl Default for Args {
    /// All-unset args for resolution tests.
    fn default() -> Self {
        Args {
            theme: None,
            mode: None,
            count: None,
            min_sentences: None,
            max_sentences: None,
            min_words: None,
            max_words: None,
            seed: None,
            lorem_start: false,
            no_lorem_start: false,
            save_defaults: false,
            infinite: false,
            output_format: OutputFormat::Txt,
            interval: None,
            compact: false,
            list_themes: false,
        }
    }
}
