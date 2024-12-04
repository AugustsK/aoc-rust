use crate::SolutionPair;
use std::cmp;
use std::collections::{HashMap, HashSet};

type Coordinate = (usize, usize);

fn solve_part_one(input: &str) -> u64 {
    let mut result: u64 = 0;
    let mut check_coords: HashSet<Coordinate> = HashSet::new();
    let mut maybe_part_numbers: HashMap<(usize, usize, usize), u64> = HashMap::new();

    for (pos_y, line) in input.lines().enumerate() {
        let mut current_digits_x1: usize = 0;
        let mut last_x: usize = 0;
        let mut current_digits = String::new();

        for (pos_x, ch) in line.chars().enumerate() {
            last_x = pos_x;

            if ch.is_digit(10) {
                if current_digits.len() == 0 {
                    current_digits_x1 = 0 + pos_x;
                }

                current_digits.push(ch);
            } else {
                if current_digits.len() >= 1 {
                    maybe_part_numbers.insert(
                        (current_digits_x1, pos_x - 1, pos_y),
                        current_digits.parse::<u64>().unwrap(),
                    );
                    current_digits_x1 = 0;
                    current_digits = String::new();
                }

                if ch != '.' {
                    if pos_x > 0 {
                        check_coords.insert((pos_x - 1, pos_y));
                        check_coords.insert((pos_x - 1, pos_y + 1));

                        if pos_y > 0 {
                            check_coords.insert((pos_x - 1, pos_y - 1));
                        }
                    }

                    if pos_y > 0 {
                        check_coords.insert((pos_x, pos_y - 1));
                        check_coords.insert((pos_x + 1, pos_y - 1));
                    }

                    check_coords.insert((pos_x, pos_y));
                    check_coords.insert((pos_x + 1, pos_y));
                    check_coords.insert((pos_x, pos_y + 1));
                    check_coords.insert((pos_x + 1, pos_y + 1));
                }
            }
        }

        if current_digits.len() >= 1 {
            maybe_part_numbers.insert(
                (current_digits_x1, last_x - 1, pos_y),
                current_digits.parse::<u64>().unwrap(),
            );
        }
    }

    for (start_end_coordinates, part_number) in &maybe_part_numbers {
        let x1 = start_end_coordinates.0;
        let x2 = start_end_coordinates.1;
        let y = start_end_coordinates.2;

        for x in x1..=x2 {
            if check_coords.contains(&(x, y)) {
                result += part_number;
                break;
            }
        }
    }

    result
}

fn solve_part_two(input: &str) -> u64 {
    let mut result: u64 = 0;
    let mut gear_coords: HashSet<Coordinate> = HashSet::new();
    let mut part_numbers: HashMap<usize, u64> = HashMap::new();
    let mut part_number_coords: HashMap<Coordinate, usize> = HashMap::new();
    let mut cur_part_number: usize = 0;

    for (pos_y, line) in input.lines().enumerate() {
        let mut current_digits_x1: usize = 0;
        let mut last_x: usize = 0;
        let mut current_digits = String::new();

        for (pos_x, ch) in line.chars().enumerate() {
            last_x = pos_x;

            if ch.is_digit(10) {
                if current_digits.len() == 0 {
                    current_digits_x1 = 0 + pos_x;
                }

                current_digits.push(ch);
            } else {
                if current_digits.len() >= 1 {
                    part_numbers.insert(cur_part_number, current_digits.parse::<u64>().unwrap());

                    for x in current_digits_x1..=pos_x - 1 {
                        part_number_coords.insert((x, pos_y), cur_part_number);
                    }

                    cur_part_number += 1;
                }

                current_digits_x1 = 0;
                current_digits = String::new();
            }

            if ch == '*' {
                gear_coords.insert((pos_x, pos_y));
            }
        }

        if current_digits.len() >= 1 {
            part_numbers.insert(cur_part_number, current_digits.parse::<u64>().unwrap());

            for x in current_digits_x1..=last_x - 1 {
                part_number_coords.insert((x, pos_y), cur_part_number);
            }

            cur_part_number += 1;
        }
    }

    for gear_coord in &gear_coords {
        let mut part_number_ids: HashSet<&usize> = HashSet::new();
        let x1 = cmp::max(gear_coord.0 - 1, 0);
        let x2 = gear_coord.0 + 1;
        let y1 = cmp::max(gear_coord.1 - 1, 0);
        let y2 = gear_coord.1 + 1;

        for x in x1..=x2 {
            for y in y1..=y2 {
                if part_number_coords.contains_key(&(x, y)) {
                    let part_number_id = part_number_coords.get(&(x, y));

                    if part_number_id.is_some() {
                        part_number_ids.insert(part_number_id.unwrap());
                    }
                }
            }
        }

        if part_number_ids.len() == 2 {
            let mut ratio = 1;

            for part_number_id in part_number_ids {
                let part_number = part_numbers.get(part_number_id);

                if part_number.is_some() {
                    ratio *= part_number.unwrap();
                }
            }

            result += ratio;
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
