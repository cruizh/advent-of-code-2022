use std::str::FromStr;

struct Range {
    lower: u32,
    upper: u32
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.lower <= other.lower && other.upper <= self.upper
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
    let result = include_str!("../../input/day04.txt")
    .lines()
    .filter(|line| {
        let (first, second) = line.split_once(',').unwrap();
        let first_range: Range = first.parse().unwrap();
        let second_range: Range = second.parse().unwrap();
        first_range.contains(&second_range) || second_range.contains(&first_range)
    })
    .count();

    println!("{}", result);
}