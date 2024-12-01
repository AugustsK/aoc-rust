use crate::{SolutionPair};
use crate::utils::inputs;

fn solve_part_one(input: &str) -> i64 {
    let mut result: i64 = 0;

    result
}

fn solve_part_two(input: &str) -> i64 {
    let mut result: i64 = 0;

    result
}

pub fn solve(use_test_input: bool, path_to_inputs: String) ->SolutionPair {
    let input = inputs::get_input(path_to_inputs, 2024, 1, use_test_input);

    (format!("{}", solve_part_one(&input)), format!("{}", solve_part_two(&input)))
}