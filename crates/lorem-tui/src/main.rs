//! `lorem-tui` — interactive ratatui front end. Entry point and event loop
//! only: state and key handling live in `app`, rendering in `ui`, the
//! palette in `theme`.

mod app;
mod theme;
mod ui;

use ratatui::crossterm::event::{self, Event, KeyEventKind};

use crate::app::App;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();
    app.generate();

    while !app.quit {
        terminal.draw(|f| ui::draw(f, &mut app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                app.on_key(key.code);
            }
        }
    }

    ratatui::restore();
    Ok(())
}
