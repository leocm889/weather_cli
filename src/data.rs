use crate::models::Weather;
use colored::*;
use std::{collections::HashMap, fs};
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
