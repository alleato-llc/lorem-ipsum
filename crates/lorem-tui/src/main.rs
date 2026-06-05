//! `lorem-tui` — interactive ratatui front end: a form for generator options
//! and a scrollable table of generated text.

use lorem_core::{
    generate, load_settings, save_settings, GeneratedText, GeneratorOptions, Mode, Theme,
};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState};
use ratatui::Frame;

// Royal purple palette, matching the GUI (gui/src/styles.css).
const ACCENT: Color = Color::Rgb(120, 81, 169); // #7851a9 — fills, borders
const ACCENT_LIGHT: Color = Color::Rgb(167, 139, 219); // #a78bdb — text accents
const ROW_HIGHLIGHT: Color = Color::Rgb(46, 38, 66); // muted purple row bg

const FIELDS: [&str; 9] = [
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
const OPENER_FIELD: usize = 7;
const SEED_FIELD: usize = 8;

#[derive(PartialEq)]
enum Focus {
    Form,
    Output,
}

struct App {
    focus: Focus,
    field: usize,
    theme_idx: usize,
    mode_idx: usize,
    count: usize,
    min_sentences: usize,
    max_sentences: usize,
    min_words: usize,
    max_words: usize,
    start_with_lorem: bool,
    seed_input: String,
    result: Option<GeneratedText>,
    table_state: TableState,
    status: Option<String>,
    quit: bool,
}

impl App {
    fn new() -> Self {
        let d = load_settings();
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

    fn theme(&self) -> Theme {
        Theme::ALL[self.theme_idx]
    }

    fn mode(&self) -> Mode {
        Mode::ALL[self.mode_idx]
    }

    fn count_max(&self) -> usize {
        match self.mode() {
            Mode::Words => 500,
            Mode::Sentences => 100,
            Mode::Paragraphs => 50,
        }
    }

    fn options(&self) -> GeneratorOptions {
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

    fn generate(&mut self) {
        self.result = Some(generate(&self.options()));
        self.table_state.select(Some(0));
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

    /// Whether a form field applies to the current mode (inapplicable ones
    /// render dimmed).
    fn field_active(&self, i: usize) -> bool {
        match i {
            3 | 4 => self.mode() == Mode::Paragraphs,
            5 | 6 => self.mode() != Mode::Words,
            OPENER_FIELD => self.theme() == Theme::Classic,
            _ => true,
        }
    }

    fn save_defaults(&mut self) {
        self.status = Some(match save_settings(&self.options()) {
            Ok(path) => format!("Saved defaults to {}", path.display()),
            Err(e) => format!("Could not save defaults: {e}"),
        });
    }

    fn on_key(&mut self, code: KeyCode) {
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

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();
    app.generate();

    while !app.quit {
        terminal.draw(|f| draw(f, &mut app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                app.on_key(key.code);
            }
        }
    }

    ratatui::restore();
    Ok(())
}

fn draw(f: &mut Frame, app: &mut App) {
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(34), Constraint::Min(0)])
        .split(outer[0]);

    draw_form(f, app, cols[0]);
    draw_output(f, app, cols[1]);

    let footer = match &app.status {
        Some(msg) => Line::from(Span::styled(
            format!(" {msg}"),
            Style::new().fg(ACCENT_LIGHT).italic(),
        )),
        None => Line::from(vec![
            Span::styled(" Tab ", Style::new().bold().fg(ACCENT_LIGHT)),
            Span::raw("switch  "),
            Span::styled("↑↓", Style::new().bold().fg(ACCENT_LIGHT)),
            Span::raw(" navigate  "),
            Span::styled("←→", Style::new().bold().fg(ACCENT_LIGHT)),
            Span::raw(" adjust  "),
            Span::styled("Enter/g", Style::new().bold().fg(ACCENT_LIGHT)),
            Span::raw(" generate  "),
            Span::styled("s", Style::new().bold().fg(ACCENT_LIGHT)),
            Span::raw(" save defaults  "),
            Span::styled("q", Style::new().bold().fg(ACCENT_LIGHT)),
            Span::raw(" quit"),
        ]),
    };
    f.render_widget(Paragraph::new(footer), outer[1]);
}

fn draw_form(f: &mut Frame, app: &App, area: Rect) {
    let focused = app.focus == Focus::Form;
    let border_style = if focused {
        Style::new().fg(ACCENT)
    } else {
        Style::new().fg(Color::DarkGray)
    };

    let mut lines = Vec::new();
    for (i, name) in FIELDS.iter().enumerate() {
        let value = match i {
            0 => app.theme().display_name().to_string(),
            1 => app.mode().display_name().to_string(),
            2 => app.count.to_string(),
            3 => app.min_sentences.to_string(),
            4 => app.max_sentences.to_string(),
            5 => app.min_words.to_string(),
            6 => app.max_words.to_string(),
            OPENER_FIELD => if app.start_with_lorem { "on" } else { "off" }.to_string(),
            _ => {
                if app.seed_input.is_empty() {
                    "(random)".to_string()
                } else {
                    app.seed_input.clone()
                }
            }
        };
        let selected = focused && app.field == i;
        let style = if selected {
            Style::new().fg(Color::White).bg(ACCENT)
        } else if app.field_active(i) {
            Style::new()
        } else {
            Style::new().fg(Color::DarkGray)
        };
        let marker = if selected { "▸ " } else { "  " };
        lines.push(Line::from(vec![
            Span::raw(marker),
            Span::styled(format!("{name:<14}"), style.add_modifier(Modifier::BOLD)),
            Span::styled(value, style),
        ]));
        lines.push(Line::raw(""));
    }
    lines.push(Line::from(Span::styled(
        "  [ Enter: Generate ]",
        Style::new().fg(ACCENT_LIGHT).bold(),
    )));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(" Options ");
    f.render_widget(Paragraph::new(lines).block(block), area);
}

fn draw_output(f: &mut Frame, app: &mut App, area: Rect) {
    let focused = app.focus == Focus::Output;
    let border_style = if focused {
        Style::new().fg(ACCENT)
    } else {
        Style::new().fg(Color::DarkGray)
    };

    let title = match &app.result {
        Some(r) => {
            let sentences = if r.sentence_count > 0 {
                format!(" · {} sentences", r.sentence_count)
            } else {
                String::new()
            };
            format!(
                " {} · {} · {} words{} · seed {} ",
                r.theme_name,
                r.mode.display_name().to_lowercase(),
                r.word_count,
                sentences,
                r.seed
            )
        }
        None => " Output ".to_string(),
    };
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(title);

    let Some(result) = &app.result else {
        f.render_widget(Paragraph::new("Press Enter to generate").block(block), area);
        return;
    };

    // Wrap each item to the text column width; row height = line count.
    let text_width = area.width.saturating_sub(2 + 4 + 7 + 4).max(20) as usize;
    let rows: Vec<Row> = result
        .items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let wrapped = textwrap::wrap(item, text_width);
            let height = wrapped.len() as u16 + 1;
            let text = Text::from(
                wrapped
                    .iter()
                    .map(|l| Line::raw(l.to_string()))
                    .collect::<Vec<_>>(),
            );
            Row::new(vec![
                Cell::from(format!("{}", i + 1)),
                Cell::from(format!("{}", item.split_whitespace().count())),
                Cell::from(text),
            ])
            .height(height)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(4),
            Constraint::Length(7),
            Constraint::Min(20),
        ],
    )
    .header(
        Row::new(vec!["#", "Words", result.mode.item_label()])
            .style(Style::new().bold().fg(ACCENT_LIGHT))
            .bottom_margin(1),
    )
    .block(block)
    .row_highlight_style(Style::new().bg(ROW_HIGHLIGHT))
    .highlight_symbol("▌");

    f.render_stateful_widget(table, area, &mut app.table_state);
}
