use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter, UpperHex, Write};
use anyhow::{anyhow, Error};
use itertools::Itertools;
use crate::aocday::{AocDay, AocInfo, AocInput, AocOutput};
use crate::aocday::AocOutput::Unimplemented;

#[derive(Default)]
pub struct Day22 {
    buyers: Vec<Buyer>,
}

type Today = Day22;

impl Today {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

}

impl AocDay for Today {

    fn info(&self) -> AocInfo {
        (22, "Monkey Market").into()
    }

    fn init(&mut self, input: AocInput) -> Result<(), Error> {
        self.buyers = input.lines()
            .iter()
            .filter_map(|s| s.parse().ok())
            .map(|seed| {
                let mut buyer = Buyer::new(seed);
                buyer.rounds(2000);
                buyer
            })
            .collect();

        Ok(())
    }

    fn part1(&self) -> Result<AocOutput, Error> {
        let buyers = self.buyers.clone();

        Ok(buyers.iter()
            .map(|buyer| buyer.secret as u64)
            .sum::<u64>()
            .into())
    }

    fn part2(&self) -> Result<AocOutput, Error> {
        let buyers = self.buyers.clone();

        let seq_set = buyers.iter()
            .flat_map(|buyer| buyer.seq_map.iter())
            .into_group_map_by(|(seq, _)| *seq);

        let mut max = 0;
        for (_, prices) in seq_set {
            let total = prices.iter().map(|(_, price)| **price as u32).sum::<u32>();
            if total > max {
                max = total;
            }
        }

        Ok(max.into())
    }

}

#[derive(Clone, Default)]
struct Buyer {
    seed: u32,
    secret: u32,
    rounds: u16,
    price: u8,
    seq: Seq,
    price_change: i8,
    seq_map: HashMap<Seq, u8>,
}

impl Buyer {
    fn new(seed: u32) -> Self {
        Buyer {
            seed,
            secret: seed,
            price: (seed % 10) as u8,
            seq_map: HashMap::with_capacity(2000),
            .. Default::default()
        }
    }

    fn rounds(&mut self, rounds: u16) -> u32 {
        for _ in 0..rounds {
            self.round();
        }
        self.secret
    }

    fn round(&mut self) -> u32 {
        let mut next = self.secret;
        next = prune(mix(next, next << 6));
        next = prune(mix(next, next >> 5));
        next = prune(mix(next, next << 11));
        
        let price = (next % 10) as u8;
        let price_change = price.checked_signed_diff(self.price).unwrap();

        self.price = price;
        self.price_change = price_change;

        self.seq.shift(price_change);

        self.seq_map.entry(self.seq).or_insert(self.price);

        self.rounds += 1;
        self.secret = next;
        next
    }
}

fn mix(secret: u32, value: u32) -> u32 {
    secret ^ value
}

fn prune(secret: u32) -> u32 {
    secret % 16777216
}

#[derive(Clone, Copy, Hash, Eq, Ord, PartialOrd, PartialEq)]
struct Seq(u32);

impl Seq {
    fn shift(&mut self, value: i8) {
        self.0 = (self.0 << 8) | (value as u32 & 0xFF);
    }

    fn to_vec(&self) -> Vec<i8> {
        vec![
            (self.0 >> 24 & 0xFF) as i8,
            (self.0 >> 16 & 0xFF) as i8,
            (self.0 >> 8 & 0xFF) as i8,
            (self.0 & 0xFF) as i8,
        ]
    }
}

impl Default for Seq {
    fn default() -> Self {
        let mut seq = Seq(0);
        seq.shift(10);
        seq.shift(10);
        seq.shift(10);
        seq.shift(10);
        seq
    }
}

impl Debug for Seq {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Seq({})", self.to_vec().iter().join(", ")))
    }
}

impl UpperHex for Seq {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:08X}", self.0))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const EX1: &str = "1
                       10
                       100
                       2024";

    const EX2: &str = "1
                       2
                       3
                       2024";

    #[test]
    fn example() {
        let day = init(EX1);
        assert_eq!(day.part1().expect("Part 1"), 37327623u64.into());

        let day = init(EX2);
        assert_eq!(day.part2().expect("Part 2"), 23.into());
    }

    #[test]
    fn test_mix() {
        assert_eq!(mix(42, 15), 37);
    }

    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920);
    }

    #[test]
    fn test_buyer123() {
        let mut buyer = Buyer::new(123);
        println!("{:>8}: {} ({})", buyer.secret, buyer.price, buyer.price_change);
        println!("          {:X}", buyer.seq);
        assert_eq!(buyer.round(), 15887950);
        println!("{:>8}: {} ({})", buyer.secret, buyer.price, buyer.price_change);
        println!("          {:X}", buyer.seq);
        assert_eq!(buyer.round(), 16495136);
        println!("{:>8}: {} ({})", buyer.secret, buyer.price, buyer.price_change);
        println!("          {:X}", buyer.seq);
        assert_eq!(buyer.round(), 527345);
        println!("{:>8}: {} ({})", buyer.secret, buyer.price, buyer.price_change);
        println!("          {:X}", buyer.seq);
        assert_eq!(buyer.round(), 704524);
        println!("{:>8}: {} ({})", buyer.secret, buyer.price, buyer.price_change);
        println!("          {:X}", buyer.seq);
        assert_eq!(buyer.round(), 1553684);
        println!("{:>8}: {} ({})", buyer.secret, buyer.price, buyer.price_change);
        println!("          {:X}", buyer.seq);
        assert_eq!(buyer.round(), 12683156);
        println!("{:>8}: {} ({})", buyer.secret, buyer.price, buyer.price_change);
        println!("          {:X}", buyer.seq);
        assert_eq!(buyer.round(), 11100544);
        println!("{:>8}: {} ({})", buyer.secret, buyer.price, buyer.price_change);
        println!("          {:X}", buyer.seq);
        assert_eq!(buyer.round(), 12249484);
        println!("{:>8}: {} ({})", buyer.secret, buyer.price, buyer.price_change);
        println!("          {:X}", buyer.seq);
        assert_eq!(buyer.round(), 7753432);
        println!("{:>8}: {} ({})", buyer.secret, buyer.price, buyer.price_change);
        println!("          {:X}", buyer.seq);
        assert_eq!(buyer.round(), 5908254);
        println!("{:>8}: {} ({})", buyer.secret, buyer.price, buyer.price_change);
        println!("          {:X}", buyer.seq);
    }

    fn init(input: &str) -> Today {
        let mut day = Today::new();
        day.init(AocInput::new(input).trim()).expect("Init failed");
        day
    }

}
