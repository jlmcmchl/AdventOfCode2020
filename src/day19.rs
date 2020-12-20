use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until},
    character::complete::digit1,
    IResult,
};

type Sequence = Vec<u8>;

#[derive(Debug, Clone)]
pub enum Rule {
    Value(String),
    Either(Box<Rule>, Box<Rule>),
    Sequence(Sequence),
}

fn parse_rule(input: &str) -> IResult<&str, (u8, Rule)> {
    let (input, id) = digit1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, rule) = alt((parse_value, parse_either, parse_sequence))(input)?;

    Ok((input, (id.parse().unwrap(), rule)))
}

fn parse_value(input: &str) -> IResult<&str, Rule> {
    let (input, _) = tag("\"")(input)?;
    let (input, val) = take(1usize)(input)?;
    let (input, _) = tag("\"")(input)?;

    Ok((input, Rule::Value(val.to_owned())))
}

fn parse_either(input: &str) -> IResult<&str, Rule> {
    let (input, first) = take_until(" | ")(input)?;
    let (input, _) = tag(" | ")(input)?;

    let (_, first) = parse_sequence(first)?;
    let (input, second) = parse_sequence(input)?;

    Ok((input, Rule::Either(Box::new(first), Box::new(second))))
}

fn parse_sequence(input: &str) -> IResult<&str, Rule> {
    let mut output = Vec::new();
    let (rest, num) = digit1(input)?;

    let mut input = rest;
    output.push(num.parse().unwrap());

    while !input.is_empty() && !input.starts_with(" | ") {
        let (rest, _) = tag(" ")(input)?;
        let (rest, digit) = digit1(rest)?;
        input = rest;
        output.push(digit.parse().unwrap());
    }

    Ok((input, Rule::Sequence(output)))
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> (HashMap<u8, Rule>, Vec<String>) {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    (
        parts[0]
            .lines()
            .map(|line| parse_rule(line).unwrap().1)
            .collect(),
        parts[1].lines().map(|line| line.to_owned()).collect(),
    )
}

fn apply_seq<'a>(rules: &HashMap<u8, Rule>, seq: Sequence, input: &'a str) -> IResult<&'a str, ()> {
    seq.iter()
        .map(|rule| move |input| apply(rules, *rule, input))
        .fold(Ok((input, ())), |last, func| {
            last.and_then(|(rest, _)| func(rest))
        })
}

fn apply<'a>(rules: &HashMap<u8, Rule>, root: u8, input: &'a str) -> IResult<&'a str, ()> {
    match rules[&root].clone() {
        Rule::Value(v) => tag(v.as_str())(input).map(|(i, _)| (i, ())),
        Rule::Either(left, right) => {
            if let Rule::Sequence(lseq) = *left {
                if let Rule::Sequence(rseq) = *right {
                    return apply_seq(rules, lseq, input)
                        .or_else(|_| apply_seq(rules, rseq, input))
                        .map(|(i, _)| (i, ()));
                }
            }
            unreachable!()
        }
        Rule::Sequence(seq) => apply_seq(rules, seq, input),
    }
}

fn apply_rule_42<'a>(
    rules: &HashMap<u8, Rule>,
    input: &'a str,
) -> IResult<&'a str, (usize, usize)> {
    let mut rest = input;
    let mut count = 0;

    loop {
        let (input, _) = apply(rules, 42, rest)?;
        count += 1;
        rest = input;

        let (input, other_count) = apply_rule_31(rules, rest)?;
        if other_count < count && input.is_empty() && other_count > 0 && count > 1 {
            return Ok((input, (count, other_count)));
        }
    }
}

fn apply_rule_31<'a>(rules: &HashMap<u8, Rule>, input: &'a str) -> IResult<&'a str, usize> {
    let mut rest = input;
    let mut count = 0;
    loop {
        let res = apply(rules, 31, rest);
        if res.is_err() {
            return Ok((rest, count));
        }
        let (input, _) = res?;
        count += 1;
        rest = input;
    }
}

#[aoc(day19, part1)]
pub fn solve_p1((rules, tests): &(HashMap<u8, Rule>, Vec<String>)) -> usize {
    tests
        .iter()
        .filter(|line| apply(rules, 0, line.as_str()).map_or(false, |(i, _)| i.is_empty()))
        .count()
}

