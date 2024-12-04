#[macro_use]
extern crate enum_map;

mod solutions;
mod utils;

use crate::utils::inputs;
use clap::Parser;
use solutions::{y2023, y2024};
use std::time::Instant;

pub type SolutionPair = (String, String);

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    year: u16,

    #[arg(short, long)]
    day: u8,

    #[arg(short, long, default_value_t = false)]
    test_input: bool,

    #[arg(short, long, default_value = "./inputs")]
    path_to_inputs: String,
}

fn get_year(year: u16) -> fn(day: u8, input: String) -> SolutionPair {
    match year {
        2023 => y2023::solve,
        2024 => y2024::solve,
        _ => unimplemented!(),
    }
}

fn get_input(year: u16, day: u8, test_input: bool, path_to_inputs: String) -> String {
    inputs::get_input(path_to_inputs, year, day, test_input)
}

fn main() {
    let cli = Cli::parse();
    let solve = get_year(cli.year);
    let input = get_input(cli.year, cli.day, cli.test_input, cli.path_to_inputs);
    let time = Instant::now();
    let solution = solve(cli.day, input);
    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;

    println!("\n --- AoC day {}/{} ---", cli.day, cli.year);
    println!("  » part 1 = {}", solution.0);
    println!("  » Part 2 = {}", solution.1);
    println!("\n Executed in {:.4} ms", elapsed_ms)
}
