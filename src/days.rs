use crate::aocday::AocDay;
use crate::day1;
use crate::day2;
use crate::day3;
use crate::day4;
use crate::day5;
use crate::day6;
use crate::day7;
use crate::day8;

pub fn build_days() -> Vec<Box<dyn AocDay>> {
    vec![
        Box::new(day1::Day1::new()),
        Box::new(day2::Day2::new()),
        Box::new(day3::Day3::new()),
        Box::new(day4::Day4::new()),
        Box::new(day5::Day5::new()),
        Box::new(day6::Day6::new()),
        Box::new(day7::Day7::new()),
        Box::new(day8::Day8::new()),
    ]
}
