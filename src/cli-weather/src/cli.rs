use clap::Parser;
use inquire::{Select, Text};

/// CLI mode selector and data input
#[derive(Parser, Debug)]
#[command(name = "WeatherCLI", about = "Fetch or inspect weather data via API or database")]
pub struct Args {}

pub enum Mode {
    CurrentWeather,
    DatabaseQuery,
}

pub struct CliInput {
    pub mode: Mode,
    pub city: String,
    pub selected_time: Option<String>,
}

pub fn get_user_input(entry_times: Option<Vec<String>>) -> anyhow::Result<CliInput> {
    // Welcome message
    println!("🌤   Welcome to WeatherCLI!");
    println!("This app allows you to fetch the current weather or explore saved weather data.\n");

    // Step 1: choose mode
    let mode_options = vec!["Check current weather", "Query saved weather from database"];
    let mode_choice = Select::new("What would you like to do?", mode_options).prompt()?;

    // Step 2: input city
    let city = Text::new("Enter city name:").prompt()?;

    // Step 3: if DB mode, choose a date
    let selected_time = if mode_choice == "Query saved weather from database" {
        let times = entry_times.unwrap_or_else(|| vec!["No data available".into()]);
        let choice = Select::new("Choose a recorded time:", times).prompt()?;
        Some(choice)
    } else {
        None
    };

    Ok(CliInput {
        mode: if mode_choice == "Check current weather" {
            Mode::CurrentWeather
        } else {
            Mode::DatabaseQuery
        },
        city,
        selected_time,
    })
}
