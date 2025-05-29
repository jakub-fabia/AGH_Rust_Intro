use crate::models::WeatherData;
use colored::*;
use inquire::Select;
use anyhow::Result;

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

        let art = ascii_art(&entry.condition.text, entry.is_day == 1);
        let temp_display = format_temperature(entry.temp_c);

        let right = format!(
            "Wind: {:.1} kph\nHumidity: {}%\nChance of rain: {}%\nPM2.5: {:.1} µg/m³\nPM10: {:.1} µg/m³",
            entry.wind_kph,
            entry.humidity,
            entry.chance_of_rain,
            entry.air_quality.pm2_5,
            entry.air_quality.pm10
        );

        let right_lines = right.lines().count();
        for i in 0..temp_display.len().max(art.len()).max(right_lines) {
            let left = art.get(i).map_or("", |s| s);
            let temp = temp_display.get(i).map_or("", |s| s);
            let meta = right.lines().nth(i).unwrap_or("");
            println!("{:<30} {:<20} {:<20}", left, temp.bold().bright_white(), meta);
        }

        println!("\nPress Enter to choose again...");
        let _ = std::io::stdin().read_line(&mut String::new());
    }

    Ok(())
}

fn ascii_art(condition: &str, is_day: bool) -> Vec<&'static str> {
    match (condition.trim().to_lowercase().as_str(), is_day) {
        ("sunny", true) => vec![
            "   \\ | /   ",
            "    ☀️     ",
            "   / | \\   ",
        ],
        ("clear", false) => vec![
            "     🌙     ",
            "   🌙🌙🌙  ",
            "    ~~~     ",
        ],
        ("partly cloudy", true) => vec![
            "    ☁☀☁    ",
            "   ☀☁☀   ",
            "   \\☁☀/   ",
        ],
        ("partly cloudy", false) => vec![
            "    ☁🌙☁    ",
            "   🌙☁🌙   ",
            "   /🌙☁\\   ",
        ],
        ("cloudy", true) => vec![
            "    ☁☁☁    ",
            "   ☁☁☁☁   ",
            "   ────   ",
        ],
        ("cloudy", false) => vec![
            "    ☁☁☁    ",
            "   ☁☁☁☁   ",
            "   ────   ",
        ],
        ("overcast", true) => vec![
            "   ☁☁☁☁   ",
            "  ☁☁☁☁☁  ",
            "   ~~~~~   ",
        ],    
        ("overcast", false) => vec![
            "   ☁☁☁☁   ",
            "  ☁☁☁☁☁  ",
            "   ~~~~~   ",
        ],
        ("mist", true) => vec![
            "   ~~~~~   ",
            "  ~~~~~~~  ",
            "   .....   ",
        ],    
        ("mist", false) => vec![
            "   ~~~~~   ",
            "  ~~~~~~~  ",
            "   .....   ",
        ],    
        ("patchy rain possible", true) => vec![
            "    ☀☁🌧    ",
            "    🌧🌧🌧    ",
            "   ~~~~~   ",
        ],
        ("patchy rain possible", false) => vec![
            "    🌙☁🌧    ",
            "    🌧🌧🌧    ",
            "   ~~~~~   ",
        ],
        ("fog", true) => vec![
            "    ☁     ",
            "   ~~~~~   ",
            "  ~~~~~~~  ",
        ],    
        ("fog", false) => vec![
            "    🌙     ",
            "   ~~~~~   ",
            "  ~~~~~~~  ",
        ],
        ("patchy light drizzle", true) => vec![
            "    ☁💧☁    ",
            "    💧💧💧    ",
            "   . . .   ",
        ],    
        ("patchy light drizzle", false) => vec![
            "    ☁💧☁    ",
            "    💧💧💧    ",
            "   . . .   ",
        ],
        ("light drizzle", true) => vec![
            "    ☁💧☁    ",
            "    💧💧💧    ",
            "   .....   ",
        ],    
        ("light drizzle", false) => vec![
            "    ☁💧☁    ",
            "    💧💧💧    ",
            "   .....   ",
        ],    
        ("patchy light rain", true) => vec![
            "    ☀☁🌦    ",
            "    🌦💧🌦    ",
            "   . . .   ",
        ],    
        ("patchy light rain", false) => vec![
            "    🌙☁🌦    ",
            "    🌦💧🌦    ",
            "   . . .   ",
        ],
        ("light rain", true) => vec![
            "    ☁☔☁    ",
            "    ☔☔☔    ",
            "   . . .   ",
        ],    
        ("light rain", false) => vec![
            "    🌙☔☁    ",
            "    ☔☔☔    ",
            "   . . .   ",
        ],
        ("moderate rain at times", true) => vec![
            "    ☀☁🌧    ",
            "    🌧🌧🌧    ",
            "   ~~~~~   ",
        ],    
        ("moderate rain at times", false) => vec![
            "    🌙☁🌧    ",
            "    🌧🌧🌧    ",
            "   ~~~~~   ",
        ],
        ("moderate rain", true) => vec![
            "    ☁🌧☁    ",
            "    🌧☔🌧    ",
            "   :::::   ",
        ],    
        ("moderate rain", false) => vec![
            "    🌙☔☁    ",
            "    🌧☔🌧    ",
            "   :::::   ",
        ],
        ("heavy rain at times", true) => vec![
            "    ☀☁⛈️    ",
            "    ⛈️⛈️⛈️    ",
            "   /////   ",
        ],    
        ("heavy rain at times", false) => vec![
            "    🌙☁⛈️    ",
            "    ⛈️⛈️⛈️    ",
            "   /////   ",
        ],
        ("heavy rain", true) => vec![
            "    ☁⛈️☁    ",
            "    ⛈️⛈️⛈️    ",
            "   ≡≡≡≡≡   ",
        ],    
        ("heavy rain", false) => vec![
            "    🌙⛈️☁    ",
            "    ⛈️⛈️⛈️    ",
            "   ≡≡≡≡≡   ",
        ],
        ("light rain shower", true) => vec![
            "    ☁💧☁    ",
            "  💧☔☔💧  ",
            "   .:.:.   ",
        ],    
        ("light rain shower", false) => vec![
            "    🌙💧☁    ",
            "  💧☔☔💧  ",
            "   .:.:.   ",
        ],
        ("patchy rain nearby", true) => vec![
            "   ☀☁🌧   ",
            "  🌧💧🌧  ",
            "   ~~~~~   ",
        ],    
        ("patchy rain nearby", false) => vec![
            "   🌙☁🌧   ",
            "  🌧💧🌧  ",
            "   ~~~~~   ",
        ],
        _ => vec![
            "[no art]",
            "<unknown condition>",
            "¯\\_(ツ)_/¯",
        ],
    }
}


fn format_temperature(temp: f64) -> Vec<String> {
    vec![
        format!("    {:.1}°C    ", temp),
        "             ".to_string(),
        "             ".to_string(),
    ]
}
