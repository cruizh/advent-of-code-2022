mod moves;
mod strategy;
mod round;
mod outcome;

use strategy::*;
fn main() {
    let strategy: OutcomeStrategy = include_str!("../../../input/day02.txt").parse().unwrap();
    let total_score: i32 = strategy.rounds.into_iter().map(|round| round.player_move().player_score() + round.outcome.score()).sum();

    println!("{}", total_score);
}