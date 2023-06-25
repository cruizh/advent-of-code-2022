use std::str::FromStr;
use num_derive::{FromPrimitive,ToPrimitive};

#[derive(Debug, PartialEq, Eq, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 0
}

impl Move {
    pub fn player_score(self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid opponent move")]
    Opponent(String)
}

impl FromStr for Move {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            _ => Err(ParseError::Opponent(s.to_string()))
        }
    }
}

impl From<Move> for i32 {
    fn from(shape: Move) -> Self {
        match shape {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}
