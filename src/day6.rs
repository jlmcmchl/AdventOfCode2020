use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vec<Vec<u8>>> {
    // let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
    input
        .split("\n\n")
        .map(|votes| {
            votes
                .lines()
                .map(|line| line.as_bytes().iter().cloned().collect())
                .collect()
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_p1(input: &Vec<Vec<Vec<u8>>>) -> usize {
    input
        .iter()
        .map(|set| {
            set.iter()
                .fold(HashSet::new(), |mut acc, voter| {
                    voter.iter().for_each(|vote| {
                        acc.insert(*vote);
                    });
                    acc
                })
                .len()
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn solve_p2(input: &Vec<Vec<Vec<u8>>>) -> usize {
    input
        .iter()
        .map(|set| {
            let votes = set.iter().fold(HashSet::new(), |mut acc, voter| {
                voter.iter().for_each(|vote| {
                    acc.insert(*vote);
                });
                acc
            });
            votes
                .iter()
                .filter(|vote| set.iter().all(|voter| voter.contains(*vote)))
                .count()
        })
        .sum()
}
