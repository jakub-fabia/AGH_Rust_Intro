use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherData {
    pub location: Location,
    pub current: Current,
    pub forecast: Forecast,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub name: String,
    pub country: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Current {
    pub last_updated: String,
    pub temp_c: f64,
    pub is_day: u8,
    pub wind_kph: f64,
    pub precip_mm: f64,
    pub humidity: u8,
    pub condition: Condition,
    pub air_quality: AirQuality,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Condition {
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AirQuality {
    pub pm2_5: f64,
    pub pm10: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Forecast {
    pub forecastday: Vec<ForecastDay>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastDay {
    pub date: String,
    pub hour: Vec<HourData>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HourData {
    pub time: String,
    pub temp_c: f64,
    pub wind_kph: f64,
    pub humidity: u8,
    pub chance_of_rain: u8,
    pub condition: Condition,
    pub air_quality: AirQuality,
}