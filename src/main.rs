pub mod data;
pub mod forecast;
pub mod input;
pub mod menu;
pub mod utils;
pub mod weather;

use crate::menu::display_menu;

fn main() {
    let file_path = "data/weather.json";
    display_menu(file_path);
}
