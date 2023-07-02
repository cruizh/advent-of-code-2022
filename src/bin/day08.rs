use std::fs::read_to_string;

type Forest = Vec<Vec<u8>>;

mod part1 {
    use super::Forest;
    fn visible_from_left(forest: &Forest, i: usize, j: usize) -> bool {
        let line = forest.get(i).unwrap().get(0..j).unwrap_or_default();
        let this = forest.get(i).unwrap().get(j).unwrap();
        match line.iter().max() {
            None => true,
            Some(max) => max < this,
        }
    }

    fn visible_from_right(forest: &Forest, i: usize, j: usize) -> bool {
        let line = forest
            .get(i)
            .unwrap()
            .get(j + 1..forest.get(i).unwrap().len())
            .unwrap_or_default();
        let this = forest.get(i).unwrap().get(j).unwrap();
        match line.iter().max() {
            None => true,
            Some(max) => max < this,
        }
    }

    fn visible_from_top(forest: &Forest, i: usize, j: usize) -> bool {
        let line = forest
            .get(0..i)
            .unwrap_or_default()
            .iter()
            .map(|l| l.get(j).unwrap());
        let this = forest.get(i).unwrap().get(j).unwrap();
        match line.into_iter().max() {
            None => true,
            Some(max) => max < this,
        }
    }

    fn visible_from_bottom(forest: &Forest, i: usize, j: usize) -> bool {
        let line = forest
            .get(i + 1..forest.len())
            .unwrap_or_default()
            .iter()
            .map(|l| l.get(j).unwrap());
        let this = forest.get(i).unwrap().get(j).unwrap();
        match line.into_iter().max() {
            None => true,
            Some(max) => max < this,
        }
    }

    pub fn visible(forest: &Forest, i: usize, j: usize) -> bool {
        vec![
            visible_from_left,
            visible_from_right,
            visible_from_top,
            visible_from_bottom,
        ]
        .into_iter()
        .map(|f| f(forest, i, j))
        .any(|f| f)
    }

    #[cfg(test)]
    mod tests {
        use super::super::parse_forest;
        use super::*;

        #[test]
        fn test_visible_from_top_0() {
            let input_str = "30373\n\
        25512\n\
        65332\n\
        33549\n\
        35390";
            let forest = parse_forest(input_str);
            assert!(
                visible_from_top(&forest, 0, 0),
                "(0,0) is not visible from top"
            );
            assert!(
                visible_from_top(&forest, 0, forest.len() - 1),
                "(0,n) is not visible from top"
            );
        }
    }
}

mod part2 {
    use super::Forest;
    fn visibility_left(forest: &Forest, i: usize, j: usize) -> usize {
        let mut result: usize = 0;
        for jx in (0..j).rev() {
            result += 1;
            if forest[i][jx] >= forest[i][j] {
                break;
            }
        }
        result
    }

    fn visibility_right(forest: &Forest, i: usize, j: usize) -> usize {
        let mut result: usize = 0;
        for jx in j + 1..forest[i].len() {
            result += 1;
            if forest[i][jx] >= forest[i][j] {
                break;
            }
        }
        result
    }

    fn visibility_top(forest: &Forest, i: usize, j: usize) -> usize {
        let mut result: usize = 0;
        for ix in (0..i).rev() {
            result += 1;
            if forest[ix][j] >= forest[i][j] {
                break;
            }
        }
        result
    }

    fn visibility_bottom(forest: &Forest, i: usize, j: usize) -> usize {
        let mut result: usize = 0;
        for ix in i + 1..forest.len() {
            result += 1;
            if forest[ix][j] >= forest[i][j] {
                break;
            }
        }
        result
    }

    pub fn scenic_score(forest: &Forest, i: usize, j: usize) -> usize {
        vec![
            visibility_left,
            visibility_right,
            visibility_top,
            visibility_bottom,
        ]
        .into_iter()
        .map(|f| f(forest, i, j))
        .product()
    }
}

fn parse_forest(value: &str) -> Forest {
    value
        .lines()
        .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect()
}

fn main() {
    let input = read_to_string("input/day08.txt").unwrap();
    let input_forest: Forest = parse_forest(input.as_str());

    // half naïve solution: iterate over rows and count the
    let total_visible_by_row_count: usize = input_forest
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(j, _)| part1::visible(&input_forest, i, *j))
                .count()
        })
        .sum();

    /*
    To do the nested iteration, we need to be careful about borrowing.
    If we do it the naïve way, with just nested map, we get the following error:
        closure may outlive the current function, but it borrows `i`, which is owned by the current function
    To fix this, we change the closure to a `move` closure.
    However, the `move` keyword will move all variables mentioned in the scope, including `input_forest`.
    To fix this, we instead assign a name to the reference `&input_forest` so we only move the reference.
    */
    let total_visible_by_nested_map: usize = input_forest
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            let input_forest_ref = &input_forest;
            row.iter()
                .enumerate()
                .filter(move |(j, _)| part1::visible(input_forest_ref, i, *j))
        })
        .count();

    println!(
        "Part 1: Row count: {}, Nested map: {}",
        total_visible_by_row_count, total_visible_by_nested_map
    );

    let max_visibility: usize = input_forest
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            let input_forest_ref = &input_forest;
            row.iter()
                .enumerate()
                .map(move |(j, _)| part2::scenic_score(input_forest_ref, i, j))
        })
        .max()
        .unwrap();
    println!("Part 2: {}", max_visibility);
}
