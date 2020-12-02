use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    bytes::complete::take_until,
    character::complete::not_line_ending,
    combinator::map,
    multi::separated_list0,
    sequence::{terminated, tuple},
};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(usize, usize, char, String)> {
    let res: nom::IResult<_, _> = separated_list0(
        tag("\n"),
        tuple((
            map(terminated(take_until("-"), tag("-")), |s: &str| {
                s.parse::<usize>().unwrap()
            }),
            map(terminated(take_until(" "), tag(" ")), |s: &str| {
                s.parse::<usize>().unwrap()
            }),
            map(terminated(take_until(": "), tag(": ")), |s: &str| {
                FromStr::from_str(s).unwrap()
            }),
            map(not_line_ending, |s: &str| s.into()),
        )),
    )(input);

    res.unwrap().1
}

#[aoc(day2, part1)]
pub fn solve_p1(input: &[(usize, usize, char, String)]) -> usize {
    // let input: Vec<(usize, usize, char, String)> = vec![
    //     (1, 3, 'a', "abcde".into()),
    //     (1, 3, 'b', "cdefg".into()),
    //     (2, 9, 'c', "ccccccccc".into()),
    // ];

    input
        .iter()
        .filter(|(l, h, c, s)| {
            let count = s.chars().filter(|chr| chr == c).count();
            count >= *l && count <= *h
        })
        .count()
}

#[aoc(day2, part2)]
pub fn solve_p2(input: &[(usize, usize, char, String)]) -> usize {
    // let input: Vec<(usize, usize, char, String)> = vec![
    //     (1, 3, 'a', "abcde".into()),
    //     (1, 3, 'b', "cdefg".into()),
    //     (2, 9, 'c', "ccccccccc".into()),
    // ];

    input
        .iter()
        .filter(|(l, h, c, s)| {
            let first = s.chars().collect::<Vec<_>>()[l - 1] == *c;
            let second = s.chars().collect::<Vec<_>>()[h - 1] == *c;
            first ^ second
        })
        .count()
}
