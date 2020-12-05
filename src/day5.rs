use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<u16> {
    input
        .replace("B", "1")
        .replace("F", "0")
        .replace("R", "1")
        .replace("L", "0")
        .lines()
        .map(|num| u16::from_str_radix(num, 2).unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_p1(input: &Vec<u16>) -> u16 {
    input.iter().fold(0, |acc, x| acc.max(*x))
}

#[aoc(day5, part2)]
pub fn solve_p2(input: &Vec<u16>) -> u16 {
    let max = input.iter().fold(0, |acc, x| acc.max(*x));
    let min = input.iter().fold(max, |acc, x| acc.min(*x));
    let seat = (min..=max)
        .filter(|seat| !input.contains(seat))
        .collect::<Vec<_>>();
    seat[0]
}
