use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

// lines() divides by linebreak
// flat_map() gets all lines into one line
// We then construct both columns by iterating in steps of 2 (+ an offset)
#[aoc_generator(day1, part1)]
fn parse_part1(input: &str) -> (Vec<u32>, Vec<u32>) {
    let numbers: Vec<u32> = input
        .lines()
        .flat_map(|line| line.split_whitespace())
        .filter_map(|s| s.parse().ok())
        .collect();

    let col1: Vec<u32> = numbers.iter().step_by(2).cloned().collect();
    let col2: Vec<u32> = numbers.iter().skip(1).step_by(2).cloned().collect();

    (col1, col2)
}

// We need to cast as i32 because abs() is not available for u32
fn sum_of_differences(arr1: &[u32], arr2: &[u32]) -> u32 {
    let sum_dif: i32 = arr1
        .iter()
        .zip(arr2.iter())
        .map(|(&x, &y)| (x as i32 - y as i32).abs())
        .sum();

    sum_dif as u32
}

#[aoc(day1, part1)]
fn part1(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let (left_col, right_col) = input;
    let mut lc = left_col.clone();
    let mut rc = right_col.clone();

    lc.sort();
    rc.sort();

    sum_of_differences(&lc, &rc)
}

// Make a hashmap, mapping the occurences of each number
// entry(num).or_insert(0) will get the value in the hashmap for a number
// and if it's not already in the map it will insert it with a value of 0
// super neat functions in the hashmap!
fn count_occurrences(vec: Vec<u32>) -> HashMap<u32, u32> {
    let mut counts = HashMap::new();

    for &num in &vec {
        *counts.entry(num).or_insert(0) += 1;
    }

    counts
}
#[aoc_generator(day1, part2)]
fn parse_part2(input: &str) -> (Vec<u32>, Vec<u32>) {
    parse_part1(input)
}

#[aoc(day1, part2)]
fn part2(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let (left_col, right_col) = input;
    let lc = left_col.clone();
    let rc = right_col.clone();

    let rc_hash = count_occurrences(rc);

    let similarity_score: u32 = lc.iter().map(|&x| x * rc_hash.get(&x).unwrap_or(&0)).sum();

    similarity_score
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PART1: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_part1(EXAMPLE_PART1)), 11);
    }

    const EXAMPLE_PART2: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_part2(EXAMPLE_PART2)), 31);
    }
}
