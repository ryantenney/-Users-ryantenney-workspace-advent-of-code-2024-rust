use std::fmt::{Debug, Display, Formatter};
use std::io::Lines;
use std::ops::Deref;
use anyhow::{anyhow, Error};
use crate::main;

pub trait AocDay {

    fn info(&self) -> AocInfo;

    fn init(&mut self, input: AocInput) -> Result<(), Error>;

    fn part1(&self) -> Result<AocOutput, Error>;

    fn part2(&self) -> Result<AocOutput, Error>;

}

pub struct AocInfo {
    day: u8,
    name: String,
}

impl AocInfo {

    pub fn new(day: u8, name: &str) -> Self {
        AocInfo {
            day,
            name: name.to_string()
        }
    }

    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

}

impl Into<(u8, String)> for AocInfo {
    fn into(self) -> (u8, String) {
        (self.day, self.name)
    }
}

impl From<(u8, String)> for AocInfo {
    fn from(value: (u8, String)) -> Self {
        AocInfo::new(value.0, value.1.as_str())
    }
}

impl From<(u8, &str)> for AocInfo {
    fn from(value: (u8, &str)) -> Self {
        AocInfo::new(value.0, value.1)
    }
}

pub struct AocInput {
    lines: Vec<String>,
    raw: String,
    pos: usize,
}

impl AocInput {
    pub fn new(raw: &str) -> Self {
        Self {
            lines: raw.lines().map(str::to_string).collect(),
            raw: raw.to_string(),
            pos: 0,
        }
    }

    pub fn lines(&self) -> Vec<String> {
        self.lines.clone()
    }

    pub fn raw(&self) -> String {
        self.raw.clone()
    }

    pub fn len(&self) -> usize {
        self.raw.len()
    }

    pub fn trim(&self) -> Self {
        let trimmed_lines: Vec<String> = self.lines.iter()
            .map(|s| s.trim().to_string())
            .collect();

        Self {
            lines: trimmed_lines.clone(),
            raw: trimmed_lines.join("\n"),
            pos: 0,
        }
    }
}

impl Clone for AocInput {
    fn clone(&self) -> Self {
        Self {
            lines: self.lines.clone(),
            raw: self.raw.clone(),
            pos: 0
        }
    }
}

impl Iterator for AocInput {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.lines.get(self.pos);
        self.pos += 1;
        next.map(String::clone)
    }
}

#[derive(Debug, PartialEq)]
pub enum AocOutput {
    Unimplemented,
    String(String),
    Multiline(String),
    Num(i32),
    BigUnsigned(usize),
    BigSigned(isize),
}

impl Display for AocOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AocOutput::Unimplemented => f.write_str("Unimplemented"),
            AocOutput::String(str) => f.write_str(str),
            AocOutput::Multiline(str) => f.write_str(str),
            AocOutput::Num(num) => f.write_fmt(format_args!("{num}")),
            AocOutput::BigUnsigned(num) => f.write_fmt(format_args!("{num}")),
            AocOutput::BigSigned(num) => f.write_fmt(format_args!("{num}")),
        }
    }
}

impl From<i8> for AocOutput {
    fn from(value: i8) -> Self {
        Self::Num(value as i32)
    }
}

impl From<i16> for AocOutput {
    fn from(value: i16) -> Self {
        Self::Num(value as i32)
    }
}

impl From<i32> for AocOutput {
    fn from(value: i32) -> Self {
        Self::Num(value)
    }
}

impl From<i64> for AocOutput {
    fn from(value: i64) -> Self {
        Self::BigSigned(value as isize)
    }
}

impl From<isize> for AocOutput {
    fn from(value: isize) -> Self {
        Self::BigSigned(value)
    }
}

impl From<u8> for AocOutput {
    fn from(value: u8) -> Self {
        Self::Num(value as i32)
    }
}

impl From<u16> for AocOutput {
    fn from(value: u16) -> Self {
        Self::Num(value as i32)
    }
}

impl From<u32> for AocOutput {
    fn from(value: u32) -> Self {
        Self::Num(value as i32)
    }
}

impl From<u64> for AocOutput {
    fn from(value: u64) -> Self {
        Self::BigUnsigned(value as usize)
    }
}

impl From<usize> for AocOutput {
    fn from(value: usize) -> Self {
        Self::BigUnsigned(value)
    }
}

impl From<&str> for AocOutput {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for AocOutput {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
