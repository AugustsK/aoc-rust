use crate::solutions::d2023::{day01, day02, day03};
use crate::SolutionPair;

pub fn solve(day: u8, input: String) -> SolutionPair {
    let func = get_solver(day);

    func(input)
}

fn get_solver(day: u8) -> fn(input: String) -> SolutionPair {
    match day {
        1 => day01::solve,
        2 => day02::solve,
        3 => day03::solve,
        _ => unimplemented!(),
    }
}
