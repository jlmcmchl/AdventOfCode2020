use std::{collections::HashSet, fmt::Debug, unimplemented};

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::{is_a, take},
    multi::separated_list0,
    sequence::{terminated, tuple},
    IResult,
};

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<(String, i16)> {
    //let input = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";
    let res: IResult<_, _> = separated_list0(
        tag("\n"),
        tuple((
            terminated(take(3usize), tag(" ")),
            alt((tag("-"), tag("+"))),
            is_a("1234567890"),
        )),
    )(input);

    let (_, txt) = res.unwrap();

    txt.iter()
        .map(|(a, b, c)| {
            (
                (*a).into(),
                c.parse::<i16>().unwrap() * if *b == "-" { -1 } else { 1 },
            )
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_p1(input: &Vec<(String, i16)>) -> State {
    let mut seen_lines = HashSet::new();
    let mut curr = 0;
    let mut acc = 0;

    loop {
        if !seen_lines.insert(curr) || curr == input.len() {
            break;
        }

        let (op, num) = &input[curr];
        match op.as_ref() {
            "acc" => {
                acc = acc + num;
                curr = curr + 1;
            }
            "jmp" => {
                curr = curr + *num as usize;
            }
            "nop" => {
                curr = curr + 1;
            }
            _ => unimplemented!(),
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
pub fn solve_p2(input: &Vec<(String, i16)>) -> State {
    let mut asm = input.clone();
    let mut curr = 0;
    let mut state;
    loop {
        let (op, num) = &input[curr];
        let new_op = match op.as_ref() {
            "jmp" => "nop",
            "nop" => "jmp",
            a => a,
        };

        asm[curr] = (new_op.into(), *num);

        state = solve_p1(&asm);
        if state.index == input.len() {
            break;
        }

        asm[curr] = (op.clone(), *num);
        curr += 1;
    }

    state
}
