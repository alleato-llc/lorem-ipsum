//! `lorem` — Markov-chain lorem ipsum generator with JSON output.

use std::io::Write;
use std::thread;
use std::time::Duration;

use clap::{Parser, ValueEnum};
use lorem_core::{
    generate, list_themes, load_settings, save_settings, Generator, GeneratorOptions, Mode, Theme,
};

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

    /// Stream forever, ignoring --count, one word/sentence/paragraph at a
    /// time (per --mode)
    #[arg(long)]
    infinite: bool,

    /// Format for --infinite: pipe-friendly plain text (seed on stderr),
    /// or JSON Lines (meta line first, then one object per item)
    #[arg(long, value_enum, default_value_t = OutputFormat::Txt)]
    output_format: OutputFormat,

    /// Milliseconds between streamed items
    /// (default: 100 words / 400 sentences / 900 paragraphs)
    #[arg(long, value_name = "MS")]
    interval: Option<u64>,

    /// Emit compact JSON instead of pretty-printed
    #[arg(long)]
    compact: bool,

    /// List available themes as JSON and exit
    #[arg(long)]
    list_themes: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
enum OutputFormat {
    Txt,
    Json,
}

fn fail(msg: impl std::fmt::Display) -> ! {
    eprintln!("error: {msg}");
    std::process::exit(1);
}

fn resolve_interval(opts: &GeneratorOptions, interval: Option<u64>) -> Duration {
    Duration::from_millis(interval.unwrap_or(match opts.mode {
        Mode::Words => 100,
        Mode::Sentences => 400,
        Mode::Paragraphs => 900,
    }))
}

/// Stream JSON Lines forever, pausing `interval` between items. Stops
/// quietly when the reader goes away (broken pipe).
fn stream_json(opts: &GeneratorOptions, interval: Duration) {
    let mut generator = Generator::new(opts);

    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    let mut emit = |line: serde_json::Value| -> bool {
        writeln!(out, "{line}").and_then(|()| out.flush()).is_ok()
    };

    let meta = serde_json::json!({
        "theme": opts.theme.id(),
        "mode": opts.mode.id(),
        "seed": generator.seed(),
    });
    if !emit(meta) {
        return;
    }

    for index in 0usize.. {
        if index > 0 {
            thread::sleep(interval);
        }
        let line = serde_json::json!({ "index": index, "text": generator.next_item() });
        if !emit(line) {
            return;
        }
    }
}

/// Stream plain prose to stdout forever: words flow with ~72-column
/// wrapping, sentences land one per line, paragraphs get a blank line
/// between them. The seed goes to stderr so stdout stays pure text.
/// Stops quietly when the reader goes away (broken pipe).
fn stream_text(opts: &GeneratorOptions, interval: Duration) {
    let mut generator = Generator::new(opts);
    eprintln!("seed: {}", generator.seed());

    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    let mut line_len = 0usize;

    let mut first = true;
    loop {
        if !first {
            thread::sleep(interval);
        }
        first = false;

        let item = generator.next_item();
        let written = match opts.mode {
            Mode::Words => {
                let sep = if line_len == 0 {
                    ""
                } else if line_len + 1 + item.len() > 72 {
                    line_len = 0;
                    "\n"
                } else {
                    " "
                };
                let res = write!(out, "{sep}{item}");
                line_len += item.len() + usize::from(!sep.is_empty());
                res
            }
            Mode::Sentences => writeln!(out, "{item}"),
            Mode::Paragraphs => writeln!(out, "{item}\n"),
        };
        if written.and_then(|()| out.flush()).is_err() {
            return; // reader hung up
        }
    }
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

    if args.infinite {
        let interval = resolve_interval(&opts, args.interval);
        match args.output_format {
            OutputFormat::Txt => stream_text(&opts, interval),
            OutputFormat::Json => stream_json(&opts, interval),
        }
        return;
    }

    let output = generate(&opts);
    let json = if args.compact {
        serde_json::to_string(&output).unwrap()
    } else {
        serde_json::to_string_pretty(&output).unwrap()
    };
    println!("{json}");
}
