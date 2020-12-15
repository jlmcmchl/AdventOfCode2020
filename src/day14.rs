use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{branch::alt, bytes::complete::is_a};
use nom::{bytes::complete::tag, IResult};

#[derive(Debug, Clone)]
pub enum Op {
    Mask(HashMap<u8, u8>),
    Set(usize, u64),
}

#[aoc_generator(day14)]
fn input_generator(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|e| alt((as_set, as_mask))(e))
        .collect::<Vec<_>>()
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .iter()
        .map(|(_, op)| op.clone())
        .collect()
}

fn as_mask(input: &str) -> IResult<&str, Op> {
    let (input, _) = tag("mask = ")(input)?;
    let (input, mask) = is_a("01X")(input)?;
    let mask = mask
        .bytes()
        .rev()
        .enumerate()
        .filter_map(|(i, e)| match e {
            b'0' => Some((i as u8, 0)),
            b'1' => Some((i as u8, 1)),
            _ => None,
        })
        .collect();

    Ok((input, Op::Mask(mask)))
}

fn as_set(input: &str) -> IResult<&str, Op> {
    let (input, _) = tag("mem[")(input)?;
    let (input, loc) = is_a("0123456789")(input)?;
    let (input, _) = tag("] = ")(input)?;
    let (input, num) = is_a("0123456789")(input)?;

    Ok((input, Op::Set(loc.parse().unwrap(), num.parse().unwrap())))
}

fn apply_mask(val: u64, mask: &HashMap<u8, u8>) -> u64 {
    mask.iter().fold(val, |val, (i, v)| {
        let masked_bit = val & 1 << i;
        val ^ masked_bit | (*v as u64) << i
    })
}

fn mask_base(val: usize, mask: &HashMap<u8, u8>) -> usize {
    mask.iter()
        .fold(0, |acc, (k, v)| acc | (*v as usize) << k | (val & (1 << k)))
}

fn apply_mask2(val: usize, mask: &HashMap<u8, u8>) -> Vec<usize> {
    let count = 35 - mask.len();
    let base = mask_base(val, mask);
    let res = (0..2 << count)
        .map(|filler| {
            (0..36)
                .filter(|v| !mask.contains_key(v))
                .enumerate()
                .fold(base, |acc, (filler_ind, key_ind)| {
                    acc | ((filler >> filler_ind) & 1) << key_ind
                })
        })
        .collect();

    res
}

#[aoc(day14, part1)]
pub fn solve_p1(input: &Vec<Op>) -> u64 {
    let mut mask = &Default::default();
    let mut mem: HashMap<usize, u64> = Default::default();

    input.iter().for_each(|op| match op {
        Op::Mask(inner) => mask = inner,
        Op::Set(ind, val) => {
            mem.insert(*ind, apply_mask(*val, &mask));
        }
    });

    mem.iter().map(|(_, b)| b).sum()
}

#[aoc(day14, part2)]
pub fn solve_p2(input: &Vec<Op>) -> u64 {
    let mut mask = &Default::default();
    let mut mem: HashMap<usize, u64> = Default::default();

    input.iter().for_each(|op| match op {
        Op::Mask(inner) => mask = inner,
        Op::Set(ind, val) => apply_mask2(*ind, mask).iter().for_each(|k| {
            mem.insert(*k, *val);
        }),
    });

    mem.iter().map(|(_, b)| b).sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_p1() {
        let input =
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";
        let parsed = crate::day14::input_generator(input);
        assert_eq!(crate::day14::solve_p1(&parsed), 165);
    }

    #[test]
    fn test_p2() {
        let input =
            "mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1";
        let parsed = crate::day14::input_generator(input);
        assert_eq!(crate::day14::solve_p2(&parsed), 208);
    }
}
