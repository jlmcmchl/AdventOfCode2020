use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|e| e.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_p1(input: &[u32]) -> u32 {
    let opts: Vec<_> = input.iter().combinations(2).filter(|v| v.iter().map(|a| *a).sum::<u32>() == 2020).map(|v| v.iter().map(|a| *a).product()).collect();
    opts[0]
}

#[aoc(day1, part2)]
pub fn solve_p2(input: &[u32]) -> u32 {
    let opts: Vec<_> = input.iter().combinations(3).filter(|v| v.iter().map(|a| *a).sum::<u32>() == 2020).map(|v| v.iter().map(|a| *a).product()).collect();
    opts[0]
}