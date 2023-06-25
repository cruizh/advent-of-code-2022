use std::str::FromStr;

struct Strategy {
    rounds: Vec<Round>,
}
struct Round {
    opponent: Move,
    player: Move,
}

#[derive(Debug, PartialEq)]
enum RoundOutcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl Round {
    fn decide(self) -> RoundOutcome {
        match ((self.opponent as i32) - (self.player as i32)).rem_euclid(3) {
            0 => RoundOutcome::Draw,
            1 => RoundOutcome::Loss,
            _ => RoundOutcome::Win,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, thiserror::Error)]
pub enum MoveParseError {
    #[error("Invalid opponent move")]
    OpponentParseError(String),
    #[error("Invalid player move")]
    PlayerParseError(String),
}

impl FromStr for Round {
    type Err = MoveParseError;

    fn from_str(round: &str) -> Result<Self, MoveParseError> {
        let moves: Vec<&str> = round.split_whitespace().collect();

        let opponent = match *moves.first().unwrap_or(&"") {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            s => Err(MoveParseError::OpponentParseError(s.to_string())),
        }?;

        let player = match *moves.last().unwrap_or(&"") {
            "X" => Ok(Move::Rock),
            "Y" => Ok(Move::Paper),
            "Z" => Ok(Move::Scissors),
            s => Err(MoveParseError::OpponentParseError(s.to_string())),
        }?;

        Ok(Round { opponent, player })
    }
}

impl FromStr for Strategy {
    type Err = MoveParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rounds: Vec<Round> = s
            .lines()
            .map(|l| l.parse::<Round>())
            .collect::<Result<_, _>>()?;
        Ok(Strategy { rounds })
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

impl From<Round> for i32 {
    fn from(round: Round) -> Self {
        (round.player as i32) + (round.decide() as i32)
    }
}

impl From<Strategy> for i32 {
    fn from(strategy: Strategy) -> Self {
        strategy
            .rounds
            .into_iter()
            .map(<Round as std::convert::Into<i32>>::into)
            .sum()
    }
}

fn main() {
    let strategy: Strategy = include_str!("../../input/day02.txt").parse().unwrap();

    let total_score: i32 = strategy.into();

    println!("{}", total_score);
}

#[test]
fn test_round_decide() {
    assert_eq!(
        (Round {
            opponent: Move::Rock,
            player: Move::Rock
        })
        .decide(),
        RoundOutcome::Draw
    );
    assert_eq!(
        (Round {
            opponent: Move::Rock,
            player: Move::Paper
        })
        .decide(),
        RoundOutcome::Win
    );

    assert_eq!(
        (Round {
            opponent: Move::Rock,
            player: Move::Scissors
        })
        .decide(),
        RoundOutcome::Loss
    );
}

#[test]
fn test_moves_as_i32() {
    assert_eq!(Move::Rock as i32, 1);
    assert_eq!(Move::Paper as i32, 2);
    assert_eq!(Move::Scissors as i32, 3);
}

#[test]
fn test_comparison_of_moves() {
    assert_eq!(((Move::Rock as i32) - (Move::Scissors as i32)).rem_euclid(3), 1)
}