use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2, part1)]
fn parse_part1(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect()
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &Vec<Vec<i32>>) -> i32 {
    input.iter().filter(|row| check_row_safety(&row)).count() as i32
}

fn check_row_safety(row: &Vec<i32>) -> bool {
    // Sanity check: n_elements > 1
    if row.len() < 2 {
        return false;
    }

    let increasing = row[1] > row[0];

    row.windows(2).all(|window| {
        let diff = window[1] - window[0];

        // Ensure consistency in direction
        if increasing && diff <= 0 {
            return false;
        }
        if !increasing && diff >= 0 {
            return false;
        }

        diff.abs() <= 3
    })
}
#[aoc_generator(day2, part2)]
fn parse_part2(input: &str) -> Vec<Vec<i32>> {
    parse_part1(input)
}

// If a row is unsafe, iterate through it, remove the i-th element and check if it's safe
// This is done by iterating through the indexes of the row
// Then we create a modified row that removes that i-th index using filter_map() and then_some to
// return the row without that element
//
// Once we find a safe combination, any will return with true, otherwise false
#[aoc(day2, part2)]
fn part2(input: &[Vec<i32>]) -> i32 {
    input
        .iter()
        .filter(|row| {
            if check_row_safety(row) {
                return true;
            }

            row.iter().enumerate().any(|(i, _)| {
                // Create a new row with the i-th element removed
                let modified_row: Vec<i32> = row
                    .iter()
                    .enumerate()
                    .filter_map(|(j, &val)| (i != j).then_some(val))
                    .collect();

                // Check if the modified row is safe
                check_row_safety(&modified_row)
            })
        })
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_part1(EXAMPLE_INPUT)), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_part2(EXAMPLE_INPUT)), 4);
    }
}
