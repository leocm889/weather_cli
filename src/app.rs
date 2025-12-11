use std::time::{Duration, Instant};

use crate::{data::load_weathers_from_file, weather::Weather};

pub struct App {
    pub query: String,
    pub cursor: usize,
    pub results: Vec<Weather>,
    pub mode: Mode,
    pub running: bool,
    pub file_path: String,
    pub last_tick: Instant,
    pub tick_rate: Duration,
}

pub enum Mode {
    Search,
    Add,
    Details,
    Sort,
}

impl App {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            cursor: 0,
            results: vec![],
            mode: Mode::Search,
            running: true,
            file_path: "data/weather.json".to_string(),
            last_tick: Instant::now(),
            tick_rate: Duration::from_secs(5),
        }
    }

    pub fn search(&mut self) {
        let weathers = load_weathers_from_file(&self.file_path);

        if self.query.is_empty() {
            self.results = weathers.values().cloned().collect();
            self.cursor = 0;
            return;
        }

        let lower = self.query.to_lowercase();
        self.results = weathers
            .values()
            .filter(|w| w.city.to_lowercase().contains(&lower))
            .cloned()
            .collect();

        if self.cursor >= self.results.len() {
            self.cursor = 0;
        }
    }

    pub fn on_char(&mut self, c: char) {
        self.query.push(c);
        self.search()
    }

    pub fn on_backspace(&mut self) {
        self.query.pop();
        self.search();
    }

    pub fn move_up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if !self.results.is_empty() && self.cursor < self.results.len() - 1 {
            self.cursor += 1;
        }
    }

    pub fn confirm(&mut self) {
        if self.results.is_empty() {
            return;
        }
        self.mode = Mode::Details;
    }

    pub fn on_tick(&mut self) {
        if self.last_tick.elapsed() >= self.tick_rate {
            self.search();
            self.last_tick = Instant::now();
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
