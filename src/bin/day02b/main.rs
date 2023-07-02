mod moves;
mod strategy;
mod round;
mod outcome;

use std::fs;

use strategy::*;
fn main() {
    let input = fs::read_to_string("input/day02.txt").unwrap();
    let strategy: OutcomeStrategy = input.parse().unwrap();
    let total_score: i32 = strategy.rounds.into_iter().map(|round| round.player_move().player_score() + round.outcome.score()).sum();

    println!("{}", total_score);
}