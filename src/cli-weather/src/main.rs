mod api;
mod filter;
mod models;
mod db;
mod cli;
mod display;

use dotenv::dotenv;
use std::env;
use db::MongoDb;
use cli::{get_user_input, Mode};
use display::display_weather;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let api_key = env::var("WEATHER_API_KEY")?;
    let mongo_url = env::var("MONGO_URL")?;

    let db = MongoDb::new(&mongo_url, "weather_app", "weather_data").await?;

    let times = db.get_all_entry_times().await?;

    let user_input = get_user_input(Some(times)).unwrap();

    match user_input.mode {
    Mode::CurrentWeather => {
        let data = api::fetch_weather_from_api(&api_key, &user_input.city).await?;
        db.insert_if_new(&data).await?;
        println!("🌤 Placeholder: Fetched and stored weather for {}", data.location.name);
        display_weather(&data);
    }

    Mode::DatabaseQuery => {
        if let Some(ref time) = user_input.selected_time {
            if let Some(ref data) = db.get_by_location_and_time(&user_input.city, time).await? {
                println!("📜 Showing DB entry for {} at {}", user_input.city, time);
                display_weather(data);
            } else {
                println!("❌ No weather data found for that city and time.");
            }
        }
    }
}


    Ok(())
}
