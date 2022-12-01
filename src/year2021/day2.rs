use advent_of_code_traits::{days::Day2, ParseInput, Part1, Part2, Solution};

use super::AdventOfCode2021;

impl ParseInput<'_, Day2, Part1> for AdventOfCode2021<Day2> {
    type Parsed = Vec<Instruction>;

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        let input: Vec<String> = input.lines().map(|line| line.to_owned()).collect();

        let mut introductions = Vec::new();

        for instruction in input {
            let split: Vec<String> = instruction
                .split(" ")
                .into_iter()
                .map(|value| value.to_string())
                .collect();

            introductions.push(match split.get(0).unwrap().as_str() {
                "up" => Instruction::Up(split.get(1).unwrap().parse().unwrap()),
                "down" => Instruction::Down(split.get(1).unwrap().parse().unwrap()),
                "forward" => Instruction::Forward(split.get(1).unwrap().parse().unwrap()),
                _ => Instruction::None,
            });
        }

        introductions
    }
}

impl Solution<'_, Day2, Part1> for AdventOfCode2021<Day2> {
    type Input = Vec<Instruction>;

    type Output = i64;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        let mut horizontal: i64 = 0;
        let mut depth: i64 = 0;

        for instruction in input {
            match instruction {
                Instruction::Forward(x) => horizontal += x,
                Instruction::Up(x) => depth -= x,
                Instruction::Down(x) => depth += x,
                Instruction::None => {}
            }
        }

        return horizontal * depth;
    }
}

impl Solution<'_, Day2, Part2> for AdventOfCode2021<Day2> {
    type Input = Vec<Instruction>;

    type Output = i64;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        let mut horizontal: i64 = 0;
        let mut depth: i64 = 0;

        let mut aim: i64 = 0;

        for instruction in input {
            match instruction {
                Instruction::Forward(x) => {
                    horizontal += x;
                    depth += x * aim;
                }
                Instruction::Up(x) => aim -= x,
                Instruction::Down(x) => aim += x,
                Instruction::None => {}
            }
        }

        return horizontal * depth;
    }
}

#[derive(Debug)]
pub enum Instruction {
    Forward(i64),
    Up(i64),
    Down(i64),
    None,
}
