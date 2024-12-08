use std::cmp::{max, min};
use anyhow::Error;
use itertools::Itertools;
use regex::Regex;
use crate::aocday::{AocDay, AocInfo, AocInput, AocOutput};

#[derive(Default)]
pub struct Day1 {
    left: Vec<i32>,
    right: Vec<i32>,
}

type Today = Day1;

impl Today {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

}

impl AocDay for Today {

    fn info(&self) -> AocInfo {
        (1, "Historian Hysteria").into()
    }

    fn init(&mut self, input: AocInput) -> Result<(), Error> {
        let regex = Regex::new(r"\s+")?;
        let input = input.lines().iter()
            .map(|s| {
                regex.split(s)
                    .take(2)
                    .filter_map(|part| part.parse::<i32>().ok())
                    .collect_vec()
            })
            .collect_vec();

        self.left = input.iter()
            .filter_map(|v| v.get(0).copied())
            .sorted()
            .collect();

        self.right = input.iter()
            .filter_map(|v| v.get(1).copied())
            .sorted()
            .collect();

        Ok(())
    }

    fn part1(&self) -> Result<AocOutput, Error> {
        Ok(self.left.clone().into_iter()
            .zip(self.right.clone())
            .map(|(left, right)| (right - left).abs())
            .sum::<i32>()
            .into())
    }

    fn part2(&self) -> Result<AocOutput, Error> {
        let len = self.left.len();
        let mut r_pos = 0;
        let mut output = 0;

        let mut right = self.right.clone();
        right.push(i32::MAX);

        for &l_val in &self.left {
            let mut r_val = right[r_pos];
            let mut count = 0;
            while r_val <= l_val {
                if r_val == l_val {
                    count += 1;
                }
                r_pos = min(r_pos + 1, len);
                r_val = right[r_pos];
            }
            output += l_val * count;
        }

        Ok(output.into())
    }

}



#[cfg(test)]
mod tests {

    use super::*;

    const EX1: &str = "3   4
                       4   3
                       2   5
                       1   3
                       3   9
                       3   3";

    #[test]
    fn example1() {
        let day = init(EX1);

        assert_eq!(day.part1().expect("Part 1"), 11.into());
    }

    #[test]
    fn example2() {
        let day = init(EX1);

        assert_eq!(day.part2().expect("Part 2"), 31.into());
    }

    fn init(input: &str) -> Today {
        let mut day = Today::new();
        day.init(AocInput::new(input).trim()).expect("Init failed");
        day
    }

}
