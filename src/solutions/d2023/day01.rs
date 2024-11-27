use std::collections::HashMap;

use crate::{SolutionPair};
use crate::utils::inputs;

fn solve_part_one(input: &str) -> i64 {
    let mut result: i64 = 0;

    for line in input.lines() {
        let chars = line.chars();
        let mut first_digit: Option<char> = None;
        let mut last_digit: Option<char> = None;

        for c in chars {
            if c.is_digit(10) {
                first_digit = Some(c);
                break;
            }
        }

        let rev_chars = line.chars().rev();

        for c in rev_chars {
            if c.is_digit(10) {
                last_digit = Some(c);
                break;
            }
        }

        let mut number_parts = String::new();

        if let Some(first_digit_value) = first_digit {
            number_parts.push(first_digit_value)
        }

        if let Some(last_digit_value) = last_digit {
            number_parts.push(last_digit_value)
        }

        let parsed = number_parts.parse::<i64>();

        if parsed.is_ok() {
            result = result + parsed.unwrap();
        }
    }

    result
}

fn check_temp_match(maybe_match: &str) -> Option<char> {
    let map = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    for (key, value) in &map {
        if let Some(_) = maybe_match.find(key) {
            return Some(*value);
        }
    }

    None
}

fn solve_part_two(input: &str) -> i64 {
    let mut result: i64 = 0;

    for line in input.lines() {
        // 2jxzhlkhdktxfjjleightdfpgfxjv
        let chars = line.chars();
        let mut temp_match = String::new();
        let mut first_digit: Option<char> = None;
        let mut last_digit: Option<char> = None;

        for c in chars {
            if c.is_digit(10) {
                first_digit = Some(c);
                break;
            }

            temp_match.push(c);

            if let Some(maybe_first_digit) = check_temp_match(&temp_match) {
                first_digit = Some(maybe_first_digit);
                break;
            }
        }

        temp_match = String::new();
        let rev_chars = line.chars().rev();

        for c in rev_chars {
            if c.is_digit(10) {
                last_digit = Some(c);
                break;
            }

            temp_match.push(c);

            if let Some(maybe_last_digit) = check_temp_match(&temp_match.chars().rev().collect::<String>()) {
                last_digit = Some(maybe_last_digit);
                break;
            }
        }

        let mut number_parts = String::new();

        if let Some(first_digit_value) = first_digit {
            number_parts.push(first_digit_value)
        }

        if let Some(last_digit_value) = last_digit {
            number_parts.push(last_digit_value)
        }

        let parsed = number_parts.parse::<i64>();

        if parsed.is_ok() {
            result = result + parsed.unwrap();
        }
    }

    result
}

pub fn solve(use_test_input: bool, path_to_inputs: String) ->SolutionPair {
    let input = inputs::get_input(path_to_inputs, 2023, 1, use_test_input);

    (format!("{}", solve_part_one(&input)), format!("{}", solve_part_two(&input)))
}