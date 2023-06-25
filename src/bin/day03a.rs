use std::collections::HashSet;

type Compartment = HashSet<char>;

fn compartment_from_str(s: &str) -> Compartment {
    let mut result: Compartment = HashSet::new();
    for item in s.chars() {
        result.insert(item);
    }
    result
}

fn priority(item: char) -> Option<i32> {
    match item {
        'a'..='z' => Some( 1 + (item as i32) - ('a' as i32)),
        'A'..='Z' => Some(27 + (item as i32) - ('A' as i32)),
        _ => None
    }
}

fn main() {

    let result: i32 = include_str!("../../input/day03.txt").lines().map(|line: &str| {
        let (left, right) = line.split_at(line.len()/2);
        let left_hash = compartment_from_str(left);
        let common = right.chars().find(|c| left_hash.contains(c)).unwrap();
        priority(common).unwrap_or(0)
    }).sum();

    println!("{}", result);
}

#[test]
fn test_lowercase_priority() {
    assert_eq!(priority('a'), Some(1));
    assert_eq!(priority('b'), Some(2));
}

#[test]
fn test_uppercase_priority() {
    assert_eq!(priority('A'), Some(27));
    assert_eq!(priority('B'), Some(28));
}