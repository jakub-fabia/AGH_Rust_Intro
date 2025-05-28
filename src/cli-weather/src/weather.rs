use reqwest::blocking::get;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WeatherData {
    pub location: Location,
    pub current: Current,
    pub forecast: Forecast,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub name: String,
    pub country: String,
}

#[derive(Debug, Deserialize)]
pub struct Current {
    pub last_updated: String,
    pub temp_c: f64,
    pub feelslike_c: f64,
    pub is_day: u8,
    pub wind_kph: f64,
    pub wind_dir: String,
    pub humidity: u8,
    pub condition: Condition,
    pub air_quality: AirQuality,
}

#[derive(Debug, Deserialize)]
pub struct Condition {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct AirQuality {
    #[serde(rename = "pm2_5")]
    pub pm2_5: f64,
    #[serde(rename = "pm10")]
    pub pm10: f64,
}

#[derive(Debug, Deserialize)]
pub struct Forecast {
    pub forecastday: Vec<ForecastDay>,
}

#[derive(Debug, Deserialize)]
pub struct ForecastDay {
    pub date: String,
    pub day: Day,
    pub hour: Vec<HourData>,
}

#[derive(Debug, Deserialize)]
pub struct Day {
    pub avgtemp_c: f64,
    pub condition: Condition,
}

#[derive(Debug, Deserialize)]
pub struct HourData {
    pub time: String,
    pub temp_c: f64,
    pub condition: Condition,
}

pub fn fetch_weather(api_key: &str, city: &str) -> Result<WeatherData, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.weatherapi.com/v1/forecast.json?key={}&q={}&days=3&aqi=yes&lang=pl",
        api_key, city
    );

    let resp = get(&url)?.json::<WeatherData>()?;
    Ok(resp)
}
