extern crate regex;

use std::error::Error;

use self::regex::Regex;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct MaskWithInstr {
    mask: String,
    instr: Vec<Instruction>,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Instruction {
    reg: usize,
    dec_value: usize,
}

#[aoc_generator(day14)]
pub fn generator_part1(input: &str) -> Vec<MaskWithInstr> {
    let mut curr_block = MaskWithInstr {
        mask: "".to_string(),
        instr: vec![],
    };
    let mut result = Vec::new();
    input
        .lines()
        .map(|line| {
            let mut it = line.split(" = ");
            match it.next().unwrap() {
                "mask" => {
                    result.push(curr_block.clone());
                    curr_block = MaskWithInstr {
                        mask: it.next().unwrap().to_string(),
                        instr: vec![],
                    }
                }
                x => curr_block.instr.push(Instruction {
                    reg: x.replace("mem[", "").replace("]", "").parse().unwrap(),
                    dec_value: it.next().unwrap().parse().unwrap(),
                }),
            }
        })
        .count();
    result.push(curr_block);
    result.get(1..).unwrap().to_vec()
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &[MaskWithInstr]) -> Result<usize, Box<dyn Error>> {
    Ok(4)
}

pub fn apply_mask_to_value(mask: &str, value: &usize) -> usize {
    // todo: Figure out how to construct n-bit integer
    let mut value = value;
    for c in mask.chars() {
        value.rotate_left(1);
    }
    4
}

pub fn is_valid_regex(input: &str) -> bool {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^([a-f]|[0-9]){6}$").unwrap();
    }
    REGEX.is_match(input)
}

#[cfg(test)]
pub mod tests {
    use crate::day14::generator_part1;

    #[test]
    fn parse_test01() {
        let string_to_parse = r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"#;
        assert_eq!(101, generator_part1(string_to_parse)[0].instr[1].dec_value);
    }
}
