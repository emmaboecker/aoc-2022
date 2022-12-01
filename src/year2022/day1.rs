use advent_of_code_traits::{days::Day1, ParseInput, Part1, Part2, Solution};

use crate::utils::split_delimited;

use super::AdventOfCode2022;

impl ParseInput<'_, Day1, Part1> for AdventOfCode2022<Day1> {
    type Parsed = Vec<Vec<u64>>;

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        let input = &input.lines().collect::<Vec<&str>>();
        let input = split_delimited(input, &"");
        input
            .into_iter()
            .map(|str| {
                str.into_iter()
                    .map(|str| str.parse().expect("Not A Number"))
                    .collect()
            })
            .collect()
    }
}

impl Solution<'_, Day1, Part1> for AdventOfCode2022<Day1> {
    type Input = Vec<Vec<u64>>;

    type Output = u64;

    fn solve(&self, input: &Vec<Vec<u64>>) -> Self::Output {
        let mut sums: Vec<u64> = input
            .into_iter()
            .map(|numbers| numbers.into_iter().sum())
            .collect();

        sums.sort();

        return sums.last().expect("No number").to_owned();
    }
}

impl Solution<'_, Day1, Part2> for AdventOfCode2022<Day1> {
    type Input = Vec<Vec<u64>>;

    type Output = u64;

    fn solve(&self, input: &Vec<Vec<u64>>) -> Self::Output {
        let mut sums: Vec<u64> = input
            .into_iter()
            .map(|numbers| numbers.into_iter().sum())
            .collect();

        sums.sort();
        sums.reverse();

        let mut sum = 0;

        for number in &sums[0..3] {
            sum = sum + number;
        }

        sum
    }
}
