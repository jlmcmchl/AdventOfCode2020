use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|u| u.parse::<u64>().unwrap()).collect()
}

fn is_predicated(e: u64, queue: &[u64]) -> bool {
    let set: HashSet<_> = queue.iter().collect();

    set.iter().tuple_combinations().any(|(a, b)| **a + **b == e)
}

#[aoc(day9, part1)]
pub fn solve_p1(input: &[u64]) -> u64 {
    let mut queue = Vec::new();

    input.iter().take(25).for_each(|e| queue.push(*e));

    input
        .iter()
        .skip(25)
        .filter(|e| {
            let res = is_predicated(**e, &queue);
            queue.drain(0..1);
            queue.push(**e);
            !res
        })
        .take(1)
        .sum()
}

#[aoc(day9, part2)]
pub fn solve_p2(input: &[u64]) -> u64 {
    let target = solve_p1(input);

    let mut i = 0;
    loop {
        let mut len = 1;
        while input.iter().skip(i).take(len).sum::<u64>() < target {
            len += 1;
        }

        if input.iter().skip(i).take(len).sum::<u64>() == target {
            // do the thing
            let min = input.iter().skip(i).take(len).min().unwrap();
            let max = input.iter().skip(i).take(len).max().unwrap();
            // println!("{} {}", min, max);
            return min + max;
        }

        i += 1;
    }
}
