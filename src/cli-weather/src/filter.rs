use crate::models::{ForecastDay, HourData, WeatherData};
use chrono::{Local, NaiveDate, Timelike, TimeZone};
use std::vec::Vec;

// Funkcja filtrująca dane pogodowe, usuwa niepotrzebne godziny (np. z dzisiejszego dnia której już minęły) i zostawia tylko godziny wyświetlane w prognozie (8:00, 13:00, 18:00)
pub fn filter_weather_data(mut data: WeatherData) -> WeatherData {
    let now = Local::now();
    let today = now.date_naive();
    let current_time = now.time();

    let forecastday_filtered: Vec<ForecastDay> = data
        .forecast
        .forecastday
        .into_iter()
        .filter_map(|mut day| {
            let day_date = NaiveDate::parse_from_str(&day.date, "%Y-%m-%d").ok()?;

            let filtered_hours: Vec<HourData> = day
                .hour
                .into_iter()
                .filter(|hour| {
                    if let Ok(hour_time) = Local.datetime_from_str(&hour.time, "%Y-%m-%d %H:%M") {
                        let h = hour_time.time().hour();
                        let m = hour_time.time().minute();
                        let is_target_hour = [8, 13, 18].contains(&h);

                        if day_date == today {
                            is_target_hour && h > current_time.hour() && m > current_time.minute()
                        } else {
                            is_target_hour
                        }
                    } else {
                        false
                    }
                })
                .collect();

            if filtered_hours.is_empty() {
                None
            } else {
                day.hour = filtered_hours;
                Some(day)
            }
        })
        .collect();

    data.forecast.forecastday = forecastday_filtered;
    data
}
