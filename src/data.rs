use crate::weather::Weather;
use colored::*;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};
use uuid::Uuid;

pub fn load_weathers_from_file(file_path: &str) -> HashMap<Uuid, Weather> {
    let file_content = fs::read_to_string(file_path);

    match file_content {
        Ok(data) => {
            let parsed: Result<Vec<Weather>, _> = serde_json::from_str(&data);

            match parsed {
                Ok(list) => {
                    println!(
                        "{}",
                        format!("Weathers loaded successfully from {file_path}")
                            .green()
                            .bold()
                    );

                    list.into_iter().map(|w| (w.id, w)).collect()
                }
                Err(error) => {
                    eprintln!(
                        "{}",
                        format!("❌ Failed to parse JSON: {error}").red().bold()
                    );
                    HashMap::new()
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
