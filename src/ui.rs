use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, Mode};

pub fn draw(f: &mut Frame, app: &App) {
    //match app.mode {
    //    Mode::Search => draw_search(f, app),
    //    Mode::Details => draw_details(f, app),
    //    Mode::Add => draw_add_form(f, app),
    //    Mode::Sort => draw_sort_menu(f, app),
    //}
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(f.size());

    let search = Paragraph::new(app.query.clone())
        .block(Block::default().title("Search City").borders(Borders::ALL));

    f.render_widget(search, chunks[0]);

    let results = Paragraph::new(format_results(app))
        .block(Block::default().title("Results").borders(Borders::ALL));

    f.render_widget(results, chunks[1]);
}

//fn draw_details(f: &mut Frame, app: &App) {
//    let weather = &app.results[app.cursor];
//    let text = format!("{}", weather);
//    let block = Paragraph::new(text).block(Block::default().title("Details").borders(Borders::ALL));
//
//    f.render_widget(block, f.size());
//}

fn format_results(app: &App) -> String {
    if app.results.is_empty() {
        "No results".into()
    } else {
        app.results
            .iter()
            .enumerate()
            .map(|(i, w)| {
                if i == app.cursor {
                    format!(
                        "> {} {}°C {}",
                        w.city,
                        w.temperature.to_celsius(),
                        w.condition
                    )
                } else {
                    format!(
                        "  {} {}°C {}",
                        w.city,
                        w.temperature.to_celsius(),
                        w.condition
                    )
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
