use std::error::Error;

#[aoc_generator(day3)]
pub fn generator_part1(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|x| x.chars().into_iter().collect())
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Vec<Vec<char>>) -> Result<usize, Box<dyn Error>> {
    Ok(go_down_slope(3, 1, input).unwrap())
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Vec<Vec<char>>) -> Result<usize, Box<dyn Error>> {
    let values: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut result = 1;
    let new: Vec<()> = values
        .iter()
        .map(|val| go_down_slope(val.0, val.1, input))
        .map(|val| result = val.unwrap() * result)
        .collect();
    Ok(result)
}

pub fn go_down_slope(
    x_step: usize,
    y_step: usize,
    input: &Vec<Vec<char>>,
) -> Result<usize, Box<dyn Error>> {
    let (mut x, mut y) = (0usize, 0usize);
    let mut hits: usize = 0;
    while y < input.len() - 1 {
        x += x_step;
        y += y_step;
        x = x % input[0].len();
        if input[y][x].eq(&'#') {
            hits += 1;
        }
    }
    Ok(hits)
}

#[cfg(test)]
pub mod tests {
    use crate::day3::{generator_part1, solve_part1};

    #[test]
    fn test01() {
        let input = generator_part1(
            r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#,
        );
        assert_eq!(7, solve_part1(&input).unwrap());
    }
}
