extern crate regex;

use self::regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;

#[aoc_generator(day22)]
pub fn generator_part1(input: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let lines: Vec<&str> = input.lines().collect();
    let player1 = lines.get(1..lines.len() / 2).unwrap();
    let player1 = player1.iter().map(|s| s.parse().unwrap()).collect();
    let player2 = lines.get(lines.len() / 2 + 2..).unwrap();
    let player2 = player2.iter().map(|s| s.parse().unwrap()).collect();
    (player1, player2)
}

#[aoc(day22, part1)]
pub fn solve_part1(input: &(VecDeque<usize>, VecDeque<usize>)) -> Result<usize, Box<dyn Error>> {
    let mut player1 = input.0.clone();
    let mut player2 = input.1.clone();
    loop {
        // Match is over
        if player1.is_empty() {
            return Ok(compute_score(player2));
        } else if player2.is_empty() {
            return Ok(compute_score(player1));
        }

        // Who has the higher card?
        let player1_card = player1.pop_front().unwrap();
        let player2_card = player2.pop_front().unwrap();
        if player1_card > player2_card {
            player1.push_back(player1_card);
            player1.push_back(player2_card);
        } else {
            player2.push_back(player2_card);
            player2.push_back(player1_card);
        }
    }
}

pub fn compute_score(input: VecDeque<usize>) -> usize {
    dbg!(&input);
    let mut result = 0;
    for (i, val) in input.iter().rev().enumerate() {
        result += (i + 1) * val;
    }
    result
}

pub fn is_valid_regex(input: &str) -> bool {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^([a-f]|[0-9]){6}$").unwrap();
    }
    REGEX.is_match(input)
}

#[cfg(test)]
pub mod tests {

    #[test]
    pub fn test_parser01() {}
}
