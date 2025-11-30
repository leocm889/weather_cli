use colored::*;
pub mod data;
pub mod forecast;
pub mod models;
pub mod utils;
use forecast::search_weather_by_name;
use utils::read_input;

fn main() {
    println!("{}", "🎉 Welcome to the Weather Forecast".bold().cyan());
    let file_path = "data/weather.json";

    loop {
        println!("{}", "Please enter city's name:".blue().bold());
        let input_query = read_input::<String>();
        search_weather_by_name(file_path, &input_query);
    }
}
