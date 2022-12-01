use advent_of_code_traits::{days::Day3, ParseInput, Part1, Part2, Solution};

use super::AdventOfCode2021;

impl ParseInput<'_, Day3, Part1> for AdventOfCode2021<Day3> {
    type Parsed = Vec<String>;

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        input.lines().map(|line| line.to_owned()).collect()
    }
}

impl Solution<'_, Day3, Part1> for AdventOfCode2021<Day3> {
    type Input = Vec<String>;

    type Output = u64;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        let length = input.iter().min_by_key(|line| line.len()).unwrap().len();

        let nums: Vec<u64> = input
            .iter()
            .map(|x| u64::from_str_radix(x, 2).expect("invalid input"))
            .collect();

        let mut gamma: u64 = 0;

        for index in 0..length {
            // gamma[rev_index] = most_common_vertical(nums, index)
            gamma |= (most_common_vertical(&nums, index) as u64) << (length - index - 1);
        }

        let epsilon: u64 = !gamma & ((1 << length) - 1);

        gamma * epsilon
    }
}

impl Solution<'_, Day3, Part2> for AdventOfCode2021<Day3> {
    type Input = Vec<String>;

    type Output = u64;

    fn solve(&self, input: &Self::Input) -> Self::Output {
        let length = input.iter().min_by_key(|line| line.len()).unwrap().len();

        let nums: Vec<u64> = input
            .iter()
            .map(|x| u64::from_str_radix(x, 2).expect("invalid input"))
            .collect();

        let oxy: u64 = oxy_co2(nums.clone(), length, false);

        let co2: u64 = oxy_co2(nums.clone(), length, true);

        oxy * co2
    }
}

fn oxy_co2(nums: Vec<u64>, length: usize, co2: bool) -> u64 {
    let mut curr: Vec<u64> = nums;
    for index in (0..length).rev() {
        let most_common = most_common_vertical(&curr, index);
        let most_common = if co2 { !most_common } else { most_common };

        curr = curr
            .iter()
            // x[index] == most_common
            .filter(|x| (*x >> index & 1) == most_common as u64)
            .cloned()
            .collect();
        if curr.len() == 1 {
            break;
        }
    }
    curr[0]
}

fn most_common_vertical(nums: &Vec<u64>, index: usize) -> bool {
    let mut ones = 0;

    for num in nums {
        let bit = (num >> index) & 1; // num[index]

        match bit {
            0 => ones -= 1,
            1 => ones += 1,
            _ => unreachable!(), // bit is either 0 or 1
        }
    }

    if ones >= 0 {
        true
    } else {
        false
    }
}
