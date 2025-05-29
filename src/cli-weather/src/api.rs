use reqwest::Client;
use anyhow::Result;
use crate::filter;
use crate::models::*;

/// Funkcja fetchująca dane pogodowe z API
/// Wejścia: 
/// - `api_key`: klucz API do WeatherAPI
/// - `city`: nazwa miasta, dla którego chcemy pobrać dane pogodowe
/// Wyjście:
/// - `Result<WeatherData>`: zwraca przefiltrowane dane pogodowe lub błąd
/// (O filtrze zobacz w pliku `filter.rs`)
pub async fn fetch_weather_from_api(api_key: &str, city: &str) -> Result<WeatherData> {
    let url = format!("https://api.weatherapi.com/v1/forecast.json?key={}&q={}&days=7&aqi=yes&lang=en", api_key, city);
    let client = Client::new();
    let resp = client.get(&url).send().await?.json::<WeatherData>().await?;
    let filtered: WeatherData = filter::filter_weather_data(resp);
    Ok(filtered)
}