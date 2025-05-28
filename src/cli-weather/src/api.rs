use reqwest::Client;
use anyhow::Result;
use crate::filter;
use crate::models::*;

pub async fn fetch_weather_from_api(api_key: &str, city: &str) -> Result<WeatherData> {
    let url = format!("https://api.weatherapi.com/v1/forecast.json?key={}&q={}&days=3&aqi=yes&lang=en", api_key, city);
    let client = Client::new();
    let resp = client.get(&url).send().await?.json::<WeatherData>().await?;
    let filtered: WeatherData = filter::filter_weather_data(resp);
    Ok(filtered)
}