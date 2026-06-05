//! Infinite streaming: paced emission of generator items to stdout, in
//! plain text or JSON Lines. Both end quietly on a broken pipe.

use std::io::Write;
use std::thread;
use std::time::Duration;

use lorem_core::{Generator, GeneratorOptions, Mode};

/// Stream plain prose forever: words flow with ~72-column wrapping,
/// sentences land one per line, paragraphs get a blank line between them.
/// The seed goes to stderr so stdout stays pure text.
pub(crate) fn text(opts: &GeneratorOptions, interval: Duration) {
    let mut generator = Generator::new(opts);
    eprintln!("seed: {}", generator.seed());

    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    let mut wrap = LineWrap::new(72);

    let mut first = true;
    loop {
        if !first {
            thread::sleep(interval);
        }
        first = false;

        let item = generator.next_item();
        let written = match opts.mode {
            Mode::Words => write!(out, "{}{item}", wrap.separator(item.len())),
            Mode::Sentences => writeln!(out, "{item}"),
            Mode::Paragraphs => writeln!(out, "{item}\n"),
        };
        if written.and_then(|()| out.flush()).is_err() {
            return; // reader hung up
        }
    }
}

/// Stream JSON Lines forever: a meta line carrying the seed first, then one
/// object per item.
pub(crate) fn json(opts: &GeneratorOptions, interval: Duration) {
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

/// Column-budget line wrapping: decides whether the next word is preceded
/// by nothing (line start), a space, or a newline.
pub(crate) struct LineWrap {
    width: usize,
    line_len: usize,
}

impl LineWrap {
    pub(crate) fn new(width: usize) -> Self {
        LineWrap { width, line_len: 0 }
    }

    pub(crate) fn separator(&mut self, word_len: usize) -> &'static str {
        if self.line_len == 0 {
            self.line_len = word_len;
            ""
        } else if self.line_len + 1 + word_len > self.width {
            self.line_len = word_len;
            "\n"
        } else {
            self.line_len += 1 + word_len;
            " "
        }
    }
}

#[cfg(test)]
mod tests;
