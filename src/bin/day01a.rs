use std::{str::FromStr, fs};

#[derive(Debug)]
struct Elf {
    foods: Vec<Food>
}

impl FromStr for Elf {
    type Err = std::num::ParseIntError;

    fn from_str(elf_spec: &str) -> Result<Self, Self::Err> {
        let foods: Vec<Food> = elf_spec
        .lines()
        .map(|s: &str| s.parse::<Food>())
        .collect::<Result<Vec<Food>, _>>()?;

        Ok(Elf { foods })
    }
}

fn total_calories(elf: &Elf) -> i32 {
    elf.foods
    .iter()
    .map(|e| e.calories)
    .sum()
}

#[derive(Debug)]
struct Food {
    calories: i32
}

impl FromStr for Food {
    type Err = std::num::ParseIntError;
    
    fn from_str(food_spec: &str) -> Result<Self, Self::Err> {
        let calories: i32 = food_spec.parse()?;
        
        Ok(Food { calories })
    }
}

fn main() {
    let input = fs::read_to_string("input/day01.txt").unwrap();
    
    let elves: Vec<Elf> = input
    .split("\n\n")
    .map(|s| s.parse::<Elf>())
    .collect::<Result<Vec<Elf>, _>>()
    .unwrap();

    let max_calories = elves
    .iter()
    .map(total_calories)
    .max()
    .unwrap();

    println!("{}", max_calories);
}