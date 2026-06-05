//! Application state and key handling — no rendering. Everything here is
//! exercised by unit tests without a terminal.

use lorem_core::{
    generate, load_settings, save_settings, GeneratedText, GeneratorOptions, Mode, Theme,
};
use ratatui::crossterm::event::KeyCode;
use ratatui::widgets::TableState;

pub(crate) const FIELDS: [&str; 9] = [
    "Theme",
    "Mode",
    "Count",
    "Min sentences",
    "Max sentences",
    "Min words",
    "Max words",
    "Lorem opener",
    "Seed",
];
pub(crate) const OPENER_FIELD: usize = 7;
pub(crate) const SEED_FIELD: usize = 8;

#[derive(PartialEq)]
pub(crate) enum Focus {
    Form,
    Output,
}

pub(crate) struct App {
    pub(crate) focus: Focus,
    pub(crate) field: usize,
    pub(crate) theme_idx: usize,
    pub(crate) mode_idx: usize,
    pub(crate) count: usize,
    pub(crate) min_sentences: usize,
    pub(crate) max_sentences: usize,
    pub(crate) min_words: usize,
    pub(crate) max_words: usize,
    pub(crate) start_with_lorem: bool,
    pub(crate) seed_input: String,
    pub(crate) result: Option<GeneratedText>,
    pub(crate) table_state: TableState,
    pub(crate) status: Option<String>,
    pub(crate) quit: bool,
}

impl App {
    pub(crate) fn new() -> Self {
        Self::from_options(load_settings())
    }

    /// Build the form state from a set of options (saved defaults).
    pub(crate) fn from_options(d: GeneratorOptions) -> Self {
        App {
            focus: Focus::Form,
            field: 0,
            theme_idx: Theme::ALL.iter().position(|t| *t == d.theme).unwrap_or(0),
            mode_idx: Mode::ALL.iter().position(|m| *m == d.mode).unwrap_or(0),
            count: d.count,
            min_sentences: d.min_sentences,
            max_sentences: d.max_sentences,
            min_words: d.min_words,
            max_words: d.max_words,
            start_with_lorem: d.start_with_lorem,
            seed_input: String::new(),
            result: None,
            table_state: TableState::default(),
            status: None,
            quit: false,
        }
    }

    pub(crate) fn theme(&self) -> Theme {
        Theme::ALL[self.theme_idx]
    }

    pub(crate) fn mode(&self) -> Mode {
        Mode::ALL[self.mode_idx]
    }

    pub(crate) fn count_max(&self) -> usize {
        match self.mode() {
            Mode::Words => 500,
            Mode::Sentences => 100,
            Mode::Paragraphs => 50,
        }
    }

    pub(crate) fn options(&self) -> GeneratorOptions {
        GeneratorOptions {
            theme: self.theme(),
            mode: self.mode(),
            count: self.count.min(self.count_max()),
            min_sentences: self.min_sentences,
            max_sentences: self.max_sentences.max(self.min_sentences),
            min_words: self.min_words,
            max_words: self.max_words.max(self.min_words),
            seed: self.seed_input.parse().ok(),
            start_with_lorem: self.start_with_lorem,
        }
    }

    pub(crate) fn generate(&mut self) {
        self.result = Some(generate(&self.options()));
        self.table_state.select(Some(0));
    }

    fn save_defaults(&mut self) {
        self.status = Some(match save_settings(&self.options()) {
            Ok(path) => format!("Saved defaults to {}", path.display()),
            Err(e) => format!("Could not save defaults: {e}"),
        });
    }

    /// Adjust the focused field by `delta`: themes and modes cycle, numbers
    /// step within their bounds.
    fn adjust(&mut self, delta: i64) {
        let count_max = self.count_max();
        let bump = |v: &mut usize, lo: usize, hi: usize| {
            *v = (*v as i64 + delta).clamp(lo as i64, hi as i64) as usize;
        };
        match self.field {
            0 => {
                let n = Theme::ALL.len() as i64;
                self.theme_idx = ((self.theme_idx as i64 + delta).rem_euclid(n)) as usize;
            }
            1 => {
                let n = Mode::ALL.len() as i64;
                self.mode_idx = ((self.mode_idx as i64 + delta).rem_euclid(n)) as usize;
                self.count = self.count.min(self.count_max());
            }
            2 => {
                // Words add up fast — step by 10 in words mode.
                let step = if self.mode() == Mode::Words { 10 } else { 1 };
                let next = self.count as i64 + delta * step;
                self.count = next.clamp(1, count_max as i64) as usize;
            }
            3 => bump(&mut self.min_sentences, 1, 20),
            4 => bump(&mut self.max_sentences, 1, 20),
            5 => bump(&mut self.min_words, 3, 40),
            6 => bump(&mut self.max_words, 3, 40),
            OPENER_FIELD => self.start_with_lorem = !self.start_with_lorem,
            _ => {}
        }
    }

    /// Whether a form field applies to the current mode/theme (inapplicable
    /// ones render dimmed).
    pub(crate) fn field_active(&self, i: usize) -> bool {
        match i {
            3 | 4 => self.mode() == Mode::Paragraphs,
            5 | 6 => self.mode() != Mode::Words,
            OPENER_FIELD => self.theme() == Theme::Classic,
            _ => true,
        }
    }

    pub(crate) fn on_key(&mut self, code: KeyCode) {
        self.status = None;
        match code {
            KeyCode::Char('q') | KeyCode::Esc => self.quit = true,
            KeyCode::Tab => {
                self.focus = match self.focus {
                    Focus::Form => Focus::Output,
                    Focus::Output => Focus::Form,
                };
            }
            KeyCode::Enter | KeyCode::Char('g') => self.generate(),
            KeyCode::Char('s') => self.save_defaults(),
            _ => match self.focus {
                Focus::Form => self.on_form_key(code),
                Focus::Output => self.on_output_key(code),
            },
        }
    }

    fn on_form_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Up => self.field = self.field.saturating_sub(1),
            KeyCode::Down => self.field = (self.field + 1).min(FIELDS.len() - 1),
            KeyCode::Left => self.adjust(-1),
            KeyCode::Right => self.adjust(1),
            KeyCode::Char(c @ '0'..='9') if self.field == SEED_FIELD => {
                if self.seed_input.len() < 10 {
                    self.seed_input.push(c);
                }
            }
            KeyCode::Backspace if self.field == SEED_FIELD => {
                self.seed_input.pop();
            }
            _ => {}
        }
    }

    fn on_output_key(&mut self, code: KeyCode) {
        let len = self.result.as_ref().map_or(0, |r| r.items.len());
        if len == 0 {
            return;
        }
        let cur = self.table_state.selected().unwrap_or(0);
        match code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.table_state.select(Some(cur.saturating_sub(1)));
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.table_state.select(Some((cur + 1).min(len - 1)));
            }
            KeyCode::Home => self.table_state.select(Some(0)),
            KeyCode::End => self.table_state.select(Some(len - 1)),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests;
