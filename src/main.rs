pub mod app;
pub mod data;
pub mod event;
pub mod forecast;
pub mod input;
pub mod loops;
pub mod menu;
pub mod ui;
pub mod utils;
pub mod weather;

use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

//use crate::event::{Event, Events};
use crossterm::{
    event::{Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{prelude::CrosstermBackend, Terminal};

use crate::{
    app::App,
    event::Event,
    loops::{render_input, render_widgets},
    menu::MenuItem,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    render_input(tx, tick_rate);

    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let menu_titles = vec!["Home", "Weathers", "Add", "Search", "Quit"];
    let mut active_menu_item = MenuItem::Home;

    render_widgets(terminal);
    //let mut app = App::new();
    //let events = Events::new(Duration::from_millis(200));
    //
    //while app.running {
    //    terminal.draw(|f| ui::draw(f, &app))?;
    //
    //    match events.next()? {
    //        Event::Input(key) => match key.code {
    //            KeyCode::Char('q') => app.quit(),
    //            KeyCode::Char(c) => app.on_char(c),
    //            KeyCode::Backspace => app.on_backspace(),
    //            KeyCode::Up => app.move_up(),
    //            KeyCode::Down => app.move_down(),
    //            KeyCode::Enter => app.confirm(),
    //            _ => {}
    //        },
    //        Event::Tick => {
    //            app.on_tick();
    //        }
    //    }
    //}
    //
    //disable_raw_mode()?;
    //Ok(())
    //let file_path = "data/weather.json";
    //display_menu(file_path);
}
