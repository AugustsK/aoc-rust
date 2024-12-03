use crate::utils::inputs;
use crate::SolutionPair;
use std::cmp::PartialEq;

#[derive(Debug, Enum, PartialEq)]
enum Direction {
    Ind,
    Inc,
    Dec,
}

fn solve_part_one(input: &str) -> u64 {
    let mut result: u64 = 0;

    for line in input.lines() {
        let mut direction = Direction::Ind;
        let mut prev_id: u64 = 0;
        let mut safe = true;

        for id in line.split(" ").collect::<Vec<&str>>() {
            let id_num = id.parse::<u64>().unwrap();

            if prev_id == 0 {
                prev_id = id_num;
                continue;
            }

            if direction == Direction::Ind {
                if id_num > prev_id {
                    direction = Direction::Inc;
                } else {
                    direction = Direction::Dec;
                }
            }

            let diff = id_num.abs_diff(prev_id);

            if diff < 1 || diff > 3 {
                safe = false;
                break;
            }

            if direction == Direction::Inc && id_num < prev_id {
                safe = false;
                break;
            }

            if direction == Direction::Dec && id_num > prev_id {
                safe = false;
                break;
            }

            prev_id = id_num;
        }

        if safe {
            result += 1;
        }
    }

    result
}

fn check_validity(direction: &Direction, num_a: &u64, num_b: &u64) -> bool {
    if direction == &Direction::Inc && num_a > num_b {
        return false;
    }

    if direction == &Direction::Dec && num_a < num_b {
        return false;
    }

    let diff = num_a.abs_diff(0 + num_b);

    diff >= 1 && diff <= 3
}

fn solve_part_two(input: &str) -> u64 {
    let mut result: u64 = 0;

    for line in input.lines() {
        let mut increases: u16 = 0;
        let mut decreases: u16 = 0;
        let mut prev_id: u64 = 0;
        let mut parsed_line: Vec<u64> = Vec::new();

        for id in line.split(" ").collect::<Vec<&str>>() {
            let id_num = id.parse::<u64>().unwrap();

            parsed_line.push(id_num);

            if prev_id == 0 {
                prev_id = id_num;
                continue;
            }

            if id_num > prev_id {
                increases += 1;
            }

            if id_num < prev_id {
                decreases += 1;
            }

            prev_id = id_num;
        }

        let direction;

        if increases > decreases {
            direction = Direction::Inc;
        } else {
            direction = Direction::Dec;
        }

        // Early filter out instances with too many direction changes
        if direction == Direction::Inc && decreases >= 2 || direction == Direction::Dec && increases >= 2 {
            continue;
        }

        let mut skips: i8 = 0;

        let len = parsed_line.len();
        let mut need_to_skip_next = false;

        for (pos, id_num) in parsed_line.iter().enumerate() {
            if pos + 1 == len || need_to_skip_next {
                need_to_skip_next = false;
                continue;
            }

            // Check if current is valid against next
            if !check_validity(&direction, &id_num, &parsed_line[pos + 1]) {
                let mut prev_is_valid_against_next = true;

                if pos > 0 && pos + 2 < len {
                    prev_is_valid_against_next = check_validity(&direction, &parsed_line[pos - 1], &parsed_line[pos + 1])
                }

                if pos + 2 < len {
                    need_to_skip_next = check_validity(&direction, &id_num, &parsed_line[pos + 2])
                }

                if prev_is_valid_against_next || need_to_skip_next {
                    skips += 1;
                } else {
                    skips += 2;
                    break;
                }
            }
        }

        if skips < 2 {
            result += 1;
        }
    }

    result
}

pub fn solve(use_test_input: bool, path_to_inputs: String) -> SolutionPair {
    let input = inputs::get_input(path_to_inputs, 2024, 2, use_test_input);

    (
        format!("{}", solve_part_one(&input)),
        format!("{}", solve_part_two(&input)),
    )
}
