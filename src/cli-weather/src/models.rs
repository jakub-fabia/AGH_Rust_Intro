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

#[derive(Debug, Deserialize, Serialize)]
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
    pub day: Day,
    pub hour: Vec<HourData>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Day {
    pub maxtemp_c: f64,
    pub avgtemp_c: f64,
    pub mintemp_c: f64,
    pub maxwind_kph: f64,
    pub totalprecip_mm: f64,
    pub daily_chance_of_rain: u8,
    pub condition: Condition,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HourData {
    pub time: String,
    pub temp_c: f64,
    pub wind_kph: f64,
    pub humidity: u8,
    pub chance_of_rain: u8,
    pub condition: Condition,
}