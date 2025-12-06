use crate::weather::Weather;

pub struct App {
    pub query: String,
    pub cursor: usize,
    pub results: Vec<Weather>,
    pub mode: Mode,
    pub running: bool,
    pub file_path: String,
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
        if self.cursor < self.results.len() - 1 {
            self.cursor += 1;
        }
    }

    pub fn confirm(&mut self) {}

    pub fn on_tick(&mut self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }
}
