use std::collections::HashSet;

fn priority(item: &char) -> Option<i32> {
    match *item {
        'a'..='z' => Some(1 + (*item as i32) - ('a' as i32)),
        'A'..='Z' => Some(27 + (*item as i32) - ('A' as i32)),
        _ => None,
    }
}

fn main() {

    let commons: Vec<char> = include_str!("../../input/day03.txt")
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .filter_map(|g: &[&str]| {
            let a: HashSet<char> = HashSet::from_iter(g[0].chars().to_owned());
            let b: HashSet<char> = HashSet::from_iter(g[1].chars().to_owned());
            let c: HashSet<char> = HashSet::from_iter(g[2].chars().to_owned());
            let intersection: Vec<char> = a
                .intersection(&b)
                .filter(|it: &&char| c.contains(*it))
                .map(|c| c.to_owned())
                .collect();
            intersection.first().copied()
        })
        .collect();

    let result: i32 = commons.iter().map(|c| priority(c).unwrap_or(0)).sum();

    println!("{}", result);
}

#[test]
fn test_lowercase_priority() {
    assert_eq!(priority(&'a'), Some(1));
    assert_eq!(priority(&'b'), Some(2));
}

#[test]
fn test_uppercase_priority() {
    assert_eq!(priority(&'A'), Some(27));
    assert_eq!(priority(&'B'), Some(28));
}
