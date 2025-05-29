use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherData {
    pub location: Location,
    pub current: Current,
    pub forecast: Forecast,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub name: String,
    pub country: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Current {
    pub last_updated: String,
    pub temp_c: f64,
    pub is_day: u8,
    pub wind_kph: f64,
    pub precip_mm: f64,
    pub humidity: u8,
    pub condition: Condition,
    pub air_quality: AirQuality,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Condition {
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AirQuality {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pm2_5: Option<f64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pm10: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Forecast {
    pub forecastday: Vec<ForecastDay>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastDay {
    pub date: String,
    pub hour: Vec<HourData>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HourData {
    pub time: String,
    pub temp_c: f64,
    pub wind_kph: f64,
    pub humidity: u8,
    pub chance_of_rain: u8,
    pub is_day: u8,
    pub condition: Condition,
    pub air_quality: AirQuality,
}

pub enum Mode {
    CurrentWeather,
    DatabaseQuery,
}

pub struct CliInput {
    pub mode: Mode,
    pub city: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn location_serde_roundtrip() {
        let loc = Location { name: "Warsaw".into(), country: "PL".into() };
        let json = serde_json::to_string(&loc).unwrap();
        let loc2: Location = serde_json::from_str(&json).unwrap();
        assert_eq!(loc.name, loc2.name);
        assert_eq!(loc.country, loc2.country);
    }

    #[test]
    fn hourdata_serde_roundtrip() {
        let hour = HourData {
            time: "2025-05-28 13:00".into(),
            temp_c: 20.5,
            wind_kph: 5.0,
            humidity: 50,
            chance_of_rain: 10,
            condition: Condition { text: "Sunny".into() },
            air_quality: AirQuality { pm2_5: 12.3, pm10: 25.6 },
        };
        let json = serde_json::to_string(&hour).unwrap();
        let back: HourData = serde_json::from_str(&json).unwrap();
        assert_eq!(hour.time, back.time);
        assert!((hour.temp_c - back.temp_c).abs() < f64::EPSILON);
        assert_eq!(hour.humidity, back.humidity);
        assert_eq!(back.condition.text, "Sunny");
    }
}
