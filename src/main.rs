// src/main.rs

mod app;
mod components;
mod ui;

use app::App;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{io, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    // q quit ------------------------------------------------------------------------------------------------------
                    KeyCode::Char('q') => break,

                    // j and k to navigate only in projects ------------------------------------------------------------------------
                    KeyCode::Char('j') => {
                        if app.section == app::Section::Projects {
                            app.next_project();
                        }
                    }
                    KeyCode::Char('k') => {
                        if app.section == app::Section::Projects {
                            app.previous_project();
                        }
                    }

                    // enter and exit project----------------------------------------------------------------------------------------
                    KeyCode::Enter => {
                        app.open_project();
                    }
                    KeyCode::Char('b') => {
                        app.back();
                    }

                    // navigate thru sections with 1-5 -----------------------------------------------------------------------------
                    KeyCode::Char(c @ '1'..='5') => {
                        app.set_section(c);
                    }

                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
