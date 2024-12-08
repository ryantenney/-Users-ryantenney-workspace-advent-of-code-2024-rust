use std::collections::HashSet;
use std::ops::AddAssign;
use anyhow::{anyhow, Error};
use crate::aocday::{AocDay, AocInfo, AocInput, AocOutput};
use crate::aocday::AocOutput::Unimplemented;
use crate::grid::{Direction, Grid, Position, ToVector};
use crate::grid::Direction::Up;

#[derive(Default)]
pub struct Day6 {
    grid: Grid<char>,
    init_pos: Position,
}

type Today = Day6;

impl Today {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }
}

impl AocDay for Today {

    fn info(&self) -> AocInfo {
        (6, "Template").into()
    }

    fn init(&mut self, input: AocInput) -> Result<(), Error> {
        self.grid.init(&input.lines());

        if let Some((pos, _)) = self.grid.enumerate().filter(|(_, chr)| **chr == '^').next() {
            self.init_pos = pos;
            self.grid.set(pos, '.');
        }

        Ok(())
    }

    fn part1(&self) -> Result<AocOutput, Error> {
        let mut grid = self.grid.clone();
        let mut pos = self.init_pos.clone();
        let mut dir = Up;

        let mut pos_set = HashSet::new();
        pos_set.insert(pos);

        while let Some(chr) = grid.get_dir(pos, &dir) {
            match chr {
                '#' => {
                    pos_set.insert(pos);
                    dir = dir.clockwise()
                },
                '.' => {
                    pos_set.insert(pos);
                    grid.set(pos, 'X');
                    pos += dir;
                    pos_set.insert(pos);
                },
                'X' => {
                    pos_set.insert(pos);
                    pos += dir;
                    pos_set.insert(pos);
                },
                _ => unreachable!(),
            }
        }

        if grid.get(pos).is_some() && *grid.get(pos).unwrap() == '.' {
            grid.set(pos, 'X');
            pos_set.insert(pos);
        }

        Ok((pos_set.len() as u32).into())
    }

    fn part2(&self) -> Result<AocOutput, Error> {
        let initial_grid = self.grid.clone();
        let initial_pos = self.init_pos.clone();

        let mut barrier_set = HashSet::new();

        let mut pos = self.init_pos.clone();
        let mut dir = Up;

        while let Some(chr) = initial_grid.get_dir(pos, &dir) {
            match chr {
                '#' => dir = dir.clockwise(),
                '.' => {
                    let mut grid_clone = initial_grid.clone();
                    let barrier_pos = pos + dir;
                    grid_clone.set(barrier_pos, '#');
                    if run_cycle_check(&grid_clone, initial_pos, &Up) {
                        barrier_set.insert(barrier_pos);
                    }
                    pos += dir;
                },
                _ => unreachable!(),
            }
        }

        Ok((barrier_set.len() as u32).into())
    }

}

fn run_cycle_check(grid: &Grid<char>, pos: Position, dir: &Direction) -> bool {
    let mut pos = pos;
    let mut dir = *dir;
    let mut visited = HashSet::new();

    while let Some(chr) = grid.get_dir(pos, &dir) {
        if !visited.insert((pos, dir)) {
            return true;
        }

        match chr {
            '#' => dir = dir.clockwise(),
            '.' | 'X' => pos += dir,
            _ => todo!(),
        }
    }

    false
}

#[cfg(test)]
mod tests {

    use super::*;

    const EX: &str = "....#.....
                      .........#
                      ..........
                      ..#.......
                      .......#..
                      ..........
                      .#..^.....
                      ........#.
                      #.........
                      ......#...";

    #[test]
    fn example() {
        let day = init(EX);
        assert_eq!(day.part1().expect("Part 1"), 41.into());
        assert_eq!(day.part2().expect("Part 2"), 6.into());
    }

    fn init(input: &str) -> Today {
        let mut day = Today::new();
        day.init(AocInput::new(input).trim()).expect("Init failed");
        day
    }

}
