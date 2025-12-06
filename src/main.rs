pub mod app;
pub mod data;
pub mod forecast;
pub mod input;
pub mod menu;
pub mod utils;
pub mod weather;
pub mod ui;
pub mod event;

use std::time::Duration;

use crossterm::{terminal::{disable_raw_mode, enable_raw_mode}};
use crate::event::{Event, Events};
use ratatui::{prelude::CrosstermBackend, Terminal};

use crate::{app::App, menu::display_menu};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let events = Events::new(Duration::from_millis(200));

    while app.running {
        terminal.draw(|f| ui::draw(f, &app))?;
        
        match events.next()? {
            Event::Input(key) => {
                match key.code {

                    KeyCode::Char('q') => app.quit(),
                    KeyCode::Char(c) => app.on_char(c),
                    KeyCode::Backspace => {
                        app.on_backspace()
                    }
                    KeyCode::Up => {
                        app.move_up()
                    }
                    KeyCode::Down => {
                        app.move_down()
                    }
                    KeyCode::Enter => {
                        app.confirm(),
                    }
                    _ => {}
                }
                Event::Tick => {
                    app.on_tick();
                }
            }
        }

        if event::poll(std::time:::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => app.quit(),
                    KeyCode::Char(c) => app.query.push(c),
                    KeyCode::Backspace => {
                        app.query.pop();
                    }
                    KeyCode::Up => {
                        if app.cursor > 0 {
                            app.cursor -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if app.cursor < app.results.len() - 1 {
                            app.cursor += 1;
                        }
                    }
                    KeyCode::Enter => {

                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
    //let file_path = "data/weather.json";
    //display_menu(file_path);
}
