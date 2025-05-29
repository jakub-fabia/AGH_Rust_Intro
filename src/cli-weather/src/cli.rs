use inquire::{Select, Text};
use crate::models::*;
use colored::*;
use anyhow::Result;
use crate::image::match_image;

pub fn get_source_and_city() -> anyhow::Result<CliInput> {
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

pub fn interactive_weather_view(data: &WeatherData) -> Result<()> {
    let mut daily_groups: Vec<(String, Vec<(String, String)>)> = Vec::new();

    for (i, day) in data.forecast.forecastday.iter().enumerate() {
        let day_label = if i == 0 {
            "Today".to_string()
        } else {
            day.date.clone()
        };

        let mut hours = Vec::new();
        for target_hour in [&2, &6, &10, &14, &18, &22] {
            if let Some(h) = day.hour.iter().find(|h| h.time.ends_with(&format!("{:02}:00", target_hour))) {
                hours.push((h.time.clone(), h.condition.text.clone()));
            }
        }

        if !hours.is_empty() {
            daily_groups.push((day_label, hours));
        }
    }

    loop {
        let day_choices: Vec<String> = daily_groups.iter().map(|(d, _)| d.clone()).chain(["Exit".to_string()]).collect();
        let day_choice = Select::new("Choose day:", day_choices).prompt()?;
        if day_choice == "Exit" {
            println!("\nGoodbye! ☀");
            break;
        }

        let hours = daily_groups.iter().find(|(d, _)| *d == day_choice).unwrap().1.clone();
        let hour_labels: Vec<String> = hours.iter().map(|(t, cond)| format!("{} - {}", t, cond)).chain(["Back".to_string()]).collect();
        let hour_choice = Select::new("Choose hour:", hour_labels).prompt()?;
        if hour_choice == "Back" {
            continue;
        }

        let (selected_time, _) = hours.iter().find(|(t, c)| format!("{} - {}", t, c) == hour_choice).unwrap();

        let entry = data.forecast.forecastday
            .iter()
            .flat_map(|d| &d.hour)
            .find(|h| &h.time == selected_time)
            .unwrap();

        std::process::Command::new("clear").status().ok();
        println!("{:^80}\n", format!("{} ({})", data.location.name.bold().cyan(), data.location.country.bold().cyan()));
        println!("{}", "WeatherCLI".bold().yellow());
        println!("{}\n", entry.time);
        println!("{}\n", entry.condition.text);

        let image = match_image(&entry.condition.text, entry.is_day == 1);
        let temp_display = format_temperature(entry.temp_c);
        let pm2_5_display = display_reading(entry.air_quality.pm2_5);
        let pm10_display  = display_reading(entry.air_quality.pm10);

        let right = format!(
            "Wind: {:.1} kph\n\
            Humidity: {}%\n\
            Chance of rain: {}%\n\
            PM2.5: {}\n\
            PM10: {}",
            entry.wind_kph,
            entry.humidity,
            entry.chance_of_rain,
            pm2_5_display,
            pm10_display,
        );

        let right_lines = right.lines().count();
        for i in 0..temp_display.len().max(image.len()).max(right_lines) {
            let left = image.get(i).map_or("", |s| s);
            let temp = temp_display.get(i).map_or("", |s| s);
            let meta = right.lines().nth(i).unwrap_or("");
            println!("{:<30} {:<20} {:<20}", left, temp.bold().bright_white(), meta);
        }

        println!("\nPress Enter to choose again...");
        let _ = std::io::stdin().read_line(&mut String::new());
    }

    Ok(())
}

fn format_temperature(temp: f64) -> Vec<String> {
    vec![
        format!("    {:.1}°C    ", temp),
        "             ".to_string(),
        "             ".to_string(),
    ]
}

fn display_reading(value: Option<f64>) -> String {
    match value {
        Some(v) => format!("{:.1} µg/m³", v),
        None    => format!("no data"),
    }
}
