use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2, part1)]
fn parse_part1(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect()
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(_input: &[Vec<u32>]) -> u32 {
    4
}

#[aoc_generator(day2, part2)]
fn parse_part2(input: &str) -> Vec<Vec<u32>> {
    parse_part1(input)
}

#[aoc(day2, part2)]
fn part2(_input: &[Vec<u32>]) -> u32 {
    5
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
        assert_eq!(part2(&parse_part2(EXAMPLE_INPUT)), 31);
    }
}
