use crate::models::WeatherData;
use colored::*;
use inquire::Select;
use anyhow::Result;

pub fn interactive_weather_view(data: &WeatherData) -> Result<()> {
    let mut all_data = Vec::new();

    all_data.push((
        data.current.last_updated.clone(),
        data.current.temp_c,
        data.current.wind_kph,
        data.current.humidity,
        data.current.precip_mm,
        data.current.condition.text.clone(),
        data.current.is_day != 0,
        data.current.air_quality.pm2_5,
        data.current.air_quality.pm10,
        true,
    ));

    // Forecast entries (08:00, 13:00, 18:00) with chance_of_rain and placeholders for air quality
    for day in &data.forecast.forecastday {
        for hour in [&8, &13, &18] {
            if let Some(h) = day.hour.iter().find(|h| h.time.ends_with(&format!("{:02}:00", hour))) {
                all_data.push((
                    h.time.clone(),
                    h.temp_c,
                    h.wind_kph,
                    h.humidity,
                    h.chance_of_rain as f64,
                    h.condition.text.clone(),
                    h.time[11..13].parse::<u8>().unwrap_or(12) < 18,
                    h.air_quality.pm2_5,
                    h.air_quality.pm10,
                    false,
                ));
            }
        }
    }

    let mut labels: Vec<String> = all_data
        .iter()
        .map(|entry| format!("{} - {}", entry.0, entry.5))
        .collect();
    labels.push("Exit".to_string());

    loop {
        let choice = Select::new("Choose forecast view (↑↓, ⏎ to select):", labels.clone()).prompt()?;
        if choice == "Exit" {
            // Exit loop
            println!("\nGoodbye! ☀");
            break;
        }

        let index = all_data
            .iter()
            .position(|entry| format!("{} - {}", entry.0, entry.5) == choice)
            .unwrap_or(0);

        std::process::Command::new("clear").status().ok();
        println!("{:^80}\n", format!("{} ({})", data.location.name.bold().cyan(), data.location.country.bold().cyan()));

        println!("{}", "WeatherCLI".bold().yellow());
        println!("{}", all_data[index].0);
        println!("{}\n", all_data[index].5.clone());

        let art = ascii_art(&all_data[index].5, all_data[index].6);
        let temp_display = format_temperature(all_data[index].1);

        let precip_label = if all_data[index].9 {
            format!("Precipitation: {:.1} mm", all_data[index].4)
        } else {
            format!("Chance of rain: {}%", all_data[index].4 as u8)
        };

        let right = format!(
            "Wind: {:.1} kph\nHumidity: {}%\n{}\nPM2.5: {:.1} µg/m³\nPM10: {:.1} µg/m³",
            all_data[index].2,
            all_data[index].3,
            precip_label,
            all_data[index].7,
            all_data[index].8
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
            "   🌙🌙🌙   ",
            "    ~~~    ",
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
