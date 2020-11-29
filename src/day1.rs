#[aoc_generator(day1)]
pub fn generator_part1(input: &str) -> Vec<isize> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[isize]) -> isize {
    input.iter().map(|item| calculate_fuel(item)).sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[isize]) -> isize {
    input
        .iter()
        .map(|item| calculate_fuel_recursively(*item))
        .sum()
}

pub fn calculate_fuel(x: &isize) -> isize {
    ((*x as f32) / 3f32) as isize - 2
}

pub fn calculate_fuel_recursively(x: isize) -> isize {
    let mut sum: isize = 0;
    let mut curr_value: isize = x;

    while calculate_fuel(&curr_value) > 0 {
        curr_value = calculate_fuel(&curr_value);
        sum += curr_value;
    }

    sum
}

#[cfg(test)]
pub mod tests {
    use crate::day1::{calculate_fuel_recursively, solve_part1};

    #[test]
    fn day1_mass() {
        assert_eq!(solve_part1(vec![12].as_slice()), 2);
    }

    #[test]
    fn day1_part2() {
        assert_eq!(calculate_fuel_recursively(14), 2);
        assert_eq!(calculate_fuel_recursively(100756), 50346);
    }
}
