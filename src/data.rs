use crate::weather::Weather;
use colored::*;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};
use uuid::Uuid;

pub fn load_weather() -> Vec<Weather> {
    let file_path = "data/weather.json";

    let data = fs::read_to_string(file_path).expect("Failed to read weather.json");

    let parsed: serde_json::Result<Vec<Weather>> = serde_json::from_str(&data);

    match parsed {
        Ok(list) => {
            println!(
                "{}",
                format!("✔  Weather loaded from {file_path}").green().bold()
            );
            list
        }
        Err(e) => {
            eprintln!("❌ Failed to parse JSON: {e}");
            vec![]
        }
    }
}

pub fn load_weathers_from_file(file_path: &str) -> HashMap<Uuid, Weather> {
    let file_content = fs::read_to_string(file_path);

    match file_content {
        Ok(data) => {
            let parsed: serde_json::Result<std::collections::HashMap<String, Weather>> =
                serde_json::from_str(&data);

            match parsed {
                Ok(map) => {
                    println!(
                        "{}",
                        format!("Weathers loaded successfully from {file_path}")
                            .green()
                            .bold()
                    );
                    map.into_values().collect()
                }
                Err(error) => {
                    eprintln!(
                        "{}",
                        format!("❌ Failed to parse JSON: {error}").red().bold()
                    );
                    vec![]
                }
            }
        }
        Err(_) => {
            println!(
                "{}",
                "⚠️ File not found, starting with an empty list of weathers."
                    .yellow()
                    .bold()
            );
            HashMap::new()
        }
    }
}

pub fn save_weathers_to_file(weathers: &HashMap<Uuid, Weather>, file_path: &str) {
    match File::create(file_path) {
        Ok(mut file) => {
            let json = serde_json::to_string_pretty(weathers).unwrap();
            if let Err(error) = file.write_all(json.as_bytes()) {
                eprintln!(
                    "{}",
                    format!("❌ Failed to write to file: {error}").red().bold()
                )
            } else {
                println!(
                    "{}",
                    format!("💾 Weathers saved successfully to {file_path}")
                        .green()
                        .bold()
                );
            }
        }
        Err(error) => eprintln!(
            "{}",
            format!("❌ Failed to create file: {error}").red().bold()
        ),
    }
}
