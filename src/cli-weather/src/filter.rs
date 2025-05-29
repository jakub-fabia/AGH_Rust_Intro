use crate::models::{ForecastDay, HourData, WeatherData};
use chrono::{Local, NaiveDate, NaiveDateTime, Timelike, TimeZone};
use std::vec::Vec;

// Funkcja filtrująca dane pogodowe, usuwa niepotrzebne godziny (np. z dzisiejszego dnia której już minęły) i zostawia tylko godziny wyświetlane w prognozie (8:00, 13:00, 18:00)
pub fn filter_weather_data(mut data: WeatherData) -> WeatherData {
    let now = Local::now() + chrono::Duration::hours(2);
    let today = now.date_naive();
    let current_time = now.time();

    let forecastday_filtered: Vec<ForecastDay> = data
        .forecast
        .forecastday
        .into_iter()
        .filter_map(|mut day| {
            let day_date = NaiveDate::parse_from_str(&day.date, "%Y-%m-%d").ok()?;

            if day_date < today || day_date > today + chrono::Duration::days(7) {
                return None;
            }

            let filtered_hours: Vec<HourData> = day
                .hour
                .into_iter()
                .filter(|hour| {
                    if let Ok(naive_dt) = NaiveDateTime::parse_from_str(&hour.time, "%Y-%m-%d %H:%M") {
                        if let Some(hour_time) = Local.from_local_datetime(&naive_dt).single() {
                            let hour_only = hour_time.time();
                            let is_target_hour = [2, 6, 10, 14, 18, 22].contains(&hour_only.hour());

                            if day_date == today {
                                is_target_hour && hour_only > current_time
                            } else {
                                is_target_hour
                            }
                        } else {
                            false
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



#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;

    fn make_hour(time: &str) -> HourData {
        HourData {
            time: time.to_string(),
            temp_c: 0.0,
            wind_kph: 0.0,
            humidity: 0,
            chance_of_rain: 0,
            condition: Condition { text: "Clear".into() },
            air_quality: AirQuality { pm2_5: 0.0, pm10: 0.0 },
        }
    }

    fn make_weather_data(date: &str, hours: Vec<HourData>) -> WeatherData {
        WeatherData {
            location: Location { name: "X".into(), country: "Y".into() },
            current: Current {
                last_updated: format!("{} 00:00", date),
                temp_c: 0.0, wind_kph: 0.0, humidity: 0, precip_mm: 0.0,
                condition: Condition { text: "N/A".into() },
                is_day: 1,
                air_quality: AirQuality { pm2_5: 0.0, pm10: 0.0 },
            },
            forecast: Forecast {
                forecastday: vec![
                    ForecastDay {
                        date: date.to_string(),
                        hour: hours,
                    }
                ],
            },
        }
    }

    #[test]
    fn future_day_keeps_only_certain_hours() {
        let tomorrow = (Local::now().date_naive() + chrono::Duration::days(1)).format("%Y-%m-%d").to_string();
        let all_hours = vec![
            make_hour(&format!("{} 00:00", tomorrow)),
            make_hour(&format!("{} 01:00", tomorrow)),
            make_hour(&format!("{} 02:00", tomorrow)),
            make_hour(&format!("{} 03:00", tomorrow)),
            make_hour(&format!("{} 04:00", tomorrow)),
            make_hour(&format!("{} 05:00", tomorrow)),
            make_hour(&format!("{} 06:00", tomorrow)),
            make_hour(&format!("{} 07:00", tomorrow)),
            make_hour(&format!("{} 08:00", tomorrow)),
            make_hour(&format!("{} 09:00", tomorrow)),
            make_hour(&format!("{} 10:00", tomorrow)),
            make_hour(&format!("{} 11:00", tomorrow)),
            make_hour(&format!("{} 12:00", tomorrow)),
            make_hour(&format!("{} 13:00", tomorrow)),
            make_hour(&format!("{} 14:00", tomorrow)),
            make_hour(&format!("{} 15:00", tomorrow)),
            make_hour(&format!("{} 16:00", tomorrow)),
            make_hour(&format!("{} 17:00", tomorrow)),
            make_hour(&format!("{} 18:00", tomorrow)),
            make_hour(&format!("{} 19:00", tomorrow)),
            make_hour(&format!("{} 20:00", tomorrow)),
            make_hour(&format!("{} 21:00", tomorrow)),
            make_hour(&format!("{} 22:00", tomorrow)),
            make_hour(&format!("{} 23:00", tomorrow)),
        ];
        let data = make_weather_data(&tomorrow, all_hours);
        let filtered = filter_weather_data(data);
        let kept: Vec<u8> = filtered.forecast.forecastday[0]
            .hour
            .iter()
            .map(|h| h.time[11..13].parse().unwrap())
            .collect();
        assert_eq!(kept, vec![2, 6, 10, 14, 18, 22]);
    }

    #[test]
    fn past_day_is_dropped() {
        let yesterday = (Local::now().date_naive() - chrono::Duration::days(1)).format("%Y-%m-%d").to_string();
        let hours = vec![make_hour(&format!("{} 08:00", yesterday))];
        let data = make_weather_data(&yesterday, hours);
        let filtered = filter_weather_data(data);
        assert!(filtered.forecast.forecastday.is_empty());
    }
}