#[aoc(day19, part2)]
pub fn solve_p2((rules, tests): &(HashMap<u8, Rule>, Vec<String>)) -> usize {
    let mut all_rules = "8: 42 | 42 8\n11: 42 31 | 42 11 31"
        .lines()
        .map(|line| parse_rule(line).unwrap().1)
        .collect::<HashMap<_, _>>();

    rules.iter().for_each(|(k, v)| {
        all_rules.entry(*k).or_insert_with(|| v.clone());
    });

    tests
        .iter()
        .filter(|line| {
            apply_rule_42(&all_rules, line.as_str())
                .map_or(false, |(i, _)| i.is_empty())
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    fn p1_wrapper(input: &str) -> usize {
        let parsed = input_generator(input);
        solve_p1(&parsed)
    }

    fn p2_wrapper(input: &str) -> usize {
        let parsed = input_generator(input);
        solve_p2(&parsed)
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1_wrapper("0: 4 1 5\n1: 2 3 | 3 2\n2: 4 4 | 5 5\n3: 4 5 | 5 4\n4: \"a\"\n5: \"b\"\n\nababbb\nbababa\nabbbab\naaabbb\naaaabbb"), 2);
        assert_eq!(p1_wrapper("42: 9 14 | 10 1\n9: 14 27 | 1 26\n10: 23 14 | 28 1\n1: \"a\"\n11: 42 31\n5: 1 14 | 15 1\n19: 14 1 | 14 14\n12: 24 14 | 19 1\n16: 15 1 | 14 14\n31: 14 17 | 1 13\n6: 14 14 | 1 14\n2: 1 24 | 14 4\n0: 8 11\n13: 14 3 | 1 12\n15: 1 | 14\n17: 14 2 | 1 7\n23: 25 1 | 22 14\n28: 16 1\n4: 1 1\n20: 14 14 | 1 15\n3: 5 14 | 16 1\n27: 1 6 | 14 18\n14: \"b\"\n21: 14 1 | 1 14\n25: 1 1 | 1 14\n22: 14 14\n8: 42\n26: 14 22 | 1 20\n18: 15 15\n7: 14 5 | 1 21\n24: 14 1\n\nabbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa\nbbabbbbaabaabba\nbabbbbaabbbbbabbbbbbaabaaabaaa\naaabbbbbbaaaabaababaabababbabaaabbababababaaa\nbbbbbbbaaaabbbbaaabbabaaa\nbbbababbbbaaaaaaaabbababaaababaabab\nababaaaaaabaaab\nababaaaaabbbaba\nbaabbaaaabbaaaababbaababb\nabbbbabbbbaaaababbbbbbaaaababb\naaaaabbaabaaaaababaa\naaaabbaaaabbaaa\naaaabbaabbaaaaaaabbbabbbaaabbaabaaa\nbabaaabbbaaabaababbaabababaaab\naabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"), 3);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2_wrapper("42: 9 14 | 10 1\n9: 14 27 | 1 26\n10: 23 14 | 28 1\n1: \"a\"\n11: 42 31\n5: 1 14 | 15 1\n19: 14 1 | 14 14\n12: 24 14 | 19 1\n16: 15 1 | 14 14\n31: 14 17 | 1 13\n6: 14 14 | 1 14\n2: 1 24 | 14 4\n0: 8 11\n13: 14 3 | 1 12\n15: 1 | 14\n17: 14 2 | 1 7\n23: 25 1 | 22 14\n28: 16 1\n4: 1 1\n20: 14 14 | 1 15\n3: 5 14 | 16 1\n27: 1 6 | 14 18\n14: \"b\"\n21: 14 1 | 1 14\n25: 1 1 | 1 14\n22: 14 14\n8: 42\n26: 14 22 | 1 20\n18: 15 15\n7: 14 5 | 1 21\n24: 14 1\n\nabbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa\nbbabbbbaabaabba\nbabbbbaabbbbbabbbbbbaabaaabaaa\naaabbbbbbaaaabaababaabababbabaaabbababababaaa\nbbbbbbbaaaabbbbaaabbabaaa\nbbbababbbbaaaaaaaabbababaaababaabab\nababaaaaaabaaab\nababaaaaabbbaba\nbaabbaaaabbaaaababbaababb\nabbbbabbbbaaaababbbbbbaaaababb\naaaaabbaabaaaaababaa\naaaabbaaaabbaaa\naaaabbaabbaaaaaaabbbabbbaaabbaabaaa\nbabaaabbbaaabaababbaabababaaab\naabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"), 12);
    }
}
