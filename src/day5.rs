use self::regex::Regex;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::error::Error;
use std::slice::SliceIndex;

extern crate regex;

type BinarySearch = Vec<bool>;

#[aoc_generator(day5)]
pub fn generator_part1(input: &str) -> Vec<(BinarySearch, BinarySearch)> {
    let mut result = Vec::new();
    for x in input.lines() {
        let mut first_search = BinarySearch::new();
        for i in x.get(..(x.len() - 3)).unwrap().chars() {
            if i == 'B' {
                first_search.push(true);
            } else if i == 'F' {
                first_search.push(false);
            } else {
                panic!("Found {}", i)
            }
        }
        let mut second_search = BinarySearch::new();
        for i in x.get((x.len() - 3)..).unwrap().chars() {
            if i == 'R' {
                second_search.push(true);
            } else if i == 'L' {
                second_search.push(false);
            } else {
                panic!("Found {}", i)
            }
        }
        result.push((first_search, second_search));
    }
    result
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[(BinarySearch, BinarySearch)]) -> Result<usize, Box<dyn Error>> {
    let mut result: usize = 0;
    for x in input {
        let value = binary_search_in(127, &x.0) * 8 + binary_search_in(7, &x.1);
        if value > result {
            result = value;
        }
    }
    Ok(result)
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[(BinarySearch, BinarySearch)]) -> Result<usize, Box<dyn Error>> {
    let mut result: usize = 0;
    let mut set: BTreeSet<usize> = BTreeSet::new();
    for x in input {
        let value = binary_search_in(127, &x.0) * 8 + binary_search_in(7, &x.1);
        set.insert(value);
    }
    let mut second_set: BTreeSet<usize> = BTreeSet::new();
    for i in 0..=(127 * 8) {
        second_set.insert(i);
    }

    dbg!(second_set.difference(&set).cloned().collect::<Vec<usize>>());

    Ok(result)
}

pub fn binary_search_in(upper: usize, vector: &BinarySearch) -> usize {
    let mut range = 0..=upper;
    let mut iter = vector.iter();
    while let Some(&x) = iter.next() {
        if x {
            range = (range.start() + (range.end() - range.start()) / 2)..=*range.end();
        } else {
            range = *range.start()..=(range.start() + ((range.end() - range.start()) / 2));
        }
    }
    *range.end()
}

pub fn is_valid_regex(input: &str) -> bool {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^([a-f]|[0-9]){6}$").unwrap();
    }
    REGEX.is_match(input)
}

#[cfg(test)]
pub mod tests {
    use crate::day5::{binary_search_in, generator_part1};

    #[test]
    fn test01() {
        assert_eq!(
            (
                vec![true, false, false, false, true, true, false],
                vec![true, true, true]
            ),
            generator_part1("BFFFBBFRRR")[0]
        );
    }

    #[test]
    fn test02() {
        assert_eq!(
            44,
            binary_search_in(127, &vec![false, true, false, true, true, false, false])
        )
    }

    #[test]
    fn test03() {
        assert_eq!(5, binary_search_in(7, &vec![true, false, true]))
    }
}
