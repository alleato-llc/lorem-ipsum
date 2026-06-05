//! Rendering only: draws the form, output table, and footer from `App`
//! state. No state mutation beyond the table's scroll position.

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table};
use ratatui::Frame;

use crate::app::{App, Focus, FIELDS, OPENER_FIELD};
use crate::theme::{ACCENT, ACCENT_LIGHT, ROW_HIGHLIGHT};

pub(crate) fn draw(f: &mut Frame, app: &mut App) {
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
    draw_footer(f, app, outer[1]);
}

fn key_hint(key: &str, action: &str) -> [Span<'static>; 2] {
    [
        Span::styled(key.to_string(), Style::new().bold().fg(ACCENT_LIGHT)),
        Span::raw(format!(" {action}  ")),
    ]
}

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let footer = match &app.status {
        Some(msg) => Line::from(Span::styled(
            format!(" {msg}"),
            Style::new().fg(ACCENT_LIGHT).italic(),
        )),
        None => {
            let mut spans = vec![Span::raw(" ")];
            for (key, action) in [
                ("Tab", "switch"),
                ("↑↓", "navigate"),
                ("←→", "adjust"),
                ("Enter/g", "generate"),
                ("s", "save defaults"),
                ("q", "quit"),
            ] {
                spans.extend(key_hint(key, action));
            }
            Line::from(spans)
        }
    };
    f.render_widget(Paragraph::new(footer), area);
}

fn border_style(focused: bool) -> Style {
    if focused {
        Style::new().fg(ACCENT)
    } else {
        Style::new().fg(Color::DarkGray)
    }
}

fn field_value(app: &App, i: usize) -> String {
    match i {
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
    }
}

fn draw_form(f: &mut Frame, app: &App, area: Rect) {
    let focused = app.focus == Focus::Form;

    let mut lines = Vec::new();
    for (i, name) in FIELDS.iter().enumerate() {
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
            Span::styled(field_value(app, i), style),
        ]));
        lines.push(Line::raw(""));
    }
    lines.push(Line::from(Span::styled(
        "  [ Enter: Generate ]",
        Style::new().fg(ACCENT_LIGHT).bold(),
    )));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style(focused))
        .title(" Options ");
    f.render_widget(Paragraph::new(lines).block(block), area);
}

fn output_title(app: &App) -> String {
    match &app.result {
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
    }
}

fn draw_output(f: &mut Frame, app: &mut App, area: Rect) {
    let focused = app.focus == Focus::Output;
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style(focused))
        .title(output_title(app));

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
