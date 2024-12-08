use std::panic::resume_unwind;
use anyhow::{anyhow, Error};
use itertools::Itertools;
use crate::aocday::{AocDay, AocInfo, AocInput, AocOutput};
use crate::aocday::AocOutput::Unimplemented;

#[derive(Default)]
pub struct Day7 {
    input: Vec<TestData>,
}

type Today = Day7;

impl Today {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

}

impl AocDay for Today {

    fn info(&self) -> AocInfo {
        (7, "Bridge Repair").into()
    }

    fn init(&mut self, input: AocInput) -> Result<(), Error> {
        self.input = input.lines().iter().map(|line| {
            let output = line.split(": ").take(2).collect_vec();
            let result = output[0].parse::<u64>().unwrap();
            let values = output[1].split(" ").filter_map(|s| s.parse::<u64>().ok()).collect_vec();
            TestData { result, values }
        })
        .collect();

        Ok(())
    }

    fn part1(&self) -> Result<AocOutput, Error> {
        let mut sum = 0u64;

        for testdata in self.input.iter() {
            let mut values_reversed = testdata.values.clone();
            values_reversed.reverse();

            if test_number_recursive(testdata.result, &values_reversed) {
                sum += testdata.result;
            }
        }

        Ok(sum.into())
    }

    fn part2(&self) -> Result<AocOutput, Error> {
        let mut sum = 0u64;

        for testdata in self.input.iter() {
            let mut values_reversed = testdata.values.clone();
            values_reversed.reverse();

            if test_number_recursive_with_concat(testdata.result, &values_reversed) {
                sum += testdata.result;
            }
        }

        Ok(sum.into())
    }

}

fn test_number_recursive(result: u64, values: &[u64]) -> bool {
    let next_value = values[0];
    let next_values = &values[1..];

    if next_values.len() == 0 {
        next_value == result
    } else {
        if result % next_value == 0 {
            if test_number_recursive(result / next_value, next_values) {
                return true;
            }
        }
        if result >= next_value {
            if test_number_recursive(result - next_value, next_values) {
                return true;
            }
        }
        false
    }
}

fn test_number_recursive_with_concat(result: u64, values: &[u64]) -> bool {
    let next_value = values[0];
    let next_values = &values[1..];

    if next_values.len() == 0 {
        next_value == result
    } else {
        if result % next_value == 0 {
            if test_number_recursive_with_concat(result / next_value, next_values) {
                return true;
            }
        }
        if result >= next_value {
            if test_number_recursive_with_concat(result - next_value, next_values) {
                return true;
            }
        }

        let result_str = result.to_string();
        let next_value_str = next_value.to_string();
        if result_str.ends_with(&next_value_str) {
            if let Ok(new_result) = result_str[0..result_str.len() - next_value_str.len()].parse::<u64>() {
                if test_number_recursive_with_concat(new_result, next_values) {
                    return true;
                }
            }
        }
        false
    }
}


struct TestData {
    result: u64,
    values: Vec<u64>,
}

#[cfg(test)]
mod tests {

    use super::*;

    const EX: &str = "190: 10 19
                      3267: 81 40 27
                      83: 17 5
                      156: 15 6
                      7290: 6 8 6 15
                      161011: 16 10 13
                      192: 17 8 14
                      21037: 9 7 18 13
                      292: 11 6 16 20";

    #[test]
    fn example() {
        let day = init(EX);
        assert_eq!(day.part1().expect("Part 1"), 3749u64.into());
        assert_eq!(day.part2().expect("Part 2"), 11387u64.into());
    }

    #[test]
    fn individual_test_cases_part1() {
        assert!(test_number_recursive(190, &[19u64, 10u64]));
    }

    #[test]
    fn individual_test_cases_part2() {
        //assert!(test_number_recursive_with_concat(156, &[6u64, 15u64]));
        //assert!(test_number_recursive_with_concat(7290, &[15u64, 6u64, 8u64, 6u64]));
        //assert!(test_number_recursive_with_concat(192, &[14u64, 8u64, 17u64]));
    }

    fn init(input: &str) -> Today {
        let mut day = Today::new();
        day.init(AocInput::new(input).trim()).expect("Init failed");
        day
    }

}
