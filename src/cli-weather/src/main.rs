mod api;
mod filter;
mod models;
mod db;

use dotenv::dotenv;
use std::env;
use db::MongoDb;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("WEATHER_API_KEY").expect("Brak WEATHER_API_KEY");
    let db = MongoDb::new("mongodb://db:27017", "weather_app", "weather_data")
        .await
        .expect("Failed to connect to MongoDB");

    let city = "Cracow"; // później zmienimy na parametr CLI

    match api::fetch_weather_from_api(&api_key, city).await {
        Ok(data) => {
            println!("🌤 Pogoda teraz: {}°C, {}", data.current.temp_c, data.current.condition.text);
            println!("💨 PM2.5: {}, PM10: {}", data.current.air_quality.pm2_5, data.current.air_quality.pm10);

            if let Err(e) = db.insert_if_new(&data).await {
                eprintln!("❌ Błąd podczas zapisywania do bazy: {}", e);
            } else {
                println!("✅ Dane pogodowe zapisane do MongoDB.");
            }
        }
        Err(e) => eprintln!("❌ Błąd podczas pobierania danych pogodowych: {e}"),
    }
}
