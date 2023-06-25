use std::str::FromStr;
use num_traits::{FromPrimitive,ToPrimitive};
use super::moves;
use super::moves::Move;
use super::outcome;
use super::outcome::Outcome;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PlayerRound {
    opponent: Move,
    player: Move,
}

impl PlayerRound {
    fn decide_outcome(self) -> Outcome {
        match ((self.opponent as i32) - (self.player as i32)).rem_euclid(3) {
            0 => Outcome::Draw,
            1 => Outcome::Loss,
            _ => Outcome::Win,
        }
    }
}

impl FromStr for PlayerRound {
    type Err = ParseError;

    fn from_str(round: &str) -> Result<Self, ParseError> {
        let moves: Vec<&str> = round.split_whitespace().collect();

        let opponent = match *moves.first().unwrap_or(&"") {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            s => Err(ParseError::Move(moves::ParseError::Opponent(s.to_string()))),
        }?;

        let player = match *moves.last().unwrap_or(&"") {
            "X" => Ok(Move::Rock),
            "Y" => Ok(Move::Paper),
            "Z" => Ok(Move::Scissors),
            s => Err(ParseError::Outcome(outcome::ParseError::Outcome(s.to_string()))),
        }?;

        Ok(PlayerRound { opponent, player })
    }
}

impl From<PlayerRound> for i32 {
    fn from(round: PlayerRound) -> Self {
        println!("Player round {:?} vs {:?}: {}", round.player, round.opponent, (round.player as i32) + (round.decide_outcome() as i32));
        (round.player as i32) + (round.decide_outcome() as i32)
    }
}

impl FromStr for OutcomeRound {
    type Err = ParseError;

    fn from_str(round: &str) -> Result<Self, ParseError> {
        let moves: Vec<&str> = round.split_whitespace().collect();
        let opponent: Move = FromStr::from_str(moves.first().unwrap_or(&""))?;
        let outcome: Outcome = FromStr::from_str(moves.last().unwrap_or(&""))?;

        Ok(OutcomeRound { opponent, outcome })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid move")]
    Move(moves::ParseError),
    #[error("Invalid outcome")]
    Outcome(outcome::ParseError)
}

impl From<moves::ParseError> for ParseError {
    fn from(value: moves::ParseError) -> Self {
        ParseError::Move(value)
    }
}
impl From<outcome::ParseError> for ParseError {
    fn from(value: outcome::ParseError) -> Self {
        ParseError::Outcome(value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OutcomeRound {
    pub opponent: Move,
    pub outcome: Outcome
}

impl OutcomeRound {
    pub fn player_move(self) -> Move {
        let opponent_value: i8 = self.opponent.to_i8().unwrap();
        match self.outcome {
            Outcome::Draw => self.opponent,
            Outcome::Win => FromPrimitive::from_i8((opponent_value + 1).rem_euclid(3)).unwrap(),
            Outcome::Loss => FromPrimitive::from_i8((opponent_value + 2).rem_euclid(3)).unwrap()
        }
    }
}

impl From<OutcomeRound> for PlayerRound {
    fn from(value: OutcomeRound) -> Self {
        PlayerRound { opponent: value.opponent, player: value.player_move() }
    }
}
