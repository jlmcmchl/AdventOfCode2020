use std::{collections::HashSet, fmt::Debug};

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    bytes::complete::{is_a, take},
    multi::separated_list0,
    sequence::{terminated, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Nop(i16),
    Jmp(i16),
    Acc(i16),
}

impl From<(&str, i16)> for Op {
    fn from((op, val): (&str, i16)) -> Self {
        match op {
            "acc" => Op::Acc(val),
            "jmp" => Op::Jmp(val),
            "nop" => Op::Nop(val),
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Op> {
    //let input = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";
    let res: IResult<_, _> = separated_list0(
        tag("\n"),
        tuple((terminated(take(3usize), tag(" ")), is_a("+-1234567890"))),
    )(input);

    let (_, txt) = res.unwrap();

    txt.iter()
        .map(|(a, b)| Op::from((*a, b.parse::<i16>().unwrap())))
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_p1(input: &Vec<Op>) -> State {
    let mut seen_lines = HashSet::new();
    let mut curr = 0;
    let mut acc = 0;

    loop {
        if !seen_lines.insert(curr) || curr == input.len() {
            break;
        }

        let op = input[curr];
        match op {
            Op::Acc(val) => {
                acc += val;
                curr += 1;
            }
            Op::Jmp(val) => {
                curr += val as usize;
            }
            Op::Nop(_) => {
                curr += 1;
            }
        }
    }

    State { index: curr, acc }
}

#[derive(Debug, Default)]
pub struct State {
    index: usize,
    acc: i16,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[aoc(day8, part2)]
pub fn solve_p2(input: &Vec<Op>) -> State {
    let mut asm = input.clone();
    let mut curr = 0;
    let mut state;
    loop {
        let op = input[curr];

        match op {
            Op::Acc(_) => {
                curr += 1;
                continue;
            }
            Op::Jmp(val) => {
                asm[curr] = Op::Nop(val);
            }
            Op::Nop(val) => {
                asm[curr] = Op::Jmp(val);
            }
        }

        state = solve_p1(&asm);
        if state.index == input.len() {
            break;
        }

        asm[curr] = op;
        curr += 1;
    }

    state
}
