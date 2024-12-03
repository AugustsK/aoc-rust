use crate::utils::inputs;
use crate::SolutionPair;

fn solve_part_one(input: &str) -> u64 {
    let mut result: u64 = 0;
    let len = input.len();
    let mut is_mul = false;
    let mut is_second_num = false;
    let mut first_num_str = String::new();
    let mut second_num_str = String::new();
    let chars: Vec<char> = input.chars().collect();

    for pos in 3..len {
        if !is_mul && input[pos - 3..pos + 1].eq("mul(") {
            is_mul = true;
            continue;
        }

        if !is_mul {
            continue;
        }

        let char = chars[pos];

        if char == ',' {
            is_second_num = true;
            continue;
        }

        if char.is_digit(10) {
            if is_second_num {
                second_num_str.push(char);
            } else {
                first_num_str.push(char);
            }

            continue;
        }

        if char == ')' {
            let first_num = first_num_str.parse::<u64>();
            let second_num = second_num_str.parse::<u64>();

            if first_num.is_ok() && second_num.is_ok() {
                result += first_num.unwrap() * second_num.unwrap();
            }
        }

        is_mul = false;
        is_second_num = false;
        first_num_str = String::new();
        second_num_str = String::new();
    }

    result
}

fn solve_part_two(input: &str) -> u64 {
    let mut result: u64 = 0;
    let mut is_enabled = true;
    let len = input.len();
    let mut is_mul = false;
    let mut is_second_num = false;
    let mut first_num_str = String::new();
    let mut second_num_str = String::new();
    let chars: Vec<char> = input.chars().collect();

    for pos in 3..len {
        if input[pos - 3..pos + 1].eq("do()") {
            is_enabled = true;
            continue;
        }

        if pos >= 6 && input[pos - 6..pos + 1].eq("don't()") {
            is_enabled = false;
            continue;
        }

        if !is_enabled {
            continue;
        }

        if !is_mul && input[pos - 3..pos + 1].eq("mul(") {
            is_mul = true;
            continue;
        }

        if !is_mul {
            continue;
        }

        let char = chars[pos];

        if char == ',' {
            is_second_num = true;
            continue;
        }

        if char.is_digit(10) {
            if is_second_num {
                second_num_str.push(char);
            } else {
                first_num_str.push(char);
            }

            continue;
        }

        if char == ')' {
            let first_num = first_num_str.parse::<u64>();
            let second_num = second_num_str.parse::<u64>();

            if first_num.is_ok() && second_num.is_ok() {
                result += first_num.unwrap() * second_num.unwrap();
            }
        }

        is_mul = false;
        is_second_num = false;
        first_num_str = String::new();
        second_num_str = String::new();
    }

    result
}

pub fn solve(use_test_input: bool, path_to_inputs: String) -> SolutionPair {
    let input = inputs::get_input(path_to_inputs, 2024, 3, use_test_input);

    (
        format!("{}", solve_part_one(&input)),
        format!("{}", solve_part_two(&input)),
    )
}
