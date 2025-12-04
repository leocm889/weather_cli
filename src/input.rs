use crate::weather::{Temperature, WeatherCondition};

#[derive(Debug, Clone)]
pub struct AddWeatherInput {
    pub city: String,
    pub temperature: Temperature,
    pub humidity: u8,
    pub condition: WeatherCondition,
}
