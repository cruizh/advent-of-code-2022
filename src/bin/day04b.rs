use std::fs::read_to_string;
use std::str::FromStr;
use std::cmp::{max,min};

struct Range {
    lower: u32,
    upper: u32
}

impl Range {
    fn overlaps(&self, other: &Range) -> bool {
        self.intersection(other).is_some()
    }

    fn intersection(&self, other: &Range) -> Option<Range> {
        let lower = max(self.lower, other.lower);
        let upper = min(self.upper, other.upper);
        if lower<=upper {
            Some(Range {lower, upper})
        } else {
            None
        }
    }
}

impl FromStr for Range {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lower, upper) = s.split_once('-').unwrap_or_default();
        Ok(Range {lower: lower.parse()?, upper: upper.parse()?})
    }
}

fn main() {
    let result = read_to_string("input/day04.txt").unwrap()
    .lines()
    .filter(|line| {
        let (first, second) = line.split_once(',').unwrap();
        let first_range: Range = first.parse().unwrap();
        let second_range: Range = second.parse().unwrap();
        first_range.overlaps(&second_range)
    })
    .count();

    println!("{}", result);
}