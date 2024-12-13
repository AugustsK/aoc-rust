use crate::{Coordinate, SolutionPair};
use std::collections::{HashMap, HashSet};
use std::ops::Index;

fn solve_part_one(input: &str) -> u64 {
    let mut result: u64 = 0;
    let mut order_map: HashMap<u16, HashSet<u16>> = HashMap::new();

    for line in input.lines() {
        if line.is_empty() {
            // skip the empty line
            continue;
        }

        if line.contains('|') {
            if let Some((a, b)) = line.split_once('|') {
                let a_num = a.parse::<u16>().unwrap();
                let b_num = b.parse::<u16>().unwrap();

                if let Some(maybeHashSet) = order_map.get_mut(&a_num) {
                    maybeHashSet.insert(b_num);
                } else {
                    let set: HashSet<u16> = HashSet::from([b_num]);
                    order_map.insert(a_num, set);
                }
            }
        }

        if line.contains(',') {
            let mut prev_nums: Vec<u16> = Vec::new();
            let mut invalid = false;

            'num_check: for num_str in line.split(',').collect::<Vec<&str>>() {
                let num_opt = num_str.parse::<u16>();

                if num_opt.is_ok() {
                    let num = num_opt.unwrap();

                    if let Some(maybeHashSet) = order_map.get(&num) {
                        for prev_num in &prev_nums {
                            if maybeHashSet.contains(prev_num) {
                                invalid = true;
                                break 'num_check;
                            }
                        }
                    }

                    prev_nums.push(num);
                }
            }

            if invalid {
                continue;
            }

            let middle_num = prev_nums.len() / 2;
            let elem = prev_nums[middle_num];
            result += elem as u64;
        }
    }

    result
}

fn solve_part_two(input: &str) -> u64 {
    let mut result: u64 = 0;
    let mut order_map: HashMap<u16, HashSet<u16>> = HashMap::new();

    for line in input.lines() {
        if line.is_empty() {
            // skip the empty line
            continue;
        }

        if line.contains('|') {
            if let Some((a, b)) = line.split_once('|') {
                let a_num = a.parse::<u16>().unwrap();
                let b_num = b.parse::<u16>().unwrap();

                if let Some(maybeHashSet) = order_map.get_mut(&a_num) {
                    maybeHashSet.insert(b_num);
                } else {
                    let set: HashSet<u16> = HashSet::from([b_num]);
                    order_map.insert(a_num, set);
                }
            }
        }

        if line.contains(',') {
            let mut prev_nums: Vec<u16> = Vec::new();
            let mut invalid = false;

            for num_str in line.split(',').collect::<Vec<&str>>() {
                let num_opt = num_str.parse::<u16>();

                if num_opt.is_ok() {
                    let num = num_opt.unwrap();

                    if let Some(maybeHashSet) = order_map.get(&num) {
                        for prev_num in &prev_nums {
                            if maybeHashSet.contains(prev_num) {
                                invalid = true;
                                break;
                            }
                        }
                    }

                    prev_nums.push(num);
                }
            }

            if !invalid {
                continue;
            }

            prev_nums.sort_by(|a, b| {
                if let Some(maybeHashSet) = order_map.get(a) {
                    let mut is_a_greater = false;

                    if maybeHashSet.contains(b) {
                        is_a_greater = true;
                    }

                    if is_a_greater {
                        return core::cmp::Ordering::Less;
                    }

                    return core::cmp::Ordering::Greater;
                }

                core::cmp::Ordering::Equal
            });

            let middle_num = prev_nums.len() / 2;
            let elem = prev_nums[middle_num];
            result += elem as u64;
        }
    }

    result
}

pub fn solve(input: String) -> SolutionPair {
    (
        format!("{}", solve_part_one(&input)),
        format!("{}", solve_part_two(&input)),
    )
}
