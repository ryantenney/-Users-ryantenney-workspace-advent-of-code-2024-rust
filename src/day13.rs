use anyhow::{anyhow, Error};
use itertools::Itertools;
use regex::Regex;
use crate::aocday::{AocDay, AocInfo, AocInput, AocOutput};
use crate::aocday::AocOutput::Unimplemented;

#[derive(Default)]
pub struct Day13 {
    claw_machines: Vec<ClawMachine>,
}

type Today = Day13;

impl Today {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

}

impl AocDay for Today {

    fn info(&self) -> AocInfo {
        (13, "Claw Contraption").into()
    }

    fn init(&mut self, input: AocInput) -> Result<(), Error> {
        let regex = Regex::new(r"\d+")?;
        self.claw_machines = input.lines().into_iter()
            .filter(|s| !s.is_empty())
            .filter_map(|s| {
                regex.find_iter(s.as_str())
                    .filter_map(|m| {
                        m.as_str().parse::<isize>().ok()
                    })
                    .collect_tuple::<(_, _)>()
                    .and_then(|(x, y)| Some(XY::new(x, y)))
            })
            .tuples()
            .map(|(a, b, prize)| ClawMachine::new(a, b, prize))
            .collect_vec();

        Ok(())
    }

    fn part1(&self) -> Result<AocOutput, Error> {
        Ok(self.claw_machines.iter()
            .filter_map(|machine| machine.solve())
            .map(|solution| solution.score())
            .sum::<isize>()
            .into())
    }

    fn part2(&self) -> Result<AocOutput, Error> {
        Ok(self.claw_machines.iter()
            .map(|machine| machine.offset(10000000000000))
            .filter_map(|machine| machine.solve())
            .map(|solution| solution.score())
            .sum::<isize>()
            .into())
    }

}

#[derive(Clone, Debug)]
struct ClawMachine {
    button_a: XY,
    button_b: XY,
    prize: XY,
}

impl ClawMachine {
    fn new(button_a: XY, button_b: XY, prize: XY) -> Self {
        Self { button_a, button_b, prize }
    }

    fn offset(&self, offset: isize) -> Self {
        Self {
            prize: self.prize.offset(offset),
            .. self.clone()
        }
    }
 
    #[allow(non_snake_case)]
    fn solve(&self) -> Option<Solution> {
        let XY { x: A_x, y: A_y } = self.button_a;
        let XY { x: B_x, y: B_y } = self.button_b;
        let XY { x: X, y: Y } = self.prize;

        let det = A_x * B_y - A_y * B_x;
        let A = (X * B_y - Y * B_x) / det;
        let B = (A_x * Y - A_y * X) / det;

        if X == A * A_x + B * B_x && Y == A * A_y + B * B_y {
            Some(Solution::new(A, B))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct XY {
    x: isize,
    y: isize,
}

impl XY {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn offset(&self, offset: isize) -> Self {
        Self {
            x: self.x + offset,
            y: self.y + offset,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Solution {
    a: isize,
    b: isize,
}

impl Solution {
    fn new(a: isize, b: isize) -> Self {
        Self { a, b }
    }

    fn score(&self) -> isize {
        self.a * 3 + self.b
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const EX: &str = "Button A: X+94, Y+34
                      Button B: X+22, Y+67
                      Prize: X=8400, Y=5400

                      Button A: X+26, Y+66
                      Button B: X+67, Y+21
                      Prize: X=12748, Y=12176

                      Button A: X+17, Y+86
                      Button B: X+84, Y+37
                      Prize: X=7870, Y=6450

                      Button A: X+69, Y+23
                      Button B: X+27, Y+71
                      Prize: X=18641, Y=10279";

    #[test]
    fn example() {
        let day = init(EX);
        assert_eq!(day.part1().expect("Part 1"), 480isize.into());
        assert_eq!(day.part2().expect("Part 2"), Unimplemented);
    }

    fn init(input: &str) -> Today {
        let mut day = Today::new();
        day.init(AocInput::new(input).trim()).expect("Init failed");
        day
    }

}
