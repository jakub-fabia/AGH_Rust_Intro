mod weather;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let api_key = env::var("WEATHER_API_KEY").expect("Brak WEATHER_API_KEY");
    let city = "Cracow"; // później zmienimy na parametr CLI

    match weather::fetch_weather_from_file(&api_key, city) {
        Ok(data) => {
            println!("🌤 Pogoda teraz: {}°C, {}", data.current.temp_c, data.current.condition.text);
            println!("💨 PM2.5: {}, PM10: {}", data.current.air_quality.pm2_5, data.current.air_quality.pm10);
        }
        Err(e) => eprintln!("Błąd podczas pobierania danych pogodowych: {e}"),
    }
}
