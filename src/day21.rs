extern crate regex;

use self::regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error::Error;

type Ingredient = String;
type Allergen = String;

#[aoc_generator(day21)]
pub fn generator_part1(input: &str) -> Vec<(HashSet<Ingredient>, HashSet<Allergen>)> {
    input
        .lines()
        .map(|line| {
            let replace = line
                .replace(" (contains ", "|")
                .replace(",", "")
                .replace(")", "");
            let split = replace.split("|").collect::<Vec<&str>>();
            let (first, second) = (split[0], split[1]);
            let mut first_hash: HashSet<Ingredient> = HashSet::new();
            first.split_whitespace().for_each(|elem| {
                first_hash.insert(elem.to_string());
            });
            let mut second_hash: HashSet<Allergen> = HashSet::new();
            second.split_whitespace().for_each(|elem| {
                second_hash.insert(elem.to_string());
            });
            (first_hash, second_hash)
        })
        .collect()
}

#[aoc(day21, part1)]
pub fn solve_part1(
    input: &[(HashSet<Ingredient>, HashSet<Allergen>)],
) -> Result<usize, Box<dyn Error>> {
    Ok(4)
}

pub fn create_mapping_for_allergens(
    input: &[(HashSet<Ingredient>, HashSet<Allergen>)],
) -> HashMap<Allergen, HashSet<HashSet<Ingredient>>> {
    let mut mapping: HashMap<Allergen, HashSet<HashSet<Ingredient>>> = HashMap::new();
    for (ingredients, allergens) in input {
        for allergen in allergens {
            let entry = mapping.entry(allergen.clone()).or_default();
            // entry.insert(ingredients.clone()); TODO: Trait hash not satisfied for HashSet
        }
    }
    mapping
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
