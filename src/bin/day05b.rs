use std::{str::FromStr, fs::read_to_string};

type Stack = Vec<char>;

fn parse_to_crates(drawing: &str) -> Vec<Stack> {
    let mut drawing_rev = drawing.lines().rev();
    let length = drawing_rev.next().unwrap().split_whitespace().count();
    let mut crates: Vec<Stack> = Vec::with_capacity(length);
    crates.resize_with(length, Vec::new);

    for line in drawing_rev {
        for i in 0..length {
            let chars: Vec<char> = line.chars().collect();
            if let Some(cargo_crate) = chars.get(4 * i + 1) {
                if 'A' <= *cargo_crate && *cargo_crate <= 'Z' {
                    let stack: &mut Stack = crates.get_mut(i).unwrap();
                    stack.push(cargo_crate.to_owned());
                }
            }
        }
    }
    crates
}

#[derive(Debug)]
struct Action {
    quantity: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, thiserror::Error)]
enum ParseActionError {
    #[error("From not found")]
    From,
    #[error("To not found")]
    To,
    #[error("Move not found")]
    Move,
    #[error("Integer parsing error")]
    Int(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for ParseActionError {
    fn from(value: std::num::ParseIntError) -> Self {
        ParseActionError::Int(value)
    }
}

fn apply(action: &Action, supplies: &Vec<Stack>) -> Option<Vec<Stack>> {
    let mut from = supplies.get(action.from)?.to_owned();
    let mut to = supplies.get(action.to)?.to_owned();

    let mut result = supplies.to_owned();
    match action.quantity {
        n if n <= from.len() => {
            let mut grabbed = from.split_off(from.len() - action.quantity);
            to.append(&mut grabbed);
            result[action.from] = from;
            result[action.to] = to;
            Some(result)
        }
        _ => None,
    }
}

impl FromStr for Action {
    type Err = ParseActionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once("from") {
            None => Err(ParseActionError::From),
            Some((quantity_spec, direction)) => match direction.split_once("to") {
                None => Err(ParseActionError::To),
                Some((from, to)) => match quantity_spec.split_once(' ') {
                    None => Err(ParseActionError::Move),
                    Some((_, quantity)) => {
                        let q: usize = quantity.trim().parse()?;
                        let f: usize = from.trim().parse()?;
                        let t: usize = to.trim().parse()?;

                        Ok(Action {
                            quantity: q,
                            from: f - 1,
                            to: t - 1,
                        })
                    }
                },
            },
        }
    }
}

fn main() {
    let input = read_to_string("input/day05.txt").unwrap();
    let (initial_state, actions_spec) = input
        .split_once("\n\n")
        .unwrap();
    let crates: Vec<Stack> = parse_to_crates(initial_state);

    let actions: Vec<Action> = actions_spec
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let final_state: Vec<Stack> = actions.iter().fold(crates, |acc, e| {
        apply(e, &acc).unwrap_or_else(|| {
            println!("Failed to apply action {:?} on supplies {:?}", e, acc);
            acc
        })
    });

    let result: String = final_state
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect();

    println!("{}", result);
}

#[test]
fn test_initial_state() {
    let initial_state: Vec<Stack> = vec![
        vec!['F', 'C', 'P', 'G', 'Q', 'R'],
        vec!['W', 'T', 'C', 'P'],
        vec!['B', 'H', 'P', 'M', 'C'],
        vec!['L', 'T', 'Q', 'S', 'M', 'P', 'R'],
        vec!['P', 'H', 'J', 'Z', 'V', 'G', 'N'],
        vec!['D', 'P', 'J'],
        vec!['L', 'G', 'P', 'Z', 'F', 'J', 'T', 'R'],
        vec!['N', 'L', 'H', 'C', 'F', 'P', 'T', 'J'],
        vec!['G', 'V', 'Z', 'Q', 'H', 'T', 'C', 'W'],
    ];

    let initial_state_spec = concat!(
        "                        [R] [J] [W]\n",
        "            [R] [N]     [T] [T] [C]\n",
        "[R]         [P] [G]     [J] [P] [T]\n",
        "[Q]     [C] [M] [V]     [F] [F] [H]\n",
        "[G] [P] [M] [S] [Z]     [Z] [C] [Q]\n",
        "[P] [C] [P] [Q] [J] [J] [P] [H] [Z]\n",
        "[C] [T] [H] [T] [H] [P] [G] [L] [V]\n",
        "[F] [W] [B] [L] [P] [D] [L] [N] [G]\n",
        " 1   2   3   4   5   6   7   8   9 ");
    let crates: Vec<Stack> = parse_to_crates(initial_state_spec);
    assert_eq!(initial_state, crates);
}

#[test]
fn test_first_move() {
    let initial_state: Vec<Stack> = vec![
        vec!['F', 'C', 'P', 'G', 'Q', 'R'],
        vec!['W', 'T', 'C', 'P'],
        vec!['B', 'H', 'P', 'M', 'C'],
        vec!['L', 'T', 'Q', 'S', 'M', 'P', 'R'],
        vec!['P', 'H', 'J', 'Z', 'V', 'G', 'N'],
        vec!['D', 'P', 'J'],
        vec!['L', 'G', 'P', 'Z', 'F', 'J', 'T', 'R'],
        vec!['N', 'L', 'H', 'C', 'F', 'P', 'T', 'J'],
        vec!['G', 'V', 'Z', 'Q', 'H', 'T', 'C', 'W'],
    ];

    let first_move = Action { quantity: 2, from: 1, to: 7 };
    
    let final_state: Vec<Stack> = vec![
        vec!['F', 'C', 'P', 'G', 'Q', 'R'],
        vec!['W', 'T'],
        vec!['B', 'H', 'P', 'M', 'C'],
        vec!['L', 'T', 'Q', 'S', 'M', 'P', 'R'],
        vec!['P', 'H', 'J', 'Z', 'V', 'G', 'N'],
        vec!['D', 'P', 'J'],
        vec!['L', 'G', 'P', 'Z', 'F', 'J', 'T', 'R'],
        vec!['N', 'L', 'H', 'C', 'F', 'P', 'T', 'J', 'C', 'P'],
        vec!['G', 'V', 'Z', 'Q', 'H', 'T', 'C', 'W'],
    ];
    assert_eq!(final_state, apply(&first_move, &initial_state).unwrap());
}