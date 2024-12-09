use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};
use anyhow::{anyhow, Error};
use itertools::Itertools;
use crate::aocday::{AocDay, AocInfo, AocInput, AocOutput};
use crate::aocday::AocOutput::Unimplemented;
use crate::grid::{Grid, Position};

#[derive(Default)]
pub struct Day8 {
    grid: Grid<AntennaData>,
    antennas_by_freq: HashMap<char, Vec<(Position, AntennaData)>>,
}

type Today = Day8;

impl Today {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

}

impl AocDay for Today {

    fn info(&self) -> AocInfo {
        (8, "Resonant Collinearity").into()
    }

    fn init(&mut self, input: AocInput) -> Result<(), Error> {
        self.grid.init(&input.lines());

        self.antennas_by_freq = self.grid.enumerate()
            .filter(|(_, ant)| ant.freq != '.')
            .map(|(pos, ant)| (pos, ant.clone()))
            .into_group_map_by(|(_, ant)| ant.freq);

        Ok(())
    }

    fn part1(&self) -> Result<AocOutput, Error> {
        let grid = self.grid.clone();
        let mut antinodes = HashSet::new();
        self.antennas_by_freq.iter()
            .flat_map(|(_, antennas)| {
                antennas.iter()
                    .permutations(2)
                    .flat_map(move |f| {
                        let pos1 = f[0].0;
                        let pos2 = f[1].0;
                        let vector = pos2 - pos1;

                        vec![
                            pos2.checked_add(&vector),
                            pos1.checked_sub(&vector),
                        ]
                    })
                })
            .flatten()
            .filter(|pos| grid.in_bounds(*pos))
            .collect_into(&mut antinodes);

        /*
        antinodes.iter().for_each(|ps| {
            if let Some(cell) = grid.get_mut(*pos) {
                cell.add_antinode('#');
            }
        });
        println!("{}", grid);
         */

        Ok((antinodes.len() as u32).into())
    }

    fn part2(&self) -> Result<AocOutput, Error> {
        let grid = self.grid.clone();
        let mut antinodes = HashSet::new();
        self.antennas_by_freq.iter()
            .flat_map(|(_, antennas)| {
                antennas.iter()
                    .map(|f| f.0)
                    .permutations(2)
                    .map(|f| (f[0], f[1]))
                    .flat_map(move |(pos1, pos2)| {
                        let vector = pos2 - pos1;

                        let mut vec = vec![];
                        let mut pos1 = pos1;
                        let mut pos2 = pos2;
                        for _ in 0..grid.bounds.x {
                            if let Some(pos) = pos1.checked_sub(&vector) {
                                vec.push(pos);
                                pos1 = pos;
                            }
                            if let Some(pos) = pos2.checked_sub(&vector) {
                                vec.push(pos);
                                pos2 = pos;
                            }
                        }
                        vec
                    })
            })
            //.flat_map(|f| vec![f.0, f.1])
            //.flatten()
            .filter(|pos| grid.in_bounds(*pos))
            .collect_into(&mut antinodes);

        /*
        antinodes.iter().for_each(|ps| {
            if let Some(cell) = grid.get_mut(*pos) {
                cell.add_antinode('#');
            }
        });
        println!("{}", grid);
         */

        Ok((antinodes.len() as u32).into())
    }

}

#[derive(Clone, Debug)]
struct AntennaData {
    freq: char,
    antinodes: HashSet<char>,
}

impl AntennaData {
    fn is_antinode(&self) -> bool {
        !self.antinodes.is_empty()
    }

    fn add_antinode(&mut self, freq: char) -> bool {
        self.antinodes.insert(freq)
    }
}

impl Display for AntennaData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.antinodes.is_empty() {
            f.write_char(self.freq)
        } else if self.freq == '.' {
            f.write_char('#')
        } else {
            f.write_char('*')
        }
    }
}

impl From<char> for AntennaData {
    fn from(value: char) -> Self {
        Self {
            freq: value,
            antinodes: HashSet::new(),
        }
    }
}

impl Default for AntennaData {
    fn default() -> Self {
        Self {
            freq: '.',
            antinodes: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const EX: &str = "............
                      ........0...
                      .....0......
                      .......0....
                      ....0.......
                      ......A.....
                      ............
                      ............
                      ........A...
                      .........A..
                      ............
                      ............";

    #[test]
    fn example() {
        let day = init(EX);
        assert_eq!(day.part1().expect("Part 1"), 14.into());
        assert_eq!(day.part2().expect("Part 2"), 34.into());
    }

    fn init(input: &str) -> Today {
        let mut day = Today::new();
        day.init(AocInput::new(input).trim()).expect("Init failed");
        day
    }

}
