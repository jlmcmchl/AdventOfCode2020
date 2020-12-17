use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::{is_a, tag},
    IResult,
};

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u16>> {
    let (input, start) = is_a("0123456789")(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, end) = is_a("0123456789")(input)?;

    let start = start.parse().unwrap();
    let end = end.parse().unwrap();

    Ok((input, start..=end))
}

type Field = (String, (RangeInclusive<u16>, RangeInclusive<u16>));

type ParsedInput = (
    HashMap<String, (RangeInclusive<u16>, RangeInclusive<u16>)>,
    Vec<u16>,
    Vec<Vec<u16>>,
);

fn parse_field(input: &str) -> IResult<&str, Field> {
    let (input, name) = is_a("abcdefghijklmnopqrstuvwxyz ")(input)?;
    let (input, _) = tag(": ")(input)?;

    let (input, first) = parse_range(input)?;
    let (input, _) = tag(" or ")(input)?;
    let (input, second) = parse_range(input)?;

    Ok((input, (name.to_owned(), (first, second))))
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> ParsedInput {
    let pieces: Vec<_> = input.split("\n\n").collect();
    let fields = pieces[0]
        .lines()
        .map(|line| parse_field(line).map(|(_, stuff)| stuff))
        .collect::<Vec<_>>()
        .into_iter()
        .collect::<Result<HashMap<_, _>, _>>()
        .unwrap();
    let your_ticket = pieces[1]
        .split_once('\n')
        .and_then(|(_, b)| {
            b.split(',')
                .map(|i| i.parse())
                .collect::<Vec<_>>()
                .into_iter()
                .collect::<Result<Vec<_>, _>>()
                .ok()
        })
        .unwrap();

    let other_tickets = pieces[2]
        .lines()
        .skip(1)
        .map(|e| {
            e.split(',')
                .map(|i| i.parse())
                .collect::<Vec<_>>()
                .into_iter()
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Vec<_>>()
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    (fields, your_ticket, other_tickets)
}

fn validate(
    ticket: &[u16],
    ranges: &HashMap<String, (RangeInclusive<u16>, RangeInclusive<u16>)>,
) -> u16 {
    ticket.iter().fold(0, |acc, e| {
        acc + if !ranges
            .iter()
            .any(|(_, (low, high))| low.contains(e) || high.contains(e))
        {
            *e
        } else {
            0
        }
    })
}

#[aoc(day16, part1)]
pub fn solve_p1((ranges, _, other_tickets): &ParsedInput) -> u16 {
    // println!("{:?}", ranges);
    // println!("{:?}", other_tickets);

    other_tickets
        .iter()
        .fold(0, |acc, ticket| acc + validate(ticket, ranges))
}

#[aoc(day16, part2)]
pub fn solve_p2((ranges, your_ticket, other_tickets): &ParsedInput) -> u64 {
    let other_tickets = other_tickets
        .iter()
        .filter(|ticket| validate(*ticket, ranges) == 0)
        .collect::<Vec<_>>();

    let mut map = ranges
        .iter()
        .map(|(name, (first, second))| {
            (
                name,
                (0..your_ticket.len())
                    .filter(|i| {
                        other_tickets.iter().all(|ticket| {
                            first.contains(&ticket[*i]) || second.contains(&ticket[*i])
                        })
                    })
                    .collect::<HashSet<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();

    let reference = map.clone();

    map.iter_mut()
        .filter(|(_, candidates)| candidates.len() > 1)
        .for_each(|(_, candidates)| {
            let len = candidates.len();
            reference
                .iter()
                .filter(|(_, stuffs)| stuffs.len() + 1 == len)
                .for_each(|(_, smaller_set)| {
                    smaller_set.iter().for_each(|col| {
                        candidates.remove(col);
                    })
                })
        });

    map.iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .flat_map(|(_, set)| set.iter())
        .copied()
        .map(|ind| your_ticket[ind] as u64)
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    fn p1_wrapper(input: &str) -> u16 {
        let parsed = input_generator(input);
        solve_p1(&parsed)
    }

    fn p2_wrapper(input: &str) -> u64 {
        let parsed = input_generator(input);
        solve_p2(&parsed)
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1_wrapper("class: 1-3 or 5-7\nrow: 6-11 or 33-44\nseat: 13-40 or 45-50\n\nyour ticket:\n7,1,14\n\nnearby tickets:\n7,3,47\n40,4,50\n55,2,20\n38,6,12"), 71);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2_wrapper("class: 1-3 or 5-7\nrow: 6-11 or 33-44\nseat: 13-40 or 45-50\n\nyour ticket:\n7,1,14\n\nnearby tickets:\n7,3,47\n40,4,50\n55,2,20\n38,6,12"), 1);
    }
}
