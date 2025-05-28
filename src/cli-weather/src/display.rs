use crate::models::WeatherData;
use colored::*;
use std::collections::HashMap;
use inquire::Select;
use anyhow::Result;

pub fn interactive_weather_view(data: &WeatherData) -> Result<()> {
    let mut all_data = vec![];

    all_data.push((
        data.current.last_updated.clone(),
        data.current.temp_c,
        data.current.wind_kph,
        data.current.humidity,
        estimate_precipitation(data),
        data.current.condition.text.clone(),
        data.current.is_day != 0,
    ));

    for day in &data.forecast.forecastday {
        for hour in [&8, &13, &18] {
            if let Some(h) = day.hour.iter().find(|h| h.time.ends_with(&format!("{:02}:00", hour))) {
                all_data.push((
                    h.time.clone(),
                    h.temp_c,
                    h.wind_kph,
                    h.humidity,
                    0.0,
                    h.condition.text.clone(),
                    h.time[11..13].parse::<u8>().unwrap_or(12) < 18,
                ));
            }
        }
    }

    let options: Vec<String> = all_data
        .iter()
        .map(|entry| format!("{} - {}", entry.0, entry.5))
        .collect();

    let choice = Select::new("Choose forecast view (↑↓, ⏎ to select):", options).prompt()?;
    let index = all_data.iter().position(|entry| format!("{} - {}", entry.0, entry.5) == choice).unwrap_or(0);

    std::process::Command::new("clear").status().ok();
    println!("{:^80}\n", format!("{} ({})", data.location.name.bold().cyan(), data.location.country.bold().cyan()));

    println!("{}", "WeatherCLI".bold().yellow());
    println!("{}", all_data[index].0);
    println!("{}\n", all_data[index].5.clone());

    let art = ascii_art(&all_data[index].5, all_data[index].6);
    let temp_display = format_temperature(all_data[index].1);
    let right = format!(
        "Wind: {:.1} kph\nHumidity: {}%\nPrecipitation: {} mm",
        all_data[index].2,
        all_data[index].3,
        all_data[index].4
    );

    for i in 0..temp_display.len().max(art.len()).max(3) {
        let left = art.get(i).map_or("", |s| s);
        let temp = temp_display.get(i).map_or("", |s| s);
        let meta = right.lines().nth(i).unwrap_or("");
        println!("{:<30} {:<20} {:<20}", left, temp.bold().bright_white(), meta);
    }

    println!("\nGoodbye! ☀");
    Ok(())
}

fn estimate_precipitation(data: &WeatherData) -> f64 {
    data.forecast.forecastday
        .get(0)
        .map(|f| f.day.totalprecip_mm)
        .unwrap_or(0.0)
}

fn ascii_art(condition: &str, is_day: bool) -> Vec<&'static str> {
    let mut map: HashMap<(&str, bool), Vec<&'static str>> = HashMap::new();

    map.insert(("Sunny", true), vec!["     ☀     ", "   ☀☀☀   ", "     |     "]);
    map.insert(("Clear", false), vec!["     ☽     ", "   ☽☽☽   ", "     *     "]);
    map.insert(("Partly cloudy", true), vec!["   ☁  ☀   ", "  ☁☁☁    ", "     |     "]);
    map.insert(("Cloudy", true), vec!["  ☁☁☁   ", " ☁☁☁☁  ", "  ☁☁☁   "]);
    map.insert(("Overcast", true), vec!["  ▓▓▓▓  ", " ▓▓▓▓▓ ", "  ▓▓▓▓  "]);
    map.insert(("Overcast", false), vec!["  ▓▓▓▓  ", " ▓▓▓▓▓ ", "  ▓▓▓▓  "]);
    map.insert(("Light rain", true), vec!["   ☁☁   ", "  ☁☁☁  ", "    / /  "]);
    map.insert(("Light rain", false), vec!["   ☁☁   ", "  ☁☁☁  ", "    / /  "]);

    map.get(&(condition, is_day))
        .or_else(|| map.get(&(condition, !is_day)))
        .or_else(|| map.get(&(condition, true)))
        .or_else(|| map.get(&(condition, false)))
        .cloned()
        .unwrap_or_else(|| vec!["[no art]", "<unknown condition>"])
}

fn format_temperature(temp: f64) -> Vec<String> {
    vec![
        format!("    {:.1}°C    ", temp),
        "             ".to_string(),
        "             ".to_string(),
    ]
}