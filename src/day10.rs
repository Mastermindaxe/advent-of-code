extern crate regex;

use std::error::Error;

use self::regex::Regex;

#[aoc_generator(day10)]
pub fn generator_part1(input: &str) -> Vec<usize> {
    let mut result = Vec::with_capacity(input.lines().count());
    for x in input.lines() {
        result.push(x.parse().unwrap());
    }
    result
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[usize]) -> Result<usize, Box<dyn Error>> {
    Ok(3)
}

pub fn is_valid_regex(input: &str) -> bool {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^([a-f]|[0-9]){6}$").unwrap();
    }
    REGEX.is_match(input)
}

#[cfg(test)]
pub mod tests {}
