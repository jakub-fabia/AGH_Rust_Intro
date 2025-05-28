use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use chrono::TimeDelta;
 
const DATA_FILE : &str = "data.json";
 
const API_QUERY : &str = "https://api.openf1.org/v1/car_data?driver_number=55&session_key=9165&speed>0";
 
#[derive(Serialize, Deserialize)]
struct CarData {
    brake : u32,
    date: DateTime<Utc>,
    driver_number: u32,
    drs: u8,
    meeting_key: u32,
    n_gear: u8,
    rpm: u32,
    session_key: u32,
    speed: u32,
    throttle: u8
}
 

fn fetch_car_data(url: &str, data_file : &str) -> Result<()> {
    let data = reqwest::blocking::get(url)?.text()?;
    let car_data : Vec<CarData> = serde_json::from_str(&data)?;
    let writer = BufWriter::new(File::create(data_file)?);
    serde_json::to_writer(writer, &car_data)?;
    Ok(())
}
 
fn load_car_data_from_file(data_file : &str) -> Result<Vec<CarData>> {
    let car_data : Vec<CarData> = serde_json::from_reader(BufReader::new(File::open(data_file)?))?;
    Ok(car_data)
}
 
fn check_if_data_exists(data_file : &str) -> bool {
    Path::new(data_file).exists()
}
 
fn main() -> Result<()> {
 
    if !check_if_data_exists(DATA_FILE) {
        fetch_car_data(API_QUERY, DATA_FILE)?;
    }
 
    let car_data = load_car_data_from_file(DATA_FILE).unwrap();
    
    println!("Average speed: {:.2} km/h", average_speed(&car_data));
    println!("Total number of records: {}", car_data.len());
    println!("Total duration of high speed: {} seconds", high_speed(&car_data, 200).num_seconds());
    if let Some((max_rpm, gear)) = max_rpm_and_gear(&car_data) {
        println!("Max RPM: {}, Gear: {}", max_rpm, gear);
    } else {
        println!("No data available for max RPM and gear.");
    }
    Ok(())
}
 
fn average_speed(car_data: &[CarData]) -> f64 {
    let total_speed: u32 = car_data.iter().map(|data| data.speed).sum();

    total_speed as f64 / car_data.len() as f64
}

fn high_speed(car_data: &[CarData], threshold: u32) -> TimeDelta {
    let mut total_duration = chrono::Duration::zero();
    let mut prev: Option<&CarData> = None;

    for data in car_data.iter() {
        if data.speed > threshold {
            if let Some(prev_data) = prev {
                // Add the time difference between this and previous record
                let duration = data.date - prev_data.date;
                total_duration = total_duration + duration;
            }
            prev = Some(data);
        } else {
            prev = None;
        }
    }

    total_duration
}

    fn max_rpm_and_gear(car_data: &[CarData]) -> Option<(u32, u8)> {
        car_data
            .iter()
            .max_by_key(|data| data.rpm)
            .map(|data| (data.rpm, data.n_gear))
}