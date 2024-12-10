#![feature(iter_collect_into)]
#![feature(mixed_integer_ops_unsigned_sub)]
#![allow(dead_code)]
#![allow(unused_imports)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod timer;
mod util;
mod aocday;
mod days;
mod grid;
mod math;

use std::{any, env};
use std::fs;
use std::path::{Path, PathBuf};
use std::str;
use std::time::{Duration, Instant};
use anyhow::{anyhow, Error};
use regex::Regex;
use reqwest::blocking::Client;
use aocday::AocDay;
use days::build_days;
use RunMode::Unlocked;
use timer::Timer;
use util::{end_day, read_input};
use crate::aocday::{AocInfo, AocInput, AocOutput};
use crate::RunMode::{All, Single, Today};

const YEAR: i32 = 2024;
const RUN_DAY: Option<u8> = None;
const RUN_MODE: RunMode = Unlocked;
const REDACT: bool = true;

#[derive(Clone, Copy, Debug, PartialEq)]
enum RunMode {
    All,
    Today,
    Single,
    Unlocked,
}

fn main() {
    let today = end_day(YEAR);
    let mut days = build_days();

    let mut timer = Timer::new();
    for day in days.iter_mut() {
        let day_num = day.info().day();
        if RUN_MODE == Single {
            if let Some(run_day) = RUN_DAY {
                if run_day != day_num {
                    continue;
                }
            }
        } else if RUN_MODE != All {
            if let Some(today) = today {
                if (RUN_MODE == Today && day_num != today) || (RUN_MODE == Unlocked && day_num > today) {
                    continue;
                }
            }
        }

        let input = read_input(day_num);
        if input.is_err() {
            println!("Day {day_num}: input not found");
            continue;
        }

        match timer.time_with_result(|| day.init(input.unwrap())) {
            Ok((_, duration)) => {
                let (_, day_name) = day.info().into();
                if day_name.is_empty() {
                    println!("Day {day_num}");
                } else {
                    println!("Day {day_num}: {day_name}");
                }

                println!("  Init  : ({:?})", duration);
            },
            Err(e) => {
                println!("Day {day_num}: {e}");
                continue;
            },
        }

        let (part1, part1_duration) = timer.time_with_return(|| day.part1());
        print_part(1, part1, part1_duration);

        let (part2, part2_duration) = timer.time_with_return(|| day.part2());
        print_part(2, part2, part2_duration);
    }

    println!("Total: {:?}", timer.duration);
}

fn print_part(part_number: u8, result: Result<AocOutput, Error>, duration: Duration) {
    match result {
        Ok(solution) => {
            if let AocOutput::Multiline(text) = solution {
                println!("  Part {}: ({:?})\n    {}", part_number, duration, redact(text).replace('\n', "\n    "));
            } else if let AocOutput::Unimplemented = solution {
                println!("  Part {}: {}", part_number, redact(solution.to_string()));
            } else {
                println!("  Part {}: {} ({:?})", part_number, redact(solution.to_string()), duration);
            }
        },
        Err(e) => {
            println!("  Part {}: (Error)\n    {}", part_number, format!("{e:?}").replace('\n', "\n    "));
        }
    }
}

fn redact(input: String) -> String {
    if !REDACT || input == "Unimplemented" {
        return input;
    }
    Regex::new(r"[a-z0-9\.]").unwrap().replace_all(input.as_str(), "x").to_string()
}
