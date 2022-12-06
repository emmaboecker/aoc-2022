use super::AdventOfCode2022;
use advent_of_code_traits::{days::Day6, ParseInput, Part1, Part2, Solution};

impl ParseInput<'_, Day6, Part1> for AdventOfCode2022<Day6> {
    type Parsed = String;

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        return input.to_string();
    }
}

impl Solution<'_, Day6, Part1> for AdventOfCode2022<Day6> {
    type Input = String;

    type Output = i32;

    fn solve(&self, input: &String) -> Self::Output {
        let mut count = 4;

        for chars in input.chars().collect::<Vec<_>>().windows(4) {
            let abc = chars.into_iter().enumerate().any(|(i, x)| {
                let mut chars = chars.to_vec();
                chars.remove(i);
                chars.into_iter().any(|y| x == &y)
            });

            if abc {
                count += 1;
            } else {
                break;
            }
        }

        count
    }
}

impl Solution<'_, Day6, Part2> for AdventOfCode2022<Day6> {
    type Input = String;

    type Output = i32;

    fn solve(&self, input: &String) -> Self::Output {
        let mut count = 14;

        for chars in input.chars().collect::<Vec<_>>().windows(14) {
            let abc = chars.into_iter().enumerate().any(|(i, x)| {
                let mut chars = chars.to_vec();
                chars.remove(i);
                chars.into_iter().any(|y| x == &y)
            });

            if abc {
                count += 1;
            } else {
                break;
            }
        }

        count
    }
}
