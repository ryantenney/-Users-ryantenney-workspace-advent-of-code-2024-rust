use std::any::TypeId;
use std::cell::OnceCell;
use std::cmp::min;
use anyhow::{anyhow, Error};
use itertools::Itertools;
use crate::aocday::{AocDay, AocInfo, AocInput, AocOutput};
use crate::aocday::AocOutput::Unimplemented;

#[derive(Default)]
pub struct Day3 {
    parser: OnceCell<Parser>,
}

type Today = Day3;

impl Today {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

}

impl AocDay for Today {

    fn info(&self) -> AocInfo {
        (3, "Mull It Over").into()
    }

    fn init(&mut self, input: AocInput) -> Result<(), Error> {
        self.parser.set(Parser::new(input.raw().as_str(), vec!["mul(", "do()", "don't()"])).expect("TODO: panic message");

        Ok(())
    }

    fn part1(&self) -> Result<AocOutput, Error> {
        Ok(solve(self.parser.get().unwrap().clone(), false).into())
    }

    fn part2(&self) -> Result<AocOutput, Error> {
        Ok(solve(self.parser.get().unwrap().clone(), true).into())
    }

}

fn solve(mut parser: Parser, respect_do_dont: bool) -> u32 {
    parser.reset();
    let mut sum = 0;
    let mut enabled = true;
    while let Some((_, str)) = parser.next() {
        match str.as_str() {
            "do()" => enabled = true,
            "don't()" => if respect_do_dont {
                enabled = false;
            },
            "mul(" => {
                if enabled {
                    if let Some(lhs) = parser.expect_digits() {
                        if parser.expect(",") {
                            if let Some(rhs) = parser.expect_digits() {
                                if parser.expect(")") {
                                    sum += lhs * rhs;
                                }
                            }
                        }
                    }
                }
            },
            &_ => unreachable!(),
        }
    }
    sum
}

#[derive(Clone, Debug)]
struct Parser {
    input: String,
    input_len: usize,
    matches: Vec<(usize, String)>,
    match_count: usize,
    match_index: usize,
    str_pos: usize,
}

impl Parser {
    pub fn new(input: &str, needles: Vec<&str>) -> Self {
        let matches: Vec<(usize, String)> = needles.iter()
            .flat_map(|needle| {
                input.match_indices(needle)
                    .map(|(index, str)| (index + needle.len(), str.to_string()))
            })
            .sorted_by_key(|f| f.0)
            .collect();

        Parser {
            input: input.to_string(),
            input_len: input.len(),
            matches: matches.clone(),
            match_count: matches.len(),
            match_index: 0,
            str_pos: 0,
        }
    }

    pub fn next(&mut self) -> Option<(usize, String)> {
        if let Some(next) = self.matches.get(self.match_index) {
            self.match_index += 1;
            self.str_pos = next.0;
            Some(next.clone())
        } else {
            None
        }
    }

    pub fn expect(&mut self, needle: &str) -> bool {
        let str_pos = self.str_pos;
        let end_pos = str_pos + needle.len();
        if needle.eq(&self.input[str_pos..end_pos]) {
            self.str_pos = end_pos;
            true
        } else {
            false
        }
    }

    pub fn expect_digits(&mut self) -> Option<u32> {
        let str_pos = self.str_pos;
        let mut end_pos = str_pos;
        let input = &self.input;
        for pos in str_pos..self.input_len {
            let next_char = &input[pos..pos+1].chars().next().unwrap();
            if next_char.is_digit(10) {
                end_pos = pos + 1;
            } else {
                break;
            }
        }

        if str_pos == end_pos {
            None
        } else {
            self.str_pos = end_pos;
            input[str_pos..end_pos].parse().ok()
        }
    }

    pub fn reset(&mut self) {
        self.str_pos = 0;
        self.match_index = 0;
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    const EX1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EX2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part1() {
        let day = init(EX1);
        assert_eq!(day.part1().expect("Part 1"), 161.into());
    }

    #[test]
    fn part2() {
        let day = init(EX2);
        assert_eq!(day.part2().expect("Part 2"), 48.into());
    }

    fn init(input: &str) -> Today {
        let mut day = Today::new();
        day.init(AocInput::new(input)).expect("Init failed");
        day
    }

}
