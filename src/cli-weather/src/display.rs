use crate::models::WeatherData;
use colored::*;
use std::collections::HashMap;

pub fn display_weather(data: &WeatherData) {
    let current = &data.current;
    let location = &data.location;

    // Top-center: City and Country
    println!("{:^80}\n", format!("{} ({})", location.name.bold().cyan(), location.country.bold().cyan()));

    // Top-left: WeatherCLI, date, time, condition
    println!("{}", "WeatherCLI".bold().yellow());
    println!("{}", current.last_updated);
    println!("{}\n", current.condition.text.clone());

    // Left-side ASCII art (based on condition)
    let art = ascii_art(&current.condition.text, current.is_day != 0);
    let temp_display = format_temperature(current.temp_c);

    // Right-side details
    let right = format!(
        "Wind: {:.1} kph\nHumidity: {}%\nPrecipitation: {} mm",
        current.wind_kph,
        current.humidity,
        estimate_precipitation(&data)
    );

    for i in 0..temp_display.len().max(art.len()).max(3) {
        let left = art.get(i).map_or("", |s| s);
        let temp = temp_display.get(i).map_or("", |s| s);
        let meta = right.lines().nth(i).unwrap_or("");
        println!("{:<30} {:<20} {:<20}", left, temp.bold().bright_white(), meta);
    }
}

fn estimate_precipitation(data: &WeatherData) -> f64 {
    // fallback approximation from forecast (could be improved)
    data.forecast.forecastday
        .get(0)
        .map(|f| f.day.totalprecip_mm)
        .unwrap_or(0.0)
}

fn ascii_art(condition: &str, is_day: bool) -> Vec<&'static str> {
    let mut map: HashMap<(&str, bool), Vec<&'static str>> = HashMap::new();

    map.insert(("Sunny", true), vec![
        "     ☀     ",
        "   ☀☀☀   ",
        "     |     ",
    ]);

    map.insert(("Clear", false), vec![
        "     ☽     ",
        "   ☽☽☽   ",
        "     *     ",
    ]);

    map.insert(("Partly cloudy", true), vec![
        "   ☁  ☀   ",
        "  ☁☁☁    ",
        "     |     ",
    ]);

    map.insert(("Cloudy", true), vec![
        "  ☁☁☁   ",
        " ☁☁☁☁  ",
        "  ☁☁☁   ",
    ]);

    map.insert(("Overcast", true), vec![
        "  ▓▓▓▓  ",
        " ▓▓▓▓▓ ",
        "  ▓▓▓▓  ",
    ]);
    map.insert(("Overcast", false), vec![
        "  ▓▓▓▓  ",
        " ▓▓▓▓▓ ",
        "  ▓▓▓▓  ",
    ]);

    map.insert(("Light rain", true), vec![
        "   ☁☁   ",
        "  ☁☁☁  ",
        "    / /  ",
    ]);
    map.insert(("Light rain", false), vec![
        "   ☁☁   ",
        "  ☁☁☁  ",
        "    / /  ",
    ]);

    map.get(&(condition, is_day))
        .or_else(|| map.get(&(condition, !is_day)))
        .or_else(|| map.get(&(condition, true)))
        .or_else(|| map.get(&(condition, false)))
        .cloned()
        .unwrap_or_else(|| vec!["[no art]", "<unknown condition>"])
}

fn format_temperature(temp: f64) -> Vec<String> {
    // Big text display, simplistic approach
    let lines = vec![
        format!("    {:.1}°C    ", temp),
        "             ".to_string(),
        "             ".to_string(),
    ];
    lines
}
