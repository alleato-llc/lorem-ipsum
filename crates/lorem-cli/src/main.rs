//! `lorem` — Markov-chain lorem ipsum generator with JSON output.

use clap::Parser;
use lorem_core::{generate, list_themes, load_settings, save_settings, GeneratorOptions, Mode, Theme};

#[derive(Parser)]
#[command(
    name = "lorem",
    about = "Markov-chain lorem ipsum generator (JSON output)",
    after_help = "Options left unset fall back to saved defaults \
                  (`lorem --save-defaults`), then built-ins.",
    version
)]
struct Args {
    /// Theme: classic, tech, pirate, corporate, cosmic
    #[arg(short, long)]
    theme: Option<String>,

    /// What to generate: paragraphs, sentences, or words
    #[arg(short, long)]
    mode: Option<String>,

    /// How many units of --mode to generate
    #[arg(short = 'n', long)]
    count: Option<usize>,

    /// Minimum sentences per paragraph (paragraphs mode)
    #[arg(long)]
    min_sentences: Option<usize>,

    /// Maximum sentences per paragraph (paragraphs mode)
    #[arg(long)]
    max_sentences: Option<usize>,

    /// Minimum words per sentence
    #[arg(long)]
    min_words: Option<usize>,

    /// Maximum words per sentence
    #[arg(long)]
    max_words: Option<usize>,

    /// Seed for reproducible output (random if omitted; reported in output)
    #[arg(short, long)]
    seed: Option<u64>,

    /// Begin with the canonical "Lorem ipsum…" (classic theme only)
    #[arg(long, conflicts_with = "no_lorem_start")]
    lorem_start: bool,

    /// Don't begin with the canonical "Lorem ipsum…"
    #[arg(long)]
    no_lorem_start: bool,

    /// Save the resolved options as defaults for future runs
    #[arg(long)]
    save_defaults: bool,

    /// Emit compact JSON instead of pretty-printed
    #[arg(long)]
    compact: bool,

    /// List available themes as JSON and exit
    #[arg(long)]
    list_themes: bool,
}

fn fail(msg: impl std::fmt::Display) -> ! {
    eprintln!("error: {msg}");
    std::process::exit(1);
}

fn main() {
    let args = Args::parse();

    if args.list_themes {
        println!("{}", serde_json::to_string_pretty(&list_themes()).unwrap());
        return;
    }

    let defaults = load_settings();

    let theme: Theme = match args.theme {
        Some(s) => s.parse().unwrap_or_else(|e| fail(e)),
        None => defaults.theme,
    };
    let mode: Mode = match args.mode {
        Some(s) => s.parse().unwrap_or_else(|e| fail(e)),
        None => defaults.mode,
    };
    let start_with_lorem = if args.lorem_start {
        true
    } else if args.no_lorem_start {
        false
    } else {
        defaults.start_with_lorem
    };

    let opts = GeneratorOptions {
        theme,
        mode,
        count: args.count.unwrap_or(defaults.count),
        min_sentences: args.min_sentences.unwrap_or(defaults.min_sentences),
        max_sentences: args.max_sentences.unwrap_or(defaults.max_sentences),
        min_words: args.min_words.unwrap_or(defaults.min_words),
        max_words: args.max_words.unwrap_or(defaults.max_words),
        seed: args.seed,
        start_with_lorem,
    };

    if args.save_defaults {
        match save_settings(&opts) {
            Ok(path) => eprintln!("saved defaults to {}", path.display()),
            Err(e) => fail(format!("could not save defaults: {e}")),
        }
    }

    let output = generate(&opts);
    let json = if args.compact {
        serde_json::to_string(&output).unwrap()
    } else {
        serde_json::to_string_pretty(&output).unwrap()
    };
    println!("{json}");
}
