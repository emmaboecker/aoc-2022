use advent_of_code_traits::{days::Day1, run, SolutionRunner};

use crate::year2022::AdventOfCode2022;

mod year2022;

mod utils;

fn main() {
    let input = std::fs::read_to_string("./input/2022/day1.txt").expect("failed to read input");

    run!(AdventOfCode2022::<Day1>, &input);
}
