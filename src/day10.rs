extern crate regex;

use std::error::Error;

use self::regex::Regex;

#[aoc_generator(day10)]
pub fn generator_part1(input: &str) -> Vec<usize> {
    let mut result = Vec::with_capacity(input.lines().count());
    for x in input.lines() {
        result.push(x.parse().unwrap());
    }
    result.sort_unstable();
    result
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[usize]) -> Result<usize, Box<dyn Error>> {
    let mut one_difference = 0;
    let mut three_difference = 1;
    let mut first_elem = 0;
    for elem in input.iter() {
        let difference = elem - first_elem;
        if difference == 1 {
            one_difference += 1;
        } else if difference == 3 {
            three_difference += 1;
        }
        first_elem = *elem;
    }
    Ok(one_difference * three_difference)
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[usize]) -> Result<usize, Box<dyn Error>> {
    let mut traversed = Vec::new();
    traversed.push(0);
    Ok(recursive(input, &mut traversed, 0, *input.last().unwrap() + 3).unwrap())
}

pub fn recursive(
    rest: &[usize],
    already_traversed: &mut Vec<usize>,
    mut sum: usize,
    last: usize,
) -> Result<usize, Box<dyn Error>> {
    dbg!(&already_traversed);
    dbg!(&rest);
    dbg!(&sum);
    dbg!(&last);
    dbg!(&already_traversed.last().unwrap() == &&last);
    if rest.is_empty() && already_traversed.last().unwrap() + 3 == last {
        return Ok(sum + 1);
    } else if rest.is_empty() {
        return Ok(sum);
    }
    if let Some(x) = rest.get(0) {
        if x - already_traversed.last().unwrap() <= 3 {
            let mut already_traversed = already_traversed.clone();
            already_traversed.push(*x);
            sum += recursive(rest.get(1..).unwrap(), &mut already_traversed, sum, last)?;
        }
    }
    if let Some(x) = rest.get(1) {
        if x - already_traversed.last().unwrap() <= 3 {
            let mut already_traversed = already_traversed.clone();
            already_traversed.push(*x);
            sum += recursive(rest.get(2..).unwrap(), &mut already_traversed, sum, last)?;
        }
    }
    if let Some(x) = rest.get(2) {
        if x - already_traversed.last().unwrap() <= 3 {
            let mut already_traversed = already_traversed.clone();
            already_traversed.push(*x);
            sum += recursive(rest.get(3..).unwrap(), &mut already_traversed, sum, last)?;
        }
    }
    return Ok(sum);
}

pub fn is_valid_regex(input: &str) -> bool {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^([a-f]|[0-9]){6}$").unwrap();
    }
    REGEX.is_match(input)
}

#[cfg(test)]
pub mod tests {
    use crate::day10::{generator_part1, solve_part1, solve_part2};

    #[test]
    fn parser_test01() {
        let unparsed = r#"16
10
15
5
1
11
7
19
6
12
4"#;
        assert_eq!(
            vec![1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19],
            generator_part1(unparsed)
        );
    }

    #[test]
    fn solver_test01() {
        let unparsed = r#"16
10
15
5
1
11
7
19
6
12
4"#;
        assert_eq!(35, solve_part1(&*generator_part1(unparsed)).unwrap());
    }

    #[test]
    fn solver_test02() {
        let unparsed = r#"16
10
15
5
1
11
7
19
6
12
4"#;
        assert_eq!(8, solve_part2(&*generator_part1(unparsed)).unwrap());
    }
}
