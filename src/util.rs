use std::{env, fs};
use std::time::Duration;
use std::path::Path;
use chrono::{Datelike, Utc};
use chrono_tz::America::New_York;
use reqwest::blocking::Client;
use crate::aocday::{AocInfo, AocInput};
use crate::YEAR;

fn read_lines(name: String) -> Result<Vec<String>, std::io::Error> {
    let mut path = env::current_dir().unwrap();
    path.set_file_name(name);

    let contents = fs::read_to_string(path)?;
    Ok(contents.lines().map(str::to_string).collect())
}

fn read_session_token() -> Result<String, anyhow::Error> {
    let mut path = env::current_dir()?;
    path.push(".session_token");

    let contents = fs::read_to_string(path)?;
    Ok(contents.trim().to_string())
}

pub fn read_input(day: u8) -> Result<AocInput, anyhow::Error> {
    let mut path = env::current_dir()?;
    path.push(format!("src/day{day}.txt"));

    let contents = if !path.exists() {
        download_input(day, path)?
    } else {
        fs::read_to_string(path)?
    };

    Ok(AocInput::new(contents.as_str()))
}

fn download_input<P: AsRef<Path>>(day: u8, path: P) -> Result<String, anyhow::Error> {
    let session_token = read_session_token()?;
    let url = format!("https://adventofcode.com/2024/day/{}/input", day);
    let input = Client::new().get(&url)
        .header("Cookie", format!("session={session_token}"))
        .header("User-Agent", "github.com/ryantenney/advent-of-code-2024-rust by ryan@10e.us")
        .send()
        .unwrap()
        .text()?;

    fs::write(path, &input)?;

    Ok(input)
}

pub fn end_day(year: i32) -> Option<u8> {
    let date = Utc::now().with_timezone(&New_York);
    if date.year() == year && date.month() == 12 {
        Some(date.day() as u8)
    } else {
        None
    }
}
