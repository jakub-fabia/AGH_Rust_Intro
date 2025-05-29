use inquire::{Select, Text};
use crate::models::*;
use colored::*;
use anyhow::Result;
use crate::image::match_image;

/// Funkcja pobierająca dane odnośnie miasta i źródła pobierania danych pogodowych
/// Zwraca `CliInput` zawierający tryb działania aplikacji i nazwę miasta
/// Jeśli użytkownik wybierze "Quit", aplikacja zakończy działanie
pub fn get_source_and_city() -> anyhow::Result<CliInput> {
    println!("🌤   Welcome to WeatherCLI!");
    println!("This app allows you to fetch the current weather or explore saved weather data.\n");

    let mode_options = vec![
        "Check current weather",
        "Query saved weather from database",
        "Quit"
    ];
    let mode_choice = Select::new("What would you like to do?", mode_options).prompt()?;

    if mode_choice == "Quit" {
        return Ok(CliInput { mode: Mode::Quit, city: String::new() });
    }

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



/// Funkcja wyświetlająca interaktywny widok pogody
/// Przyjmuje dane pogodowe jako argument i umożliwia użytkownikowi przeglądanie prognozy
/// Urzytkownik kolejno:
/// - Wybiera dzień (dzisiaj lub kolejne dni)
/// - Wybiera godzinę (np. 2:00, 6:00, 10:00, 14:00, 18:00, 22:00) lub "Current" dla aktualnej pogody
/// Pogoda się wyświetla w formie
/*
                     Cracow (Poland)

WeatherCLI
2025-05-31 10:00

Patchy rain nearby

     ☀☁🌧                           21.1°C          Wind: 16.6 kph      
     🌧𓄼🌧                                           Humidity: 66%       
     ~~~                                            Chance of rain: 61% 
                                                    PM2.5: 19.6 µg/m³   
                                                    PM10: 22.9 µg/m³    

Press Enter to choose again...
*/
/// Użytkownik ma możliwość wyboru innego dnia lub godziny, a także powrotu do wybrania miasta i źródła danych
pub fn interactive_weather_view(data: &WeatherData) -> Result<()> {
    let mut daily_groups: Vec<(String, Vec<(String, String)>)> = Vec::new();
    // Parsowanie danych do grup dziennych do menu wyboru
    for (i, day) in data.forecast.forecastday.iter().enumerate() {
        let day_label = if i == 0 {
            "Today".to_string()
        } else {
            day.date.clone()
        };

        let mut hours = Vec::new();
        if i == 0 {
            hours.push(("Current".to_string(), data.current.condition.text.clone()));
        }

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
        // Parsowanie wyboru
        let day_choices: Vec<String> = daily_groups.iter().map(|(d, _)| d.clone())
            .chain(["Exit".to_string()]).collect();
        let day_choice = Select::new("Choose day:", day_choices).prompt()?;
        if day_choice == "Exit" {
            println!("\nGoodbye! ☀");
            break;
        }

        let hours = daily_groups.iter().find(|(d, _)| *d == day_choice).unwrap().1.clone();
        let hour_labels: Vec<String> = hours.iter()
            .map(|(t, cond)| format!("{} - {}", t, cond))
            .chain(["Back".to_string()]).collect();
        let hour_choice = Select::new("Choose hour:", hour_labels).prompt()?;
        if hour_choice == "Back" {
            continue;
        }
        
        let (selected_time, _) = hours.iter()
            .find(|(t, c)| format!("{} - {}", t, c) == hour_choice).unwrap();

        // Zbieranie danych dla wybranego czasu
        let entry = if selected_time == "Current" {
            WeatherViewEntry {
                time: data.current.last_updated.clone(),
                temp_c: data.current.temp_c,
                wind_kph: data.current.wind_kph,
                humidity: data.current.humidity,
                chance_of_rain: None,
                precip_mm: Some(data.current.precip_mm),
                condition_text: data.current.condition.text.clone(),
                is_day: data.current.is_day != 0,
                pm2_5: data.current.air_quality.pm2_5,
                pm10: data.current.air_quality.pm10,
            }
        } else {
            let hour = data.forecast.forecastday
                .iter()
                .flat_map(|d| &d.hour)
                .find(|h| &h.time == selected_time)
                .unwrap();

            WeatherViewEntry {
                time: hour.time.clone(),
                temp_c: hour.temp_c,
                wind_kph: hour.wind_kph,
                humidity: hour.humidity,
                chance_of_rain: Some(hour.chance_of_rain),
                precip_mm: None,
                condition_text: hour.condition.text.clone(),
                is_day: hour.is_day != 0,
                pm2_5: hour.air_quality.pm2_5,
                pm10: hour.air_quality.pm10,
            }
        };

        // Wyświetlanie danych pogodowych
        std::process::Command::new("clear").status().ok();
        println!("{:^80}\n", format!("{} ({})", data.location.name.bold().cyan(), data.location.country.bold().cyan()));
        println!("{}", "WeatherCLI".bold().yellow());
        println!("{}\n", entry.time);
        println!("{}\n", entry.condition_text);

        let image = match_image(&entry.condition_text, entry.is_day);
        let temp_display = format_temperature(entry.temp_c);
        let pm2_5_display = display_reading(entry.pm2_5);
        let pm10_display  = display_reading(entry.pm10);
        let rain_display = if let Some(mm) = entry.precip_mm {
            format!("Rain: {:.1} mm", mm)
        } else if let Some(chance) = entry.chance_of_rain {
            format!("Chance of rain: {}%", chance)
        } else {
            "No precipitation data".to_string()
        };
                

        let right = format!(
            "Wind: {:.1} kph\n\
            Humidity: {}%\n\
            {}\n\
            PM2.5: {}\n\
            PM10: {}",
            entry.wind_kph,
            entry.humidity,
            rain_display,
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

// Funkcje formatujące dane pogodowe do wyświetlenia
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