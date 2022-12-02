use super::AdventOfCode2022;
use advent_of_code_traits::{days::Day2, ParseInput, Part1, Part2, Solution};

impl ParseInput<'_, Day2, Part1> for AdventOfCode2022<Day2> {
    type Parsed = Vec<(u32, u32)>;

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        input
            .lines()
            .map(|line| {
                let split: Vec<&str> = line.split(" ").collect();

                let first = match *split.first().unwrap() {
                    "A" => 1,
                    "B" => 2,
                    "C" => 3,
                    _ => panic!("Invalid input 1"),
                };

                let second = match *split.last().unwrap() {
                    "X" => 1,
                    "Y" => 2,
                    "Z" => 3,
                    _ => panic!("Invalid input 2"),
                };

                return (first, second);
            })
            .into_iter()
            .collect()
    }
}

impl Solution<'_, Day2, Part1> for AdventOfCode2022<Day2> {
    type Input = Vec<(u32, u32)>;

    type Output = u32;

    fn solve(&self, input: &Vec<(u32, u32)>) -> Self::Output {
        let mut score = 0;

        for game in input {
            score += game.1;
            if game.0 == game.1 {
                score += 3;
            } else if (game.1 + 1) % 3 == game.0 - 1 {
                score += 6;
            }
        }

        score
    }
}

impl Solution<'_, Day2, Part2> for AdventOfCode2022<Day2> {
    type Input = Vec<(u32, u32)>;

    type Output = u32;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        let mut score = 0;

        for game in input {
            let chosen_move = match game.1 {
                1 => (game.0 + 1) % 3,
                2 => game.0 - 1,
                3 => game.0 % 3,
                _ => panic!(),
            };

            score += chosen_move + 1 + (game.1 - 1) * 3;
        }

        score
    }
}
