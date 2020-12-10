use std::ops::BitXor;

pub struct Input {
    lower: usize,
    upper: usize,
    ch: char,
    pass: String,
}

#[aoc_generator(day2)]
pub fn generator_part1(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(|x| {
            let mut result = Input {
                lower: 0,
                upper: 0,
                ch: '.',
                pass: "".to_string(),
            };
            let split_at_hyphen: Vec<&str> = x.split('-').collect();
            result.lower = split_at_hyphen.get(0).unwrap().parse().unwrap();

            let split_at_whitespace: Vec<&str> =
                split_at_hyphen.get(1).unwrap().split(' ').collect();
            result.upper = split_at_whitespace.get(0).unwrap().parse().unwrap();
            result.ch = split_at_whitespace
                .get(1)
                .unwrap()
                .chars()
                .into_iter()
                .next()
                .unwrap();
            result.pass = split_at_whitespace.get(2).unwrap().to_string();

            return result;
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Input]) -> usize {
    // How many passwords are valid?
    input
        .iter()
        .map(|item| is_valid_pw(item))
        .filter(|item| *item)
        .count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Input]) -> usize {
    // How many passwords are valid?
    input
        .iter()
        .map(|item| is_new_valid_pw(item))
        .filter(|item| *item)
        .count()
}

pub fn is_valid_pw(input: &Input) -> bool {
    let mut counter: usize = 0;
    for c in input.pass.chars() {
        if c == input.ch {
            counter += 1;
        }
    }
    return counter >= input.lower && counter <= input.upper;
}

pub fn is_new_valid_pw(input: &Input) -> bool {
    return input
        .pass
        .get(input.lower - 1..input.lower)
        .unwrap()
        .eq(&input.ch.to_string())
        .bitxor(
            input
                .pass
                .get(input.upper - 1..input.upper)
                .unwrap()
                .eq(&input.ch.to_string()),
        );
}

#[cfg(test)]
pub mod tests {
    use crate::day2::{Input, is_valid_pw};

    #[test]
    fn test01() {
        assert_eq!(
            is_valid_pw(&Input {
                lower: 1usize,
                upper: 3usize,
                ch: 'a',
                pass: "abcde".to_string(),
            }),
            true
        );
    }

    #[test]
    fn test02() {
        assert_eq!(
            is_valid_pw(&Input {
                lower: 1usize,
                upper: 3usize,
                ch: 'a',
                pass: "abcde".to_string(),
            }),
            true
        );
    }
}
