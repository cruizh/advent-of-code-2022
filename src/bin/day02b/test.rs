#[test]
fn test_outcome_round_to_player_round_draw() {
    let outcome_round = OutcomeRound {
        opponent: Move::Rock,
        outcome: Outcome::Draw
    };
    let player_round: PlayerRound = outcome_round.into();
    let expected: PlayerRound = PlayerRound { opponent: Move::Rock, player: Move::Rock };
    assert_eq!(player_round, expected);
}
#[test]
fn test_outcome_round_to_player_round_win() {
    let outcome_round = OutcomeRound {
        opponent: Move::Rock,
        outcome: Outcome::Win
    };
    let player_round: PlayerRound = outcome_round.into();
    let expected: PlayerRound = PlayerRound { opponent: Move::Rock, player: Move::Paper };
    assert_eq!(player_round, expected);
}
#[test]
fn test_outcome_round_to_player_round_loss() {
    let outcome_round = OutcomeRound {
        opponent: Move::Rock,
        outcome: Outcome::Loss
    };
    let player_round: PlayerRound = outcome_round.into();
    let expected: PlayerRound = PlayerRound { opponent: Move::Rock, player: Move::Scissors };
    assert_eq!(player_round, expected);
}
#[test]
fn test_outcome_round_to_player_round_draw2() {
    let outcome_round = OutcomeRound {
        opponent: Move::Paper,
        outcome: Outcome::Draw
    };
    let player_round: PlayerRound = outcome_round.into();
    let expected: PlayerRound = PlayerRound { opponent: Move::Paper, player: Move::Paper };
    assert_eq!(player_round, expected);
}
#[test]
fn test_outcome_round_to_player_round_win2() {
    let outcome_round = OutcomeRound {
        opponent: Move::Paper,
        outcome: Outcome::Win
    };
    let player_round: PlayerRound = outcome_round.into();
    let expected: PlayerRound = PlayerRound { opponent: Move::Paper, player: Move::Scissors };
    assert_eq!(player_round, expected);
}
#[test]
fn test_outcome_round_to_player_round_loss2() {
    let outcome_round = OutcomeRound {
        opponent: Move::Paper,
        outcome: Outcome::Loss
    };
    let player_round: PlayerRound = outcome_round.into();
    let expected: PlayerRound = PlayerRound { opponent: Move::Paper, player: Move::Rock };
    assert_eq!(player_round, expected);
}

#[test]
fn test_outcome_round_to_player_round_draw3() {
    let outcome_round = OutcomeRound {
        opponent: Move::Scissors,
        outcome: Outcome::Draw
    };
    let player_round: PlayerRound = outcome_round.into();
    let expected: PlayerRound = PlayerRound { opponent: Move::Scissors, player: Move::Scissors };
    assert_eq!(player_round, expected);
}
#[test]
fn test_outcome_round_to_player_round_win3() {
    let outcome_round = OutcomeRound {
        opponent: Move::Scissors,
        outcome: Outcome::Win
    };
    let player_round: PlayerRound = outcome_round.into();
    let expected: PlayerRound = PlayerRound { opponent: Move::Scissors, player: Move::Rock };
    assert_eq!(player_round, expected);
}
#[test]
fn test_outcome_round_to_player_round_loss3() {
    let outcome_round = OutcomeRound {
        opponent: Move::Scissors,
        outcome: Outcome::Loss
    };
    let player_round: PlayerRound = outcome_round.into();
    let expected: PlayerRound = PlayerRound { opponent: Move::Scissors, player: Move::Paper };
    assert_eq!(player_round, expected);
}


#[test]
fn test_score() {
    let outcome_round = OutcomeRound {
        opponent: FromStr::from_str("A").unwrap(),
        outcome: FromStr::from_str("Y").unwrap()
    };
    let player_round: PlayerRound = outcome_round.into();
    let calculated_score: i32 = player_round.into();
    let expected_score: i32 = 1+3;
    assert_eq!(calculated_score, expected_score);
}