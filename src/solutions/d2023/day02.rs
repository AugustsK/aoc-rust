use std::cmp::PartialEq;
use crate::{SolutionPair};
use crate::utils::inputs;

#[derive(Debug, Enum, PartialEq)]
enum GameColors {
    Red,
    Green,
    Blue,
}

fn match_game_part_to_color(part: &str) -> Option<(GameColors, u16)> {
    let game_color_map = enum_map! {
        GameColors::Red => " red",
        GameColors::Green => " green",
        GameColors::Blue => " blue",
    };

    for (key, &value) in &game_color_map {
        if part.ends_with(value) {
            let color_value = part.trim_end_matches(value).parse::<u16>();

            if color_value.is_ok() {
                return Some((key, color_value.unwrap()));
            }

        }
    }

    None
}

fn solve_part_one(input: &str) -> u64 {
    let mut result: u64 = 0;
    let mut game_id: u64 = 1;

    for line in input.lines() {
        let parsed_line = line.split(": ").collect::<Vec<&str>>();
        let mut valid = true;

        let game = parsed_line[1];

        for game_subset in game.split("; ") {
            for dices in game_subset.split(", ") {
                if let Some(count) = match_game_part_to_color(dices) {
                    if count.0 == GameColors::Red && count.1 > 12
                        || count.0 == GameColors::Green && count.1 > 13
                        || count.0 == GameColors::Blue && count.1 > 14
                    {
                        valid = false;
                        break;
                    }
                }
            }
        }

        if valid {
            result += game_id;
        }

        game_id += 1;
    }

    result
}

fn solve_part_two(input: &str) -> u64 {
    let mut result: u64 = 0;

    for line in input.lines() {
        let parsed_line = line.split(": ").collect::<Vec<&str>>();
        let mut red: u16 = 0;
        let mut green: u16 = 0;
        let mut blue: u16 = 0;
        let game = parsed_line[1];

        for game_subset in game.split("; ") {
            for dices in game_subset.split(", ") {
                if let Some(count) = match_game_part_to_color(dices) {
                    if count.0 == GameColors::Red && count.1 > red {
                        red = count.1;
                    }

                    if count.0 == GameColors::Green && count.1 > green {
                        green = count.1;
                    }

                    if count.0 == GameColors::Blue && count.1 > blue {
                        blue = count.1;
                    }
                }
            }
        }

        result += (red * green * blue) as u64;
    }

    result
}

pub fn solve(use_test_input: bool, path_to_inputs: String) ->SolutionPair {
    let input = inputs::get_input(path_to_inputs, 2023, 2, use_test_input);

    (format!("{}", solve_part_one(&input)), format!("{}", solve_part_two(&input)))
}