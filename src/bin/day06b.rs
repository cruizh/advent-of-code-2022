fn main() {
    let input = include_str!("../../input/day06.txt");

    let input_chars: Vec<char> = input.chars().collect();

    const WINDOW_SIZE: usize = 14;
    let start_of_message: usize = input_chars
        .windows(WINDOW_SIZE)
        .position(|s| (0..WINDOW_SIZE-1).all(|i| (i + 1..WINDOW_SIZE).all(|j| s[i] != s[j])))
        .unwrap() + WINDOW_SIZE;

    println!("{}", start_of_message);
}
