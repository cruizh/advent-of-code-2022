fn main() {
    let input = include_str!("../../input/day06.txt");

    let input_chars: Vec<char> = input.chars().collect();

    const WINDOW_SIZE: usize = 4;
    let start_of_packet: usize = input_chars
        .windows(WINDOW_SIZE)
        .position(|s| (0..WINDOW_SIZE-1).all(|i| (i + 1..WINDOW_SIZE).all(|j| s[i] != s[j])))
        .unwrap() + 4;

    println!("{}", start_of_packet);
}
