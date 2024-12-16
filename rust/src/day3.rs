use aoc_runner_derive::{aoc, aoc_generator};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
enum Instruction {
    Mul { a: i32, b: i32 },
}

fn tokenize(input: &str) -> Vec<Instruction> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(_) = chars.peek() {
        // Get the current remaining string
        let remaining: String = chars.clone().collect();
        dbg!(&remaining);

        match remaining.as_str() {
            s if s.starts_with("mul(") => {
                // Skip 'mul('
                chars.nth(3);
                dbg!(&chars);

                // Parse first number until first non-digit is reached
                // The first non-digit character afterwards should be a comma
                let first_num = parse_number(&mut chars);
                if chars.next() != Some(',') {
                    continue;
                }

                // Now we should expect a closing parenthesis
                let second_num = parse_number(&mut chars);
                if chars.next() != Some(')') {
                    continue;
                }

                if let (Some(a), Some(b)) = (first_num, second_num) {
                    // println!("--ADD MUL({},{})", &a, &b);
                    tokens.push(Instruction::Mul { a, b });
                }
            }
            _ => {
                // Skip unrecognized characters
                chars.next();
            }
        }
    }

    tokens
}

// Helper fn: parse a number from a character iterator
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

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Instruction> {
    tokenize(input)
}

#[aoc(day3, part1)]
fn part1(input: &Vec<Instruction>) -> i32 {
    // dbg!(input);
    input
        .iter()
        .filter_map(|op| match op {
            Instruction::Mul { a, b } => Some(a * b),
            _ => None,
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &Vec<Instruction>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT)), 161);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_INPUT)), 0);
    }
}
