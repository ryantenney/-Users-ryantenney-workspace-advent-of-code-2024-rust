use std::cmp::max;
use anyhow::Error;
use crate::aocday::{AocDay, AocInfo, AocInput, AocOutput};

#[derive(Default)]
pub struct Day2 {
    reports: Vec<Vec<i8>>,
}

type Today = Day2;

impl Today {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

}

impl AocDay for Today {

    fn info(&self) -> AocInfo {
        (2, "Red-Nosed Reports").into()
    }

    fn init(&mut self, input: AocInput) -> Result<(), Error> {
        self.reports = input.lines().iter()
            .map(|s| {
                s.split(' ')
                    .filter_map(|n| n.parse().ok())
                    .collect()
            }).collect();

        Ok(())
    }

    fn part1(&self) -> Result<AocOutput, Error> {
        Ok(self.reports.iter()
            .filter(|report| analyze_report(report))
            .count()
            .into())
    }

    fn part2(&self) -> Result<AocOutput, Error> {
        Ok(self.reports.iter()
            .filter(|report| analyze_report_lenient(report))
            .count()
            .into())
    }

}

fn analyze_report(report: &[i8]) -> bool {
    analyze_report_with_skip(report, usize::MAX)
}

fn analyze_report_lenient(report: &[i8]) -> bool {
    if !analyze_report_with_skip(report, usize::MAX) {
        if analyze_report_with_skip(&report[1..], usize::MAX) {
            return true;
        }
        for skip in 1..report.len() {
            if analyze_report_with_skip(report, skip) {
                return true;
            }
        }
        false
    } else {
        true
    }
}

fn analyze_report_with_skip(report: &[i8], skip: usize) -> bool {
    let mut monotonic = 0i8;
    let mut left = report[0];
    for (idx, right) in report.iter().enumerate().skip(1) {
        if idx == skip {
            continue;
        }
        let diff = left - right;
        if diff.abs() > 3 {
            return false;
        }
        if monotonic == 0 {
            if diff < 0 {
                monotonic = -1;
            } else if diff > 0 {
                monotonic = 1;
            } else {
                return false;
            }
        } else {
            if (monotonic == 1 && diff <= 0) || (monotonic == -1 && diff >= 0) {
                return false;
            }
        }
        left = *right;
    }
    true
}

#[cfg(test)]
mod tests {

    use super::*;

    const EX: &str = "7 6 4 2 1
                      1 2 7 8 9
                      9 7 6 2 1
                      1 3 2 4 5
                      8 6 4 4 1
                      1 3 6 7 9";

    #[test]
    fn example() {
        let day = init(EX);
        assert_eq!(day.part1().expect("Part 1"), 2.into());
        assert_eq!(day.part2().expect("Part 2"), 4.into());
    }

    fn init(input: &str) -> Today {
        let mut day = Today::new();
        day.init(AocInput::new(input).trim()).expect("Init failed");
        day
    }

}
