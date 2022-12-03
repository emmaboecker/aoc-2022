use advent_of_code_traits::days::{Day1, Day2, Day3};
use advent_of_code_traits::{run, SolutionRunner};

use crate::year2021::AdventOfCode2021;
use crate::year2022::AdventOfCode2022;

mod year2021;
mod year2022;

mod utils;

fn main() {
    println!("2021:");
    run!(AdventOfCode2021::<Day1>, &get_input(2021, 1));
    run!(AdventOfCode2021::<Day2>, &get_input(2021, 2));
    run!(AdventOfCode2021::<Day3>, &get_input(2021, 3));

    println!("2022:");
    run!(AdventOfCode2022::<Day1>, &get_input(2022, 1));
    run!(AdventOfCode2022::<Day2>, &get_input(2022, 2));
    run!(AdventOfCode2022::<Day3>, &get_input(2022, 3));
}

fn get_input(year: u32, day: u32) -> String {
    std::fs::read_to_string(format!("./input/{year}/day{day}.txt"))
        .expect(format!("failed to read input for day {}", day).as_str())
}
