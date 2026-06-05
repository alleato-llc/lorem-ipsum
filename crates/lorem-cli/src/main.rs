//! `lorem` — Markov-chain lorem ipsum generator. Entry point and dispatch
//! only: argument definitions live in `args`, defaults merging in `resolve`,
//! streaming in `stream`.

mod args;
mod resolve;
mod stream;

use clap::Parser;
use lorem_core::{generate, list_themes, load_settings, save_settings};

use crate::args::{Args, OutputFormat};

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

    let opts = match resolve::options(&args, load_settings()) {
        Ok(opts) => opts,
        Err(e) => fail(e),
    };

    if args.save_defaults {
        match save_settings(&opts) {
            Ok(path) => eprintln!("saved defaults to {}", path.display()),
            Err(e) => fail(format!("could not save defaults: {e}")),
        }
    }

    if args.infinite {
        let interval = resolve::interval(opts.mode, args.interval);
        match args.output_format {
            OutputFormat::Txt => stream::text(&opts, interval),
            OutputFormat::Json => stream::json(&opts, interval),
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
