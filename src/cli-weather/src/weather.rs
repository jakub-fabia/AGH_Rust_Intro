use reqwest::blocking::get;
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::io::Read;

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
    pub wind_dir: String,
    pub humidity: u8,
    pub condition: Condition,
    pub air_quality: AirQuality,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Condition {
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AirQuality {
    #[serde(rename = "pm2_5")]
    pub pm2_5: f64,
    #[serde(rename = "pm10")]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct HourData {
    pub time: String,
    pub temp_c: f64,
    pub wind_kph: f64,
    pub humidity: u8,
    pub chance_of_rain: u8,
    pub condition: Condition,
}

pub fn fetch_weather(api_key: &str, city: &str) -> Result<WeatherData, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.weatherapi.com/v1/forecast.json?key={}&q={}&days=3&aqi=yes&lang=pl",
        api_key, city
    );

    let resp = get(&url)?.json::<WeatherData>()?;

    if let Ok(json_str) = serde_json::to_string_pretty(&resp) {
        let _ = File::create("weather_dump.json").and_then(|mut f| f.write_all(json_str.as_bytes()));
    }
    Ok(resp)

    // let mut file = File::open("weather_dump.json")?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;

    // let data: WeatherData = serde_json::from_str(&contents)?;

    // Ok(data)
}
