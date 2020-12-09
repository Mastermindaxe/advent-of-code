use self::regex::Regex;
use std::collections::HashSet;
use std::error::Error;

extern crate regex;

#[aoc_generator(day9)]
pub fn generator_part1(input: &str) -> Vec<usize> {
    let mut result = Vec::with_capacity(input.lines().count());
    for x in input.lines() {
        result.push(x.parse().unwrap());
    }
    result
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[usize]) -> Result<usize, Box<dyn Error>> {
    return solve_with_preamble(input, 25);
}

pub fn solve_with_preamble(input: &[usize], preamble: usize) -> Result<usize, Box<dyn Error>> {
    let mut i = preamble;
    while let Some(curr) = input.get(i) {
        if !is_two_sum(input.get((i - preamble)..i).unwrap(), input[i]) {
            return Ok(*curr);
        }
        i += 1;
    }
    unreachable!();
}

pub fn is_two_sum(input: &[usize], number: usize) -> bool {
    for x in input {
        for y in input {
            if x + y == number && x != &number && y != &number {
                return true;
            }
        }
    }
    false
}

pub fn get_any_sum(input: &mut [usize], number: usize) -> HashSet<usize> {
    let mut set = HashSet::new();
    input.sort_unstable();
    input.reverse();

    let mut curr_sum = 0;
    let mut i = 0;
    while curr_sum < number {
        let curr = input[i];
        if curr + curr_sum < number {
            curr_sum += curr;
            set.insert(curr);
        } else if curr + curr_sum == number {
            set.insert(curr);
            return set;
        } else if curr + curr_sum > number {
        }
        i += 1;
    }

    set
}

pub fn is_valid_regex(input: &str) -> bool {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^([a-f]|[0-9]){6}$").unwrap();
    }
    REGEX.is_match(input)
}

#[cfg(test)]
pub mod tests {
    use crate::day9::{generator_part1, is_two_sum, solve_with_preamble};

    #[test]
    fn test_solver_01() {
        assert_eq!(
            127,
            solve_with_preamble(
                &*generator_part1(
                    r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"#
                ),
                5
            )
            .unwrap()
        );
    }

    #[test]
    fn test_two_sum01() {
        let array = [1, 2, 3, 4, 5, 6, 7];
        assert!(is_two_sum(&array, 8));

        let array = [35, 20, 15, 25, 47];
        assert!(is_two_sum(&array, 40));
    }
}
