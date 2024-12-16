use aoc_runner_derive::{aoc, aoc_generator};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
enum Instruction {
    Mul { a: i32, b: i32 },
    Do,
    Dont,
}

#[derive(Debug)]
struct TokenPattern {
    pattern: &'static str,
    skip_chars: usize,
    parse_fn: fn(&mut Peekable<Chars>) -> Option<Instruction>,
}

const MUL_PATTERN: TokenPattern = TokenPattern {
    pattern: "mul(",
    skip_chars: 4,
    parse_fn: |chars| {
        // Parse first number until first non-digit is reached
        // The first non-digit character afterwards should be a comma
        // peek to not consume the next character in case of return
        // this next character could be the start of a new instruction
        let first_num = parse_number(chars);
        if chars.peek() != Some(&',') {
            return None;
        }
        chars.next();
        let second_num = parse_number(chars);
        if chars.peek() != Some(&')') {
            return None;
        }
        chars.next();
        match (first_num, second_num) {
            (Some(a), Some(b)) => Some(Instruction::Mul { a, b }),
            _ => None,
        }
    },
};

const DO_PATTERN: TokenPattern = TokenPattern {
    pattern: "do()",
    skip_chars: 4,
    parse_fn: |_chars| Some(Instruction::Do),
};

const DONT_PATTERN: TokenPattern = TokenPattern {
    pattern: "don't()",
    skip_chars: 7,
    parse_fn: |_chars| Some(Instruction::Dont),
};

fn tokenize(input: &str, patterns: &[TokenPattern]) -> Vec<Instruction> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(_) = chars.peek() {
        let remaining: String = chars.clone().collect();
        let matched = patterns
            .iter()
            .find(|p| remaining.starts_with(p.pattern))
            .and_then(|pattern| {
                chars.nth(pattern.skip_chars - 1);
                (pattern.parse_fn)(&mut chars)
            });

        if let Some(token) = matched {
            tokens.push(token);
        } else {
            chars.next();
        }
    }
    tokens
}

fn parse_number(chars: &mut Peekable<Chars>) -> Option<i32> {
    let mut num_str = String::new();

    // Collect consecutive digits until non-digit is reached
    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_digit() {
            num_str.push(chars.next()?);
        } else {
            break;
        }
    }

    // Ensure the resulting string number can be parsed
    num_str.parse().ok()
}

#[aoc_generator(day3, part1)]
fn parse_part1(input: &str) -> Vec<Instruction> {
    let patterns: &[TokenPattern] = &[MUL_PATTERN];
    tokenize(input, patterns)
}

#[aoc(day3, part1)]
fn part1(input: &Vec<Instruction>) -> i32 {
    input
        .iter()
        .filter_map(|op| match op {
            Instruction::Mul { a, b } => Some(a * b),
            _ => None,
        })
        .sum()
}

#[aoc_generator(day3, part2)]
fn parse_part2(input: &str) -> Vec<Instruction> {
    let patterns: &[TokenPattern] = &[MUL_PATTERN, DO_PATTERN, DONT_PATTERN];
    tokenize(input, patterns)
}

#[aoc(day3, part2)]
fn part2(input: &Vec<Instruction>) -> i32 {
    let mut enabled: i32 = 1;
    input
        .iter()
        .filter_map(|op| match op {
            Instruction::Mul { a, b } => Some(enabled * a * b),
            Instruction::Do => {
                enabled = 1;
                None
            }
            Instruction::Dont => {
                enabled = 0;
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_part1(EXAMPLE_INPUT1)), 161);
    }

    const EXAMPLE_INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_part2(EXAMPLE_INPUT2)), 48);
    }
}
