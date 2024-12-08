use anyhow::{anyhow, Error};
use crate::aocday::{AocDay, AocInfo, AocInput, AocOutput};
use crate::aocday::AocOutput::Unimplemented;

#[derive(Default)]
pub struct Day5 {
}

type Today = Day5;

impl Today {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

}

impl AocDay for Today {

    fn info(&self) -> AocInfo {
        (5, "Print Queue").into()
    }

    fn init(&mut self, _input: AocInput) -> Result<(), Error> {
        Ok(())
    }

    fn part1(&self) -> Result<AocOutput, Error> {
        Ok(Unimplemented)
    }

    fn part2(&self) -> Result<AocOutput, Error> {
        Ok(Unimplemented)
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    const EX: &str = "";

    #[test]
    fn example() {
        let day = init(EX);
        assert_eq!(day.part1().expect("Part 1"), Unimplemented);
        assert_eq!(day.part2().expect("Part 2"), Unimplemented);
    }

    fn init(input: &str) -> Today {
        let mut day = Today::new();
        day.init(AocInput::new(input).trim()).expect("Init failed");
        day
    }

}
