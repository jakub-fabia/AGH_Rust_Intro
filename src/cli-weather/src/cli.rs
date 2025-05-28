use inquire::{Select, Text};

pub enum Mode {
    CurrentWeather,
    DatabaseQuery,
}

pub struct CliInput {
    pub mode: Mode,
    pub city: String,
}

pub fn get_user_input() -> anyhow::Result<CliInput> {
    println!("🌤   Welcome to WeatherCLI!");
    println!("This app allows you to fetch the current weather or explore saved weather data.\n");

    let mode_options = vec!["Check current weather", "Query saved weather from database"];
    let mode_choice = Select::new("What would you like to do?", mode_options).prompt()?;

    let city = Text::new("Enter city name:").prompt()?;

    Ok(CliInput {
        mode: if mode_choice == "Check current weather" {
            Mode::CurrentWeather
        } else {
            Mode::DatabaseQuery
        },
        city,
    })
}