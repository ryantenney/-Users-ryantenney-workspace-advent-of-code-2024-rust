use std::cell::OnceCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Pointer, Write};
use anyhow::{anyhow, Error};
use itertools::Itertools;
use crate::aocday::{AocDay, AocInfo, AocInput, AocOutput};
use crate::aocday::AocOutput::Unimplemented;
use crate::day11::Stone::{Single, Pair, Multiple, Array, One, Year, Zero};

#[derive(Default)]
pub struct Day11 {
    root: OnceCell<Stone>,
}

type Today = Day11;

impl Today {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

}

impl AocDay for Today {

    fn info(&self) -> AocInfo {
        (11, "Plutonian Pebbles").into()
    }

    fn init(&mut self, input: AocInput) -> Result<(), Error> {
        let stones: Vec<_> = input.raw().trim()
            .split(' ')
            .map(Stone::single)
            .collect();

        let mut stone = stones.first().unwrap().clone();
        for next_stone in stones.iter().skip(1) {
            stone = Pair(Box::new(stone.clone()), Box::new(next_stone.clone()));
        }

        self.root.set(stone)
            .map_err(|_| anyhow!("Error setting"))
    }

    fn part1(&self) -> Result<AocOutput, Error> {
        let mut stone = self.root.get().expect("Must init problem first").clone();

        for _ in 0..25 {
            stone = stone.blink().normalize();
        }

        Ok(stone.count().into())
    }

    fn part2(&self) -> Result<AocOutput, Error> {
        let mut stone = self.root.get().expect("Must init problem first").clone();

        for _ in 0..75 {
            stone = stone.blink().normalize();
        }

        Ok(stone.count().into())
    }

}

#[derive(Clone, Debug)]
enum Stone {
    Zero,
    One,
    Year,
    Single(String, u128),
    Pair(Box<Stone>, Box<Stone>),
    Multiple(Box<Stone>, usize),
    Array(Vec<Stone>),
}

impl Stone {
    fn from(val: u128) -> Self {
        match val {
            0 => Zero,
            1 => One,
            2024 => Year,
            _ => Single(val.to_string(), val),
        }
    }

    fn single(str: &str) -> Self {
        match str {
            "0" => Zero,
            "1" => One,
            "2024" => Year,
            _ => Single(str.into(), str.parse::<u128>().unwrap()),
        }
    }

    fn pair(left: &str, right: &str) -> Self {
        Pair(Box::new(Self::single(left)), Box::new(Self::single(right)))
    }

    fn multiple(str: &str, count: usize) -> Self {
        Multiple(Box::new(Self::single(str)), count)
    }

    fn value(&self) -> Option<u128> {
        match self {
            Zero => Some(0),
            One => Some(1),
            Year => Some(2024),
            Single(_, val) => Some(*val),
            Multiple(stone, _) => stone.value(),
            _ => None,
        }
    }

    fn blink(&self) -> Self {
        match self {
            Zero => One,
            One => Year,
            Year => Self::pair("20", "24"),
            Single(str, val) =>
                if *val == 0 {
                    One
                } else if *val == 1 {
                    Year
                } else if str.len() % 2 == 0 {
                    let (left, right) = str.split_at(str.len() / 2);
                    let mut right = right.trim_start_matches('0');
                    if right == "" {
                        right = "0";
                    }
                    Self::pair(left, right)
                } else {
                    let result = str.parse::<u128>().unwrap() * 2024;
                    Single(result.to_string(), result)
                },
            Pair(left, right) => Pair(Box::new(left.blink()), Box::new(right.blink())),
            Multiple(stone, count) => stone.blink().times(*count),
            Array(stones) => Array(stones.iter().map(Stone::blink).collect_vec()),
        }
    }

    fn count(&self) -> usize {
        match self {
            Pair(left, right) => left.count() + right.count(),
            Multiple(stone, count) => stone.count() * count,
            Array(stones) => stones.iter().map(Stone::count).sum(),
            _ => 1,
        }
    }

    fn flatten(&self) -> Vec<Stone> {
        let mut vec = Vec::new();
        self.flatten_into(&mut vec);
        vec
    }

    fn flatten_into(&self, vec: &mut Vec<Stone>) {
        match self {
            Pair(left, right) => {
                left.flatten_into(vec);
                right.flatten_into(vec);
            },
            Multiple(stone, count) => {
                stone.flatten().iter().map(|s| s.times(*count)).collect_into(vec);
            },
            Array(stones) => {
                stones.iter().for_each(|stone| stone.flatten_into(vec));
            }
            _ => vec.push(self.clone()),
        }
    }

    fn times(&self, count: usize) -> Self {
        if count == 0 {
            panic!("at the disco!")
        }
        if count == 1 {
            return self.clone();
        }
        match self {
            Multiple(stone, current_count) => Multiple(stone.clone(), current_count * count),
            _ => Multiple(Box::new(self.clone()), count),
        }
    }

    fn normalize(&self) -> Self {
        Array(self.flatten()
            .iter()
            .filter_map(|stone| stone.value().and_then(|value| Some((value, stone.count()))))
            .into_group_map()
            .iter()
            .sorted_by_key(|(value, _)| *value)
            .map(|(value, counts)| Stone::from(*value).times(counts.into_iter().sum()))
            .collect_vec())
    }

}

impl Display for Stone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Zero => f.write_char('0'),
            One => f.write_char('1'),
            Year => f.write_str("2024"),
            Single(str, ..) => f.write_str(str.as_str()),
            Pair(left, right) => f.write_fmt(format_args!("{} {}", left, right)),
            Multiple(stone, count) => f.write_fmt(format_args!("{}x{}", stone, count)),
            Array(stones) => f.write_str(stones.iter().map(Stone::to_string).join(" ").as_str()),
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use super::*;

    const EX: &str = "125 17";

    #[test]
    fn example() {
        let day = init(EX);
        assert_eq!(day.part1().expect("Part 1"), 55312usize.into());
        assert_eq!(day.part2().expect("Part 2"), 65601038650482usize.into());
    }

    fn init(input: &str) -> Today {
        let mut day = Today::new();
        day.init(AocInput::new(input).trim()).expect("Init failed");
        day
    }

}
