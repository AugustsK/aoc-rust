use crate::{Coordinate, SolutionPair};
use std::collections::{HashMap, HashSet};
use std::ops::Index;
use std::slice::Iter;

#[derive(Debug)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [
            Direction::Up,
            Direction::UpRight,
            Direction::Right,
            Direction::DownRight,
            Direction::Down,
            Direction::DownLeft,
            Direction::Left,
            Direction::UpLeft,
        ];
        DIRECTIONS.iter()
    }
}

fn get_xmas_chars() -> [char; 4] {
    ['X', 'M', 'A', 'S']
}

fn input_to_char_map(input: &str) -> (HashSet<Coordinate>, HashMap<Coordinate, char>) {
    let mut char_map: HashMap<Coordinate, char> = HashMap::new();
    let mut start_char_set: HashSet<Coordinate> = HashSet::new();
    let char_whitelist = get_xmas_chars();

    for (pos_y, line) in input.lines().enumerate() {
        for (pos_x, ch) in line.chars().enumerate() {
            if char_whitelist.contains(&ch) {
                char_map.insert((pos_x, pos_y), ch);

                if ch == 'X' {
                    start_char_set.insert((pos_x, pos_y));
                }
            }
        }
    }

    (start_char_set, char_map)
}

fn input_to_char_map_part_two(input: &str) -> (HashSet<Coordinate>, HashMap<Coordinate, char>) {
    let mut char_map: HashMap<Coordinate, char> = HashMap::new();
    let mut start_char_set: HashSet<Coordinate> = HashSet::new();
    let char_whitelist = ['M', 'A', 'S'];

    for (pos_y, line) in input.lines().enumerate() {
        for (pos_x, ch) in line.chars().enumerate() {
            if char_whitelist.contains(&ch) {
                char_map.insert((pos_x, pos_y), ch);

                if ch == 'A' {
                    start_char_set.insert((pos_x, pos_y));
                }
            }
        }
    }

    (start_char_set, char_map)
}

fn move_coordinate_in_direction(
    coordinate: &Coordinate,
    direction: &Direction,
) -> Option<Coordinate> {
    let (pos_x, pos_y) = coordinate;

    match direction {
        Direction::Up => {
            if 0 + pos_y == 0 {
                return None;
            }

            Some((pos_x + 0, pos_y - 1))
        }
        Direction::UpRight => {
            if 0 + pos_y == 0 {
                return None;
            }

            Some((pos_x + 1, pos_y - 1))
        }
        Direction::Right => Some((pos_x + 1, pos_y + 0)),
        Direction::DownRight => Some((pos_x + 1, pos_y + 1)),
        Direction::Down => Some((pos_x + 0, pos_y + 1)),
        Direction::DownLeft => {
            if 0 + pos_x == 0 {
                return None;
            }

            Some((pos_x - 1, pos_y + 1))
        }
        Direction::Left => {
            if 0 + pos_x == 0 {
                return None;
            }

            Some((pos_x - 1, pos_y + 0))
        }
        Direction::UpLeft => {
            if 0 + pos_y == 0 {
                return None;
            }

            if 0 + pos_x == 0 {
                return None;
            }

            Some((pos_x - 1, pos_y - 1))
        }
    }
}

fn check_if_xmas_in_map(
    cur_char_index: usize,
    coordinate: &Coordinate,
    direction: &Direction,
    char_map: &HashMap<Coordinate, char>,
    char_whitelist: Vec<char>,
) -> bool {
    if char_whitelist.len() == cur_char_index + 1 {
        return true;
    }

    let expected_next_char = char_whitelist.index(cur_char_index + 1);

    if let Some(next_coordinate_option) = move_coordinate_in_direction(&coordinate, direction) {
        if let Some(next_char_from_map) = char_map.get(&next_coordinate_option) {
            if next_char_from_map == expected_next_char {
                return check_if_xmas_in_map(
                    cur_char_index + 1,
                    &next_coordinate_option,
                    direction,
                    char_map,
                    char_whitelist,
                );
            }
        }
    }

    false
}

fn solve_part_one(input: &str) -> u64 {
    let mut result: u64 = 0;
    let (start_positions, char_map) = input_to_char_map(input);

    for start_position in start_positions {
        for direction in Direction::iterator() {
            if check_if_xmas_in_map(
                0,
                &start_position,
                direction,
                &char_map,
                get_xmas_chars().to_vec(),
            ) {
                result += 1;
            }
        }
    }

    result
}

fn solve_part_two(input: &str) -> u64 {
    let mut result: u64 = 0;
    let (start_positions, char_map) = input_to_char_map_part_two(input);

    for start_position in start_positions {
        let down_variant_a = check_if_xmas_in_map(
            0,
            &start_position,
            &Direction::UpLeft,
            &char_map,
            ['A', 'M'].to_vec(),
        ) && check_if_xmas_in_map(
            0,
            &start_position,
            &Direction::DownRight,
            &char_map,
            ['A', 'S'].to_vec(),
        );
        let down_variant_b = !down_variant_a
            && check_if_xmas_in_map(
                0,
                &start_position,
                &Direction::UpLeft,
                &char_map,
                ['A', 'S'].to_vec(),
            )
            && check_if_xmas_in_map(
                0,
                &start_position,
                &Direction::DownRight,
                &char_map,
                ['A', 'M'].to_vec(),
            );
        let down_is_valid = down_variant_a || down_variant_b;

        if !down_is_valid {
            continue;
        }

        let up_variant_a = check_if_xmas_in_map(
            0,
            &start_position,
            &Direction::DownLeft,
            &char_map,
            ['A', 'M'].to_vec(),
        ) && check_if_xmas_in_map(
            0,
            &start_position,
            &Direction::UpRight,
            &char_map,
            ['A', 'S'].to_vec(),
        );
        let up_variant_b = !up_variant_a
            && check_if_xmas_in_map(
                0,
                &start_position,
                &Direction::DownLeft,
                &char_map,
                ['A', 'S'].to_vec(),
            )
            && check_if_xmas_in_map(
                0,
                &start_position,
                &Direction::UpRight,
                &char_map,
                ['A', 'M'].to_vec(),
            );

        if down_is_valid && (up_variant_a || up_variant_b) {
            result += 1;
        }
    }

    result
}

pub fn solve(input: String) -> SolutionPair {
    (
        format!("{}", solve_part_one(&input)),
        format!("{}", solve_part_two(&input)),
    )
}
