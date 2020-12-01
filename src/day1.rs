#[aoc_generator(day1)]
pub fn generator_part1(input: &str) -> Vec<isize> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[isize]) -> isize {
    find_in_vec1(input)
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[isize]) -> isize {
    find_in_vec2(input)
}

pub fn find_in_vec1(input: &[isize]) -> isize {
    for (i, x) in input.iter().enumerate() {
        for (ii, y) in input.iter().enumerate() {
            if i == ii {
                continue;
            }
            if x + y == 2020 {
                return x * y;
            }
        }
    }
    return 0;
}

pub fn find_in_vec2(input: &[isize]) -> isize {
    for (i, x) in input.iter().enumerate() {
        for (ii, y) in input.iter().enumerate() {
            if i == ii {
                continue;
            }
            for (iii, z) in input.iter().enumerate() {
                if i == iii || i == ii {
                    continue;
                }
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }
    return 0;
}

#[cfg(test)]
pub mod tests {
    use crate::day1::{find_in_vec1, find_in_vec2};

    #[test]
    fn test01() {
        assert_eq!(
            find_in_vec1(vec![1721, 979, 366, 299, 675, 1456].as_slice()),
            514579
        );
    }

    #[test]
    fn test02() {
        assert_eq!(
            find_in_vec2(vec![1721, 979, 366, 299, 675, 1456].as_slice()),
            241861950
        );
    }
}
