//! `lorem` — Markov-chain lorem ipsum generator with JSON output.

use clap::Parser;
use lorem_core::{generate, list_themes, GeneratorOptions, Theme};

#[derive(Parser)]
#[command(
    name = "lorem",
    about = "Markov-chain lorem ipsum generator (JSON output)",
    version
)]
struct Args {
    /// Theme: classic, tech, pirate, corporate, cosmic
    #[arg(short, long, default_value = "classic")]
    theme: String,

    /// Number of paragraphs
    #[arg(short, long, default_value_t = 3)]
    paragraphs: usize,

    /// Minimum sentences per paragraph
    #[arg(long, default_value_t = 3)]
    min_sentences: usize,

    /// Maximum sentences per paragraph
    #[arg(long, default_value_t = 6)]
    max_sentences: usize,

    /// Minimum words per sentence
    #[arg(long, default_value_t = 8)]
    min_words: usize,

    /// Maximum words per sentence
    #[arg(long, default_value_t = 16)]
    max_words: usize,

    /// Seed for reproducible output (random if omitted; reported in output)
    #[arg(short, long)]
    seed: Option<u64>,

    /// Don't force the first sentence to be the canonical "Lorem ipsum…"
    /// (classic theme only)
    #[arg(long)]
    no_lorem_start: bool,

    /// Emit compact JSON instead of pretty-printed
    #[arg(long)]
    compact: bool,

    /// List available themes as JSON and exit
    #[arg(long)]
    list_themes: bool,
}

fn main() {
    let args = Args::parse();

    if args.list_themes {
        println!("{}", serde_json::to_string_pretty(&list_themes()).unwrap());
        return;
    }

    let theme: Theme = match args.theme.parse() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };

    let opts = GeneratorOptions {
        theme,
        paragraphs: args.paragraphs,
        min_sentences: args.min_sentences,
        max_sentences: args.max_sentences,
        min_words: args.min_words,
        max_words: args.max_words,
        seed: args.seed,
        start_with_lorem: !args.no_lorem_start,
    };

    let output = generate(&opts);
    let json = if args.compact {
        serde_json::to_string(&output).unwrap()
    } else {
        serde_json::to_string_pretty(&output).unwrap()
    };
    println!("{json}");
}
