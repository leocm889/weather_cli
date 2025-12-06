use crate::data::{load_weathers_from_file, save_weathers_to_file};
use crate::input::AddWeatherInput;
use crate::utils::read_input;
use crate::weather::{Temperature, Weather, WeatherCondition};
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

pub fn search_weather_by_temp_range(file_path: &str, min: f32, max: f32) {
    search_weather(file_path, move |weather| {
        let temp_c = weather.temperature.to_celsius();
        temp_c >= min && temp_c <= max
    });
}

pub fn search_menu(file_path: &str) {
    loop {
        println!("{}", "Search by:".blue().bold());
        println!("{}", "1. ID".cyan());
        println!("{}", "2. City".magenta());
        println!("{}", "3. Temperature range".yellow());
        println!("{}", "4. Back to main menu".red());

        let choice = read_input::<u32>();

        match choice {
            1 => {
                println!("{}", "Enter the ID to search:".blue().bold());
                let id_input = read_input::<String>();
                match Uuid::parse_str(&id_input) {
                    Ok(id) => {
                        search_weather_by_id(file_path, id);
                    }
                    Err(_) => println!("{}", "⚠️ Invalid UUID format.".red()),
                }
            }
            2 => {
                println!("{}", "Enter the name of the city to search:".blue().bold());
                let name_query = read_input::<String>();
                search_weather_by_name(file_path, &name_query);
            }
            3 => {
                println!("{}", "Enter minimum temperature".blue().bold());
                let min = read_input::<f32>();

                println!("{}", "Enter maximum temperature".blue().bold());
                let max = read_input::<f32>();

                search_weather_by_temp_range(file_path, min, max);
            }
            4 => break,
            _ => println!("{}", "❌ Invalid choice, try again.".red().bold()),
        }
    }
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

    println!(
        "{}",
        "Enter city's temperature (e.g., 25C, 77F):".blue().bold()
    );
    let temp_input = read_input::<String>();
    let temperature = parse_temperature(&temp_input);

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

pub fn retrive_weathers_sorted(file_path: &str) {
    loop {
        let weathers = load_weathers_from_file(file_path);

        if weathers.is_empty() {
            println!("{}", " No weathers found.".red().bold());
            return;
        }

        println!("{}", "Sort weathers by:".blue().bold());
        println!("{}", "1. Temperature ascending".yellow());
        println!("{}", "2. Temperature descending".cyan());
        println!("{}", "3. City name".magenta());
        println!("{}", "4. Back to main menu".red());

        let choice = read_input::<u32>();

        let mut weather_list: Vec<&Weather> = weathers.values().collect();

        match choice {
            1 => {
                weather_list.sort_by(|a, b| {
                    a.temperature
                        .to_celsius()
                        .partial_cmp(&b.temperature.to_celsius())
                        .unwrap()
                });
            }
            2 => {
                weather_list.sort_by(|a, b| {
                    b.temperature
                        .to_celsius()
                        .partial_cmp(&a.temperature.to_celsius())
                        .unwrap()
                });
            }
            3 => {
                weather_list.sort_by(|a, b| a.city.cmp(&b.city));
            }
            4 => break,
            _ => {
                println!(
                    "{}",
                    "Invalid choice, displaying in default creation order."
                        .red()
                        .bold()
                );
            }
        }

        println!("{}", "--- Weathers ---".bold().blue());
        for weather in &weather_list {
            println!("{weather}");
        }
    }
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

fn parse_temperature(input: &str) -> Temperature {
    let input = input.trim().to_uppercase();

    if input.ends_with("C") {
        let value = input.trim_end_matches("C").parse::<f32>().unwrap();
        Temperature::Celsius(value)
    } else if input.ends_with("F") {
        let value = input.trim_end_matches("F").parse::<f32>().unwrap();
        Temperature::Fahrenheit(value)
    } else {
        panic!("Invalid format. Example: 25C or 77F");
    }
}
