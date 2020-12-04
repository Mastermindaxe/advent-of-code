use self::regex::Regex;
use std::collections::HashMap;
use std::error::Error;

extern crate regex;

#[aoc_generator(day4)]
pub fn generator_part1(input: &str) -> Vec<HashMap<String, String>> {
    let mut maps: Vec<HashMap<String, String>> = Vec::new();
    let mut curr_map: HashMap<String, String> = HashMap::new();
    for x in input.lines() {
        if x == "" {
            maps.push(curr_map.clone());
            curr_map = HashMap::new();
            continue;
        } else {
            let _ = x
                .split_whitespace()
                .map(|y| {
                    let mut iter = y.split(":");
                    curr_map.insert(
                        iter.next().unwrap().to_string(),
                        iter.next().unwrap().to_string(),
                    );
                })
                .count();
        }
    }
    maps.push(curr_map.clone());
    maps
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Vec<HashMap<String, String>>) -> Result<usize, Box<dyn Error>> {
    let mut count = 0usize;
    for value in input {
        if is_in_map(value) {
            count += 1;
        }
    }
    Ok(count)
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Vec<HashMap<String, String>>) -> Result<usize, Box<dyn Error>> {
    let mut count = 0usize;
    for value in input {
        if is_in_map2(value) {
            count += 1;
        }
    }
    Ok(count)
}

pub fn is_in_map(input: &HashMap<String, String>) -> bool {
    let specs = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for spec in specs {
        let value = input.get(spec);
        if value.is_none() {
            return false;
        }
    }
    true
}

pub fn is_in_map2(input: &HashMap<String, String>) -> bool {
    let specs = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for &spec in specs.iter() {
        let value = input.get(spec);
        if value.is_none() {
            return false;
        }
        let value = value.unwrap();
        match spec {
            "byr" => {
                if !(1920..=2002).contains(&value.parse::<i32>().unwrap()) {
                    return false;
                }
            }
            "iyr" => {
                if !(2010..=2020).contains(&value.parse::<i32>().unwrap()) {
                    return false;
                }
            }
            "eyr" => {
                if !(2020..=2030).contains(&value.parse::<i32>().unwrap()) {
                    return false;
                }
            }
            "hgt" => {
                let height = value.get((value.len() - 2)..value.len());
                if height.is_none() {
                    return false;
                }
                if height.unwrap().eq("cm") {
                    let parsed = value
                        .get(..value.len() - 2)
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                    if !(150..=193).contains(&parsed) {
                        return false;
                    };
                }
                if height.unwrap().eq("in") {
                    let parsed = value
                        .get(..value.len() - 2)
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                    if !(59..=76).contains(&parsed) {
                        return false;
                    };
                }
            }
            "hcl" => {
                if !(value.get(0..1).unwrap().eq("#")) {
                    return false;
                };
                if !(is_valid_regex(value.get(1..value.len()).unwrap())) {
                    return false;
                }
            }
            "ecl" => {
                if !(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&**value)) {
                    return false;
                }
            }
            "pid" => {
                if value.len() != 9 {
                    return false;
                }
                if value.parse::<u128>().is_err() {
                    return false;
                }
            }
            _ => {}
        }
    }
    true
}

pub fn is_valid_regex(input: &str) -> bool {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"([a-f]|[0-9]){6}").unwrap();
    }
    REGEX.is_match(input)
}

#[cfg(test)]
pub mod tests {
    use crate::day4::{generator_part1, is_valid_regex, solve_part1, solve_part2};

    #[test]
    fn test01() {
        let parsed = generator_part1(
            r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#,
        );
        assert_eq!(2, solve_part1(&parsed).unwrap());
    }

    #[test]
    fn test_invalid_passports() {
        let parsed = generator_part1(
            r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#,
        );
        assert_eq!(0, solve_part2(&parsed).unwrap());
    }

    #[test]
    fn test_valid_passports() {
        let parsed = generator_part1(
            r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#,
        );
        assert_eq!(4, solve_part2(&parsed).unwrap());
    }

    #[test]
    fn test_valid_passports2() {
        let parsed = generator_part1(
            r#"pid:087499704 hgt:60in ecl:grn iyr:2012 eyr:2030 byr:2002
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:190cm

hcl:#123abc
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:brn
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:000000001"#,
        );
        assert_eq!(4, solve_part2(&parsed).unwrap());
    }

    #[test]
    fn test_invalid_passports2() {
        let parsed = generator_part1(
            r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:2003
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:190in

hcl:#123abz
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:wat byr:1944 eyr:2021 pid:093154719

iyr:2010 hgt:190 hcl:#b6652a ecl:wat byr:1944 eyr:2021 pid:093154719

iyr:2010 hgt:158cm hcl:123abc ecl:wat byr:1944 eyr:2021 pid:093154719

iyr:2010 hgt:158cm hcl:#b6652a ecl:wat byr:1944 eyr:2021 pid:0123456789"#,
        );
        assert_eq!(0, solve_part2(&parsed).unwrap());
    }

    #[test]
    fn test_regex01() {
        assert_eq!(true, is_valid_regex("623a2f"));
    }
}
