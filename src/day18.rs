extern crate regex;

use std::error::Error;

use self::regex::Regex;
use crate::day18::Operator::{ADD, MULTIPLY};
use crate::day18::Parenthesis::{CLOSE, OPEN};
use crate::day18::Token::{LITERAL, OPERATOR, PARENTHESIS};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub enum Token {
    LITERAL(usize),
    PARENTHESIS(Parenthesis),
    OPERATOR(Operator),
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub enum Operator {
    ADD,
    MULTIPLY,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub enum Parenthesis {
    OPEN,
    CLOSE,
}

#[aoc_generator(day18)]
pub fn generator_part1(input: &str) -> Vec<Vec<Token>> {
    input
        .lines()
        .map(|line| {
            let mut result: Vec<Token> = vec![];
            line.split_whitespace().for_each(|token| {
                match token.get(0..1).unwrap().parse::<usize>() {
                    Ok(i) => result.push(LITERAL(i)),
                    _ => match match_token(token.get(0..1).unwrap()) {
                        PARENTHESIS(OPEN) => {
                            result.push(PARENTHESIS(OPEN));
                            result.push(match_token(token.get(1..).unwrap()));
                        }
                        LITERAL(x) => {}
                        _ => result.push(match_token(token.get(0..1).unwrap())),
                    },
                }
            });
            result
        })
        .collect()
}

pub fn match_token(input: &str) -> Token {
    match input.parse::<usize>() {
        Ok(i) => LITERAL(i),
        _ => match input.get(0..input.len()).unwrap() {
            "(" => PARENTHESIS(OPEN),
            ")" => PARENTHESIS(CLOSE),
            "*" => OPERATOR(MULTIPLY),
            "+" => OPERATOR(ADD),
            x => LITERAL(x.parse().unwrap()),
        },
    }
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &[Vec<Token>]) -> Result<usize, Box<dyn Error>> {
    Ok(4)
}

pub fn is_valid_regex(input: &str) -> bool {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^([a-f]|[0-9]){6}$").unwrap();
    }
    REGEX.is_match(input)
}

#[cfg(test)]
pub mod tests {
    use crate::day18::generator_part1;
    use crate::day18::Operator::{ADD, MULTIPLY};
    use crate::day18::Parenthesis::{CLOSE, OPEN};
    use crate::day18::Token::{LITERAL, OPERATOR, PARENTHESIS};

    #[test]
    pub fn test_parser01() {
        assert_eq!(
            vec![vec![
                LITERAL(2),
                OPERATOR(MULTIPLY),
                LITERAL(3),
                OPERATOR(ADD),
                PARENTHESIS(OPEN),
                LITERAL(4),
                OPERATOR(MULTIPLY),
                LITERAL(5),
                PARENTHESIS(CLOSE)
            ]],
            generator_part1("2 * 3 + (4 * 5)")
        )
    }
}
