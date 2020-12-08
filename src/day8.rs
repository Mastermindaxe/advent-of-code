use self::regex::Regex;
use core::fmt;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Display, Formatter};

extern crate regex;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Instr {
    ACC(isize),
    JMP(isize),
    NOP(isize),
}

impl Display for Instr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[aoc_generator(day8)]
pub fn generator_part1(input: &str) -> Vec<Instr> {
    let mut result = Vec::with_capacity(input.lines().count());
    for x in input.lines() {
        match x.get(0..3).unwrap() {
            "acc" => result.push(Instr::ACC(x.get(4..).unwrap().parse().unwrap())),
            "jmp" => result.push(Instr::JMP(x.get(4..).unwrap().parse().unwrap())),
            "nop" => result.push(Instr::NOP(x.get(4..).unwrap().parse().unwrap())),
            _ => unreachable!(),
        }
    }
    result
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[Instr]) -> Result<isize, Box<dyn Error>> {
    Ok(solve_for_accumulator(input).unwrap().0)
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[Instr]) -> Result<isize, Box<dyn Error>> {
    let all_list = generate_permutations(input);
    for x in all_list {
        let result = solve_for_accumulator(&x).unwrap();
        if result.1 {
            return Ok(result.0);
        }
    }
    unreachable!()
}

pub fn solve_for_accumulator(input: &[Instr]) -> Result<(isize, bool), Box<dyn Error>> {
    let mut accumulator: isize = 0;

    let mut visited_operations: HashSet<isize> = HashSet::with_capacity(input.len());
    let mut pointer: isize = 0;
    while !visited_operations.contains(&pointer) {
        let instr = &input.get(pointer as usize);
        if instr.is_none() {
            return Ok((accumulator, true));
        }
        let instr = instr.unwrap();
        visited_operations.insert(pointer);
        match instr {
            Instr::ACC(x) => {
                accumulator += x;
                pointer += 1;
            }
            Instr::JMP(x) => pointer += x,
            Instr::NOP(_) => pointer += 1,
        }
    }

    Ok((accumulator, false))
}

pub fn generate_permutations(input: &[Instr]) -> Vec<Vec<Instr>> {
    let mut perms: Vec<Vec<Instr>> = Vec::new();
    for (pointer, instr) in input.iter().enumerate() {
        match instr {
            Instr::ACC(_) => (),
            Instr::JMP(x) => {
                let mut new_perm: Vec<Instr> = Vec::with_capacity(input.len());
                unsafe {
                    new_perm.set_len(input.len());
                }
                new_perm.copy_from_slice(input);
                new_perm[pointer] = Instr::NOP(*x);
                perms.push(new_perm);
            }
            Instr::NOP(x) => {
                let mut new_perm: Vec<Instr> = Vec::with_capacity(input.len());
                unsafe {
                    new_perm.set_len(input.len());
                }
                new_perm.copy_from_slice(input);
                new_perm[pointer] = Instr::JMP(*x);
                perms.push(new_perm);
            }
        }
    }
    perms.push(Vec::from(input));
    perms
}

pub fn is_valid_regex(input: &str) -> bool {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^([a-f]|[0-9]){6}$").unwrap();
    }
    REGEX.is_match(input)
}

#[cfg(test)]
pub mod tests {
    use crate::day8::Instr::{ACC, JMP, NOP};
    use crate::day8::{generate_permutations, generator_part1, solve_part1};

    lazy_static! {
        static ref INPUT: String = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#
            .to_string();
    }

    #[test]
    fn test_generator_01() {
        assert_eq!(
            vec![NOP(0), ACC(1), JMP(4), ACC(3), JMP(-3)],
            generator_part1(
                r#"nop +0
acc +1
jmp +4
acc +3
jmp -3"#
            )
        )
    }

    #[test]
    fn test_solver_01() {
        assert_eq!(5, solve_part1(&generator_part1(INPUT.as_str())).unwrap());
    }

    #[test]
    fn test_perms_01() {
        assert_eq!(
            vec![vec![JMP(4)], vec![NOP(4)]],
            generate_permutations(&*vec![NOP(4)])
        );
    }
}
