use reqwest::Client;
use std::fs::File;
use std::io::{Write, Read};
use crate::filter;
use crate::models::*;

// Funkcja diagnostyczna, która używa tylko danych z pliku zebranego z API (aby oszczędzić na liczbie zapytań do API)
pub fn fetch_weather_from_file(api_key: &str, _city: &str) -> Result<WeatherData, Box<dyn std::error::Error>> {
    let mut file = File::open("weather_dump.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let data: WeatherData = serde_json::from_str(&contents)?;
    
    Ok(data)
}

pub async fn fetch_weather_from_api(api_key: &str, city: &str) -> Result<WeatherData, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.weatherapi.com/v1/forecast.json?key={}&q={}&days=3&aqi=yes&lang=pl",
        api_key, city
    );

    let client = Client::new();
    let resp = client.get(&url).send().await?.json::<WeatherData>().await?;

    let filtered: WeatherData = filter::filter_weather_data(resp);

    if let Ok(json_str) = serde_json::to_string_pretty(&filtered) {
        let _ = std::fs::write("weather_dump.json", json_str);
    }

    Ok(filtered)
}