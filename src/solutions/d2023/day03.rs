use crate::{SolutionPair};
use crate::utils::inputs;


fn solve_part_one(input: &str) -> u64 {
    let mut result: u64 = 0;

    for line in input.lines() {

    }

    result
}

fn solve_part_two(input: &str) -> u64 {
    let mut result: u64 = 0;

    for line in input.lines() {

    }

    result
}

pub fn solve(use_test_input: bool, path_to_inputs: String) -> SolutionPair {
    let input = inputs::get_input(path_to_inputs, 2023, 2, use_test_input);

    (format!("{}", solve_part_one(&input)), format!("{}", solve_part_two(&input)))
}