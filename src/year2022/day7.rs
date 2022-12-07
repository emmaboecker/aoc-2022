use std::collections::HashMap;

use super::AdventOfCode2022;
use advent_of_code_traits::{days::Day7, ParseInput, Part1, Part2, Solution};

#[derive(Debug)]
pub enum DirContent {
    Directory(String),
    File(u64),
}

impl ParseInput<'_, Day7, Part1> for AdventOfCode2022<Day7> {
    type Parsed = HashMap<Vec<String>, Vec<DirContent>>;

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        let lines = input.lines();

        let mut map: Self::Parsed = HashMap::new();

        let mut current_dir = vec!["/".to_string()];

        for line in lines {
            if line.starts_with("$ cd") {
                let dir = line.split(" ").nth(2).unwrap();
                if dir == ".." {
                    current_dir.pop();
                } else if dir == "/" {
                    current_dir = vec!["/".to_string()];
                } else {
                    current_dir.push(dir.to_string());
                }
            }
            if !line.starts_with("$") {
                let mut split = line.split(" ");
                let type_or_size = split.nth(0).unwrap();

                if let Ok(size) = type_or_size.parse::<u64>() {
                    if map.contains_key(&current_dir) {
                        let vec = map.get_mut(&current_dir).unwrap();
                        vec.push(DirContent::File(size));
                    } else {
                        map.insert(current_dir.clone(), vec![DirContent::File(size)]);
                    }
                } else if type_or_size == "dir" {
                    let name = split.nth(0).unwrap();
                    if map.contains_key(&current_dir) {
                        let vec = map.get_mut(&current_dir).unwrap();
                        vec.push(DirContent::Directory(name.to_string()));
                    } else {
                        map.insert(
                            current_dir.clone(),
                            vec![DirContent::Directory(name.to_string())],
                        );
                    }
                }
            }
        }

        map
    }
}

impl Solution<'_, Day7, Part1> for AdventOfCode2022<Day7> {
    type Input = HashMap<Vec<String>, Vec<DirContent>>;

    type Output = u64;

    fn solve(&self, input: &HashMap<Vec<String>, Vec<DirContent>>) -> Self::Output {
        let mut sum = 0;

        for dir in input {
            let result = directory_size(input, dir);
            if result <= 100000 {
                sum += result;
            }
        }

        sum
    }
}

impl Solution<'_, Day7, Part2> for AdventOfCode2022<Day7> {
    type Input = HashMap<Vec<String>, Vec<DirContent>>;

    type Output = u64;

    fn solve(&self, input: &HashMap<Vec<String>, Vec<DirContent>>) -> Self::Output {
        let mut used_space = 0;
        let mut direcories = vec![];

        for dir in input {
            let result = directory_size(input, dir);
            if dir.0 == &vec!["/"] {
                used_space = result;
            }
            direcories.push((dir.0, result));
        }

        let unused_space = 70000000 - used_space;

        let mut direcories = direcories
            .into_iter()
            .filter(|a| unused_space + a.1 >= 30000000)
            .collect::<Vec<_>>();

        direcories.sort_by(|a, b| a.1.cmp(&b.1));

        direcories.first().unwrap().1.to_owned()
    }
}

fn directory_size(
    input: &HashMap<Vec<String>, Vec<DirContent>>,
    dir: (&Vec<String>, &Vec<DirContent>),
) -> u64 {
    let mut sum = 0;
    for content in dir.1 {
        let mut subdir_size = 0;
        if let DirContent::File(size) = content {
            sum += size;
        } else if let DirContent::Directory(name) = content {
            let mut new_dir = dir.0.clone();
            new_dir.push(name.to_owned());
            let subdir = input.get_key_value(&new_dir).unwrap();
            subdir_size += directory_size(input, subdir);
        }
        sum += subdir_size;
    }
    sum
}
