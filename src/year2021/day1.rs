use advent_of_code_traits::{days::Day1, ParseInput, Part1, Part2, Solution};

use super::AdventOfCode2021;

impl ParseInput<'_, Day1, Part1> for AdventOfCode2021<Day1> {
    type Parsed = Vec<u64>;

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        let input: Vec<String> = input.lines().map(|line| line.to_owned()).collect();

        input
            .into_iter()
            .map(|string| string.parse().unwrap())
            .collect()
    }
}

impl Solution<'_, Day1, Part1> for AdventOfCode2021<Day1> {
    type Input = Vec<u64>;

    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        let pairs = input.iter().zip(input.iter().skip(1));

        pairs.clone().filter(|(one, two)| one < two).count()
    }
}

impl Solution<'_, Day1, Part2> for AdventOfCode2021<Day1> {
    type Input = Vec<u64>;

    type Output = usize;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        let pairs = input.iter().zip(input.iter().skip(1));

        let triplets = pairs.zip(input.iter().skip(2));

        let windows: Vec<u64> = triplets.map(|((x, y), z)| x + y + z).collect();

        let window_pairs = windows.iter().zip(windows.iter().skip(1));

        window_pairs.filter(|(x, y)| x < y).count()
    }
}
