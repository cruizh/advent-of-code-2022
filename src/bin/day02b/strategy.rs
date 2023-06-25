use std::str::FromStr;
use super::round;
use super::round::{PlayerRound,OutcomeRound};

pub struct OutcomeStrategy {
    pub rounds: Vec<OutcomeRound>
}

impl FromStr for OutcomeStrategy {
    type Err = round::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rounds: Vec<OutcomeRound> = s
            .lines()
            .map(|l| l.parse::<OutcomeRound>())
            .collect::<Result<_, _>>()?;
        Ok(OutcomeStrategy { rounds })
    }
}

pub struct PlayerStrategy {
    rounds: Vec<PlayerRound>,
}

impl FromStr for PlayerStrategy {
    type Err = round::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rounds: Vec<PlayerRound> = s
            .lines()
            .map(|l| l.parse::<PlayerRound>())
            .collect::<Result<_, _>>()?;
        Ok(PlayerStrategy { rounds })
    }
}

impl From<PlayerStrategy> for i32 {
    fn from(strategy: PlayerStrategy) -> Self {
        strategy
            .rounds
            .into_iter()
            .map(<PlayerRound as std::convert::Into<i32>>::into)
            .sum()
    }
}

impl From<OutcomeStrategy> for PlayerStrategy {
    fn from(value: OutcomeStrategy) -> Self {
        let rounds: Vec<PlayerRound> = value.rounds.iter().map(|r| <OutcomeRound as std::convert::Into<PlayerRound>>::into(*r)).collect();
        PlayerStrategy { rounds }
    }
}