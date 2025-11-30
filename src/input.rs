use crate::weather::WeatherCondition;

#[derive(Debug, Clone)]
pub struct AddWeatherInput {
    pub city: String,
    pub temperature: i32,
    pub humidity: u8,
    pub condition: WeatherCondition,
}
