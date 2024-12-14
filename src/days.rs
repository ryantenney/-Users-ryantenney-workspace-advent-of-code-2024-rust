use crate::aocday::AocDay;
use crate::day1::Day1;
use crate::day2::Day2;
use crate::day3::Day3;
use crate::day4::Day4;
use crate::day5::Day5;
use crate::day6::Day6;
use crate::day7::Day7;
use crate::day8::Day8;
use crate::day9::Day9;
use crate::day11::Day11;
use crate::day13::Day13;

pub fn build_days() -> Vec<Box<dyn AocDay>> {
    vec![
        Box::new(Day1::new()),
        Box::new(Day2::new()),
        Box::new(Day3::new()),
        Box::new(Day4::new()),
        Box::new(Day5::new()),
        Box::new(Day6::new()),
        Box::new(Day7::new()),
        Box::new(Day8::new()),
        Box::new(Day9::new()),
        Box::new(Day11::new()),
        Box::new(Day13::new()),
    ]
}
