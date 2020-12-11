extern crate regex;

use std::error::Error;

use self::regex::Regex;
use crate::day11::State::*;

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
pub enum State {
    FLOOR,
    EMPTY,
    OCCUPIED,
}

#[aoc_generator(day11)]
pub fn generator_part1(input: &str) -> Vec<Vec<State>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => State::FLOOR,
                    'L' => State::EMPTY,
                    '#' => State::OCCUPIED,
                    _ => unreachable!(),
                })
                .collect::<Vec<State>>()
        })
        .collect::<Vec<Vec<State>>>()
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Vec<Vec<State>>) -> Result<usize, Box<dyn Error>> {
    let mut curr_seats = input.clone();
    loop {
        let (new_seats, has_changed) = change_seats(&curr_seats);
        if !has_changed {
            return Ok(new_seats
                .iter()
                .flat_map(|row| row.iter().filter(|seat| seat == &&OCCUPIED))
                .count());
        }
        curr_seats = new_seats;
    }
}

pub fn change_seats(input: &Vec<Vec<State>>) -> (Vec<Vec<State>>, bool) {
    let mut some_changed = false;
    (
        input
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(ii, _seat)| {
                        let (state, is_changed) = get_new_state_for_seat(ii, i, &input);
                        if is_changed {
                            some_changed = true;
                        }
                        state
                    })
                    .collect::<Vec<State>>()
            })
            .collect::<Vec<Vec<State>>>(),
        some_changed,
    )
}

pub fn get_new_state_for_seat(x: usize, y: usize, input: &Vec<Vec<State>>) -> (State, bool) {
    let mut count_occupied = 0;
    let curr_state = &input[y][x];
    for i in y.checked_sub(1).unwrap_or(y)..=y + 1 {
        if let Some(_row) = input.get(i) {
            for ii in x.checked_sub(1).unwrap_or(x)..=x + 1 {
                if let Some(column) = input.get(i).unwrap().get(ii) {
                    if i == y && ii == x {
                        continue;
                    }
                    if column == &OCCUPIED {
                        count_occupied += 1;
                    }
                }
            }
        }
    }
    match curr_state {
        &EMPTY if count_occupied == 0 => {
            return (OCCUPIED, true);
        }
        &OCCUPIED if count_occupied >= 4 => {
            return (EMPTY, true);
        }
        _ => {}
    };
    (curr_state.clone(), false)
}

pub fn is_valid_regex(input: &str) -> bool {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^([a-f]|[0-9]){6}$").unwrap();
    }
    REGEX.is_match(input)
}

#[cfg(test)]
pub mod tests {
    use crate::day11::State::{EMPTY, FLOOR, OCCUPIED};
    use crate::day11::{change_seats, generator_part1, get_new_state_for_seat, solve_part1};

    #[test]
    pub fn test01() {
        let string = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#;
        let parsed = generator_part1(string);
        assert_eq!(37, solve_part1(&parsed).unwrap());
    }

    #[test]
    pub fn test_one_state_change() {
        let string = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#;
        let parsed = generator_part1(string);
        let new_seats = change_seats(&parsed);
        assert_eq!(
            vec![
                OCCUPIED, FLOOR, OCCUPIED, OCCUPIED, FLOOR, OCCUPIED, OCCUPIED, FLOOR, OCCUPIED,
                OCCUPIED
            ],
            new_seats.0[0]
        );
        //assert_eq!(true, change_seats(&new_seats.0).1);
        assert_eq!(
            vec![OCCUPIED, FLOOR, EMPTY, EMPTY, FLOOR, EMPTY, OCCUPIED, FLOOR, OCCUPIED, OCCUPIED],
            change_seats(&new_seats.0).0[0]
        );
    }

    #[test]
    pub fn test_one_seat_change() {
        let string = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#;
        let parsed = generator_part1(string);
        assert_eq!(OCCUPIED, get_new_state_for_seat(2, 0, &parsed).0);
        let new_seats = change_seats(&parsed);
        let (new_state_for_seat, has_changed) = get_new_state_for_seat(2, 0, &new_seats.0);
        assert_eq!(true, has_changed);
        assert_eq!(EMPTY, new_state_for_seat);
    }
}
