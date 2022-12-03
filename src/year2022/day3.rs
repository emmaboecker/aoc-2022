use super::AdventOfCode2022;
use advent_of_code_traits::{days::Day3, ParseInput, Part1, Part2, Solution};

impl ParseInput<'_, Day3, Part1> for AdventOfCode2022<Day3> {
    type Parsed = Vec<String>;

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        input.lines().map(|line| line.to_string()).collect()
    }
}

impl Solution<'_, Day3, Part1> for AdventOfCode2022<Day3> {
    type Input = Vec<String>;

    type Output = u64;

    fn solve(&self, input: &Vec<String>) -> Self::Output {
        input
            .into_iter()
            .map(|line| {
                let (first, second) = line.split_at(line.len() / 2);
                common_char_priority(&[first, second]).unwrap()
            })
            .sum()
    }
}

impl Solution<'_, Day3, Part2> for AdventOfCode2022<Day3> {
    type Input = Vec<String>;

    type Output = u64;

    fn solve(&self, input: &Vec<String>) -> Self::Output {
        input
            .into_iter()
            .map(|string| string.as_str())
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|chunk| common_char_priority(chunk).unwrap())
            .sum()
    }
}

fn common_char_priority(strings: &[&str]) -> Option<u64> {
    let first = strings.first()?;
    let char = first.chars().nth(
        first
            .find(|char| strings.iter().all(|line| line.contains(char)))
            .unwrap(),
    )?;
    if char.is_uppercase() {
        Some(char as u64 - 38)
    } else {
        Some(char as u64 - 96)
    }
}
