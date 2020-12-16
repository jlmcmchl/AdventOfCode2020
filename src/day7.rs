use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use nom::{regex::Regex, regexp::str::re_captures, IResult};

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<String, Vec<(u8, String)>> {
    // let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.";
    // let input = "shiny gold bags contain 2 dark red bags.\ndark red bags contain 2 dark orange bags.\ndark orange bags contain 2 dark yellow bags.\ndark yellow bags contain 2 dark green bags.\ndark green bags contain 2 dark blue bags.\ndark blue bags contain 2 dark violet bags.\ndark violet bags contain no other bags.";
    let parser = Regex::new("(\\w+ \\w+) bags contain (?:(?:(\\d) (\\w+ \\w+) bags?(?:, (\\d) (\\w+ \\w+) bags?(?:, (\\d) (\\w+ \\w+) bags?(?:, (\\d) (\\w+ \\w+) bags?)?)?)?)|(?:no other bags))\\.").unwrap();
    let res: IResult<_, _> = re_captures(parser)(input);
    let (_, lines) = res.unwrap();

    lines
        .iter()
        .map(|line| {
            let stuff = line
                .chunks_exact(2)
                .skip(1)
                .map(|chunk| (chunk[0].parse::<u8>().unwrap(), chunk[1].to_owned()))
                .collect::<Vec<_>>();
            (line[1].to_owned(), stuff)
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_p1(input: &HashMap<String, Vec<(u8, String)>>) -> usize {
    let mut candidate_bags = HashSet::new();
    let inverted_map = input.iter().fold(HashMap::new(), |mut acc, (k, v)| {
        v.iter().for_each(|(_, id)| {
            if !acc.contains_key(id) {
                acc.insert(id, Vec::new());
            }
            if let Some(i) = acc.get_mut(id) {
                i.push(k.clone())
            }
        });
        acc
    });

    candidate_bags.insert("shiny gold".into());

    loop {
        let count = candidate_bags.len();

        candidate_bags = candidate_bags
            .iter()
            .fold(HashSet::new(), |mut acc, bag: &String| {
                acc.insert(bag.clone());

                if let Some(v) = inverted_map.get(bag) {
                    v.iter().for_each(|bag| {
                        acc.insert(bag.clone());
                    })
                }

                acc
            });

        if candidate_bags.len() == count {
            break;
        }
    }

    candidate_bags.len() - 1
}

fn contained_bag_count(bags: &HashMap<String, Vec<(u8, String)>>, bag: String) -> u64 {
    bags.get(&bag)
        .unwrap()
        .iter()
        .map(|(cnt, inner_bag)| *cnt as u64 * (1 + contained_bag_count(bags, inner_bag.clone())))
        .sum()
}

#[aoc(day7, part2)]
pub fn solve_p2(input: &HashMap<String, Vec<(u8, String)>>) -> u64 {
    contained_bag_count(input, "shiny gold".into())
}
