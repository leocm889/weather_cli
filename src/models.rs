use std::fmt::{Display, Formatter, Result};

use colored::Colorize;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub enum WeatherCondition {
    Sunny,
    Rainy,
    Cloudy,
}

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub id: Uuid,
    pub city: String,
    pub temperature: i32,
    pub humidity: u8,
    pub condition: WeatherCondition,
}

impl Display for WeatherCondition {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let label = match self {
            WeatherCondition::Sunny => "Sunny",
            WeatherCondition::Rainy => "Rainy",
            WeatherCondition::Cloudy => "Cloudy",
        };
        write!(f, "{label}")
    }
}

impl Display for Weather {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let condition_str = self.condition.to_string();
        let condition_color = match condition_str.as_str() {
            "Sunny" => condition_str.yellow().bold(),
            "Rainy" => condition_str.blue().bold(),
            "Cloudy" => condition_str.truecolor(128, 128, 128).bold(),
            _ => condition_str.normal(),
        };
        let weather_block = format!(
            "{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n",
            "ID:".bold(),
            self.id.to_string().cyan(),
            "City:".bold(),
            self.city.bold(),
            "Temperature:".bold(),
            self.temperature.to_string().bold(),
            "Humidity:".bold(),
            self.humidity.to_string().bold(),
            "Condition:".bold(),
            condition_color,
        );

        f.write_str(&weather_block)
    }
}
