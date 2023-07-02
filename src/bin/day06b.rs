use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day06.txt").unwrap();
    let input_chars: Vec<char> = input.chars().collect();

    const WINDOW_SIZE: usize = 14;
    let start_of_message: usize = input_chars
        .windows(WINDOW_SIZE)
        .position(|s| (0..WINDOW_SIZE-1).all(|i| (i + 1..WINDOW_SIZE).all(|j| s[i] != s[j])))
        .unwrap() + WINDOW_SIZE;

    println!("{}", start_of_message);
}
