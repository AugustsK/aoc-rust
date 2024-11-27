use crate::SolutionPair;
use crate::solutions::d2023::{day01};

pub fn solve(day: u8, use_test_input: bool, path_to_inputs: String) -> SolutionPair {
    let func = get_solver(day);

    func(use_test_input, path_to_inputs)
}

fn get_solver(day: u8) -> fn(test_input: bool, path_to_inputs: String) -> SolutionPair {
    match day {
        1 => day01::solve,
        _ => unimplemented!(),
    }
}