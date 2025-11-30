use crate::data::{load_weathers_from_file, save_weathers_to_file};
use crate::input::AddWeatherInput;
use crate::utils::read_input;
use crate::weather::{Weather, WeatherCondition};
use colored::*;
use uuid::Uuid;

pub fn search_weather<F>(file_path: &str, predicate: F)
where
    F: Fn(&Weather) -> bool,
{
    let weathers = load_weathers_from_file(file_path);

    if weathers.is_empty() {
        println!("{}", "⚠️ No Weathers found. The file is empty".yellow());
        return;
    }

    let results: Vec<&Weather> = weathers
        .values()
        .filter(|weather| predicate(weather))
        .collect();

    if results.is_empty() {
        println!("{}", "⚠️ No weathers found matching the criteria.".yellow());
    } else {
        println!(
            "{}",
            format!("Found {} weather(s):", results.len())
                .green()
                .bold()
        );
        for weather in results {
            println!("{weather}");
        }
    }
}

pub fn search_weather_by_id(file_path: &str, id: Uuid) {
    search_weather(file_path, move |weather| weather.id == id);
}

pub fn search_weather_by_name(file_path: &str, query: &str) {
    let query_lower = query.to_lowercase();
    search_weather(file_path, move |weather| {
        weather.city.to_lowercase().contains(&query_lower)
    });
}

pub fn add_weather(file_path: &str) {
    let mut weathers = load_weathers_from_file(file_path);

    let id = Uuid::new_v4();

    if weathers.contains_key(&id) {
        println!(
            "{}",
            "❌ A weather with this ID already exists. Try again.".red()
        );
        return;
    }

    println!("{}", "Enter city's name:".blue().bold());

    let city = read_input::<String>();

    println!("{}", "Enter city's tempperature:".blue().bold());

    let temperature = read_input::<i32>();

    println!("{}", "Enter city's humidity:".blue().bold());

    let humidity = read_input::<u8>();

    let condition = read_condition();

    let weather = Weather::new(AddWeatherInput {
        city,
        temperature,
        humidity,
        condition,
    });
    weathers.insert(weather.id, weather);
    save_weathers_to_file(&weathers, file_path);
    println!("{}", "Weather added successfully.".green().bold());
}

fn read_condition() -> WeatherCondition {
    loop {
        println!("{}", "Choose weather condition of the city:".blue().bold());
        println!("{}", "1. Sunny".yellow().bold());
        println!("{}", "2. Rainy".blue().bold());
        println!("{}", "3. Cloudy".truecolor(128, 128, 128).bold());

        let choice = read_input::<u32>();

        match choice {
            1 => return WeatherCondition::Sunny,
            2 => return WeatherCondition::Rainy,
            3 => return WeatherCondition::Cloudy,
            _ => {
                println!("{}", "❌ Invalid choice, try again.".red().bold());
                continue;
            }
        }
    }
}
