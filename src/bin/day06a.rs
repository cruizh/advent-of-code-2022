use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day06.txt").unwrap();
    let input_chars: Vec<char> = input.chars().collect();

    const WINDOW_SIZE: usize = 4;
    let start_of_packet: usize = input_chars
        .windows(WINDOW_SIZE)
        .position(|s| (0..WINDOW_SIZE-1).all(|i| (i + 1..WINDOW_SIZE).all(|j| s[i] != s[j])))
        .unwrap() + 4;

    println!("{}", start_of_packet);
}
