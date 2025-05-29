mod api;
mod filter;
mod models;
mod db;
mod cli;
mod image;

use dotenv::dotenv;
use std::env;
use db::MongoDb;
use models::Mode;
use cli::{get_source_and_city, interactive_weather_view};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let api_key = env::var("WEATHER_API_KEY")?;
    let mongo_url = env::var("MONGO_URL")?;

    let db = MongoDb::new(&mongo_url, "weather_app", "weather_data").await?;

    loop {
        let user_input = get_source_and_city()?;

        match user_input.mode {
            Mode::CurrentWeather => {
                let data = api::fetch_weather_from_api(&api_key, &user_input.city).await?;
                db.insert_if_new(&data).await?;
                println!("🌤  Fetched and stored weather for {}", data.location.name);
                let _ = interactive_weather_view(&data);
            }

            Mode::DatabaseQuery => {
                let times = db.get_all_entry_times_for_city(&user_input.city).await?;

                if times.is_empty() {
                    println!("❌ No weather data found for '{}'.", user_input.city);
                } else {
                    let time = inquire::Select::new("Choose entry timestamp:", times).prompt()?;
                    if let Some(ref data) = db.get_by_location_and_time(&user_input.city, &time).await? {
                        println!("📜 Showing DB entry for {} at {}", user_input.city, time);
                        let _ = interactive_weather_view(data);
                    } else {
                        println!("❌ Entry not found for {} at {}", user_input.city, time);
                    }
                }
            }

            Mode::Quit => {
                println!("Goodbye! ☀");
                break;
            }
        }
    }

    Ok(())
}