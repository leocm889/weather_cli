use crate::data::load_weathers_from_file;
use crate::models::Weather;
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
