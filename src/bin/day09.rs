use std::{str::FromStr, num::ParseIntError, fs::read_to_string};

type Position = (i32, i32);

#[derive(Debug, PartialEq, Eq)]
struct Rope {
    head: Position,
    tail: Position,
}

#[derive(Debug, PartialEq, Eq)]
enum Step {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, thiserror::Error)]
enum ParseStepError {
    #[error("Invalid step")]
    InvalidStep(String)
}

impl FromStr for Step {
    type Err = ParseStepError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Step::*;
        match s {
            "L" => Ok(Left),
            "R" => Ok(Right),
            "U" => Ok(Up),
            "D" => Ok(Down),
            _ => Err(ParseStepError::InvalidStep(s.to_string())),
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
struct Motion {
    step: Step,
    amount: usize,
}
#[derive(Debug, thiserror::Error)]
enum ParseMotionError {
    #[error("No word separator found")]
    Word,
    #[error("Invalid step")]
    Step(ParseStepError),
    #[error("Invalid amount")]
    Amount(ParseIntError)
}

impl From<ParseIntError> for ParseMotionError {
    fn from(value: ParseIntError) -> Self {
        ParseMotionError::Amount(value)
    }
}

impl From<ParseStepError> for ParseMotionError {
    fn from(value: ParseStepError) -> Self {
        ParseMotionError::Step(value)
    }    
}

impl FromStr for Motion {
    type Err = ParseMotionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, amount) = s.split_once(' ').ok_or(ParseMotionError::Word)?;
        Ok(Motion {
            step: dir.parse()?,
            amount: amount.parse()?,
        })
    }
}

fn apply_step_position(position: &Position, direction: Step) -> Position {
    let (x, y) = position;
    match direction {
        Left => (x-1, *y),
        Right => (x+1, *y),
        Up => (*x, y+1),
        Down => (*x, y-1)
    }
}

fn apply(rope: Rope, step: Step) -> Rope {
    Rope { head: (), tail: () }
}


fn main() {
    let input: String = read_to_string("input/day08.txt").unwrap();
    let motions: Vec<Motion> = input
        .lines()
        .map(FromStr::from_str)
        .collect::<Result<_,_>>()
        .unwrap();


}
