use std::ops::RangeInclusive;

use super::AdventOfCode2022;
use advent_of_code_traits::{days::Day4, ParseInput, Part1, Part2, Solution};

impl ParseInput<'_, Day4, Part1> for AdventOfCode2022<Day4> {
    type Parsed = Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>;

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        let ranges = input
            .lines()
            .map(|line| {
                line.to_string()
                    .split(",")
                    .map(|x| {
                        x.split("-")
                            .map(|x| x.parse::<u32>().unwrap())
                            .collect::<Vec<u32>>()
                    })
                    .map(|x| {
                        RangeInclusive::new(
                            x.first().unwrap().to_owned(),
                            x.last().unwrap().to_owned(),
                        )
                    })
                    .collect::<Vec<RangeInclusive<u32>>>()
            })
            .collect::<Vec<Vec<RangeInclusive<u32>>>>();

        ranges
            .into_iter()
            .map(|ranges| {
                (
                    ranges.first().unwrap().to_owned(),
                    ranges.last().unwrap().to_owned(),
                )
            })
            .collect()
    }
}

impl Solution<'_, Day4, Part1> for AdventOfCode2022<Day4> {
    type Input = Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>;

    type Output = usize;

    fn solve(&self, input: &Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>) -> Self::Output {
        // Check how many ranges in input contain each other
        input
            .into_iter()
            .filter(|x| {
                (x.0.contains(x.1.start()) && x.0.contains(x.1.end()))
                    || (x.1.contains(x.0.start()) && x.1.contains(x.0.end()))
            })
            .count()
    }
}

impl Solution<'_, Day4, Part2> for AdventOfCode2022<Day4> {
    type Input = Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>;

    type Output = usize;

    fn solve(&self, input: &Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>) -> Self::Output {
        input
            .into_iter()
            .filter(|x| {
                (x.0.contains(x.1.start()) || x.0.contains(x.1.end()))
                    || (x.1.contains(x.0.start()) || x.1.contains(x.0.end()))
            })
            .count()
    }
}
