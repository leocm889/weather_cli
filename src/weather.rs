use std::fmt::{Display, Formatter, Result};

use colored::Colorize;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::input::AddWeatherInput;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Deserialize)]
pub enum WeatherCondition {
    Sunny,
    Rainy,
    Cloudy,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Deserialize)]
pub enum Temperature {
    Celsius(f32),
    Fahrenheit(f32),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Weather {
    pub id: Uuid,
    pub city: String,
    pub temperature: Temperature,
    pub humidity: u8,
    pub condition: WeatherCondition,
}

impl WeatherCondition {
    fn emoji(&self) -> &'static str {
        match self {
            WeatherCondition::Sunny => "☀️",
            WeatherCondition::Rainy => "🌧",
            WeatherCondition::Cloudy => "☁️",
        }
    }
}

impl Display for WeatherCondition {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{} {}",
            self.emoji(),
            match self {
                WeatherCondition::Sunny => "Sunny",
                WeatherCondition::Rainy => "Rainy",
                WeatherCondition::Cloudy => "Cloudy",
            }
        )
    }
}

impl Temperature {
    pub fn to_celsius(&self) -> f32 {
        match self {
            Temperature::Celsius(c) => *c,
            Temperature::Fahrenheit(f) => (*f - 32.0) * 5.0 / 9.0,
        }
    }

    pub fn to_fahrenheit(&self) -> f32 {
        match self {
            Temperature::Celsius(c) => (*c * 9.0 / 5.0) + 32.0,
            Temperature::Fahrenheit(f) => *f,
        }
    }

    pub fn display_celsius(self) -> String {
        format!("{:.1} °C", self.to_celsius())
    }

    pub fn display_fahrenheit(self) -> String {
        format!("{:.1} °F", self.to_fahrenheit())
    }
}

impl Display for Temperature {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.display_celsius())
    }
}

impl Display for Weather {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let weather_block = format!(
            "{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n",
            "ID:".bold(),
            self.id.to_string().cyan(),
            "City:".bold(),
            self.city.bold(),
            "Temperature:".bold(),
            self.temperature,
            "Humidity:".bold(),
            self.humidity.to_string().bold(),
            "Condition:".bold(),
            match self.condition {
                WeatherCondition::Sunny => self.condition.to_string().yellow().bold(),
                WeatherCondition::Rainy => self.condition.to_string().blue().bold(),
                WeatherCondition::Cloudy =>
                    self.condition.to_string().truecolor(128, 128, 128).bold(),
            }
        );

        f.write_str(&weather_block)
    }
}

impl Weather {
    pub fn new(input: AddWeatherInput) -> Self {
        Self {
            id: Uuid::new_v4(),
            city: input.city,
            temperature: input.temperature,
            humidity: input.humidity,
            condition: input.condition,
        }
    }
}
