use std::collections::HashMap;
use crate::{SolutionPair};
use crate::utils::inputs;

fn solve_part_one(input: &str) -> u64 {
    let mut result: u64 = 0;
    let mut a: Vec<u64> = vec![];
    let mut b: Vec<u64> = vec![];

    for line in input.lines() {
        let parsed_line = line.split("   ").collect::<Vec<&str>>();
        let left = parsed_line[0].parse::<u64>();
        let right = parsed_line[1].parse::<u64>();

        if left.is_ok() && right.is_ok() {
            a.push(left.unwrap());
            b.push(right.unwrap());
        }
    }

    a.sort();
    b.sort();

    for (pos, left_value) in a.iter().enumerate() {
        let right_value = b[pos];

        result += left_value.abs_diff(right_value);
    }

    result
}

fn solve_part_two(input: &str) -> u64 {
    let mut result: u64 = 0;
    let mut left_list: Vec<u64> = vec![];
    let mut right_map: HashMap<u64, u64> = HashMap::new();

    for line in input.lines() {
        let parsed_line = line.split("   ").collect::<Vec<&str>>();
        let left = parsed_line[0].parse::<u64>();
        let right = parsed_line[1].parse::<u64>();

        if left.is_ok() && right.is_ok() {
            left_list.push(left.unwrap());
            let right_value = right.unwrap();

            if right_map.contains_key(&right_value) {
                right_map.insert(right_value, right_map.get(&right_value).unwrap() + 1);
            } else {
                right_map.insert(right_value, 1);
            }
        }
    }

    for left_value in left_list {
        if right_map.contains_key(&left_value) {
            result += left_value * right_map.get(&left_value).unwrap();
        }
    }

    result
}

pub fn solve(use_test_input: bool, path_to_inputs: String) ->SolutionPair {
    let input = inputs::get_input(path_to_inputs, 2024, 1, use_test_input);

    (format!("{}", solve_part_one(&input)), format!("{}", solve_part_two(&input)))
}