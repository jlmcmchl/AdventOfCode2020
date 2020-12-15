use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> HashMap<usize, usize> {
    input.split(",").enumerate().map(|(i,u)| (u.parse().unwrap(), i+1)).collect()
}

#[aoc(day15, part1)]
pub fn solve_p1(input: &HashMap<usize, usize>) -> usize {
    nth_spoken_number(input, 2020)
}

#[aoc(day15, part2)]
pub fn solve_p2(input: &HashMap<usize, usize>) -> usize {
    nth_spoken_number(input, 30000000)
}

fn nth_spoken_number(seed: &HashMap<usize, usize>, count: usize) -> usize {
    let mut seed_entries = seed.iter().map(|(k, v)| (*k ,*v)).collect::<Vec<_>>();
    seed_entries.sort_by_cached_key(|(_, v)| -1* (*v as isize));
    let mut latest_entry = seed_entries[0];
    let mut log: HashMap<_, _> = seed_entries.iter().skip(1).map(|i| *i).collect();

    let mut i = seed.iter().map(|(_, v)| *v).max().unwrap() + 1;

    while i <= count {
        let new_num = match log.get(&latest_entry.0) {
            Some(v) => i - v.clone() - 1,
            None => 0
        };

        log.insert(latest_entry.0, latest_entry.1);
        latest_entry = (new_num, i);        
        i+=1;
    }

    latest_entry.0
}

#[cfg(test)]
mod test {
    use super::nth_spoken_number;

    fn test_wrapper(input: &str, count: usize) -> usize {
        let seed = crate::day15::input_generator(input);
        nth_spoken_number(&seed, count)
    }
    #[test]
    fn test_p1() {
        assert_eq!(test_wrapper("0,3,6", 2020), 436);
        assert_eq!(test_wrapper("1,3,2", 2020), 1);
        assert_eq!(test_wrapper("2,1,3", 2020), 10);
        assert_eq!(test_wrapper("1,2,3", 2020), 27);
        assert_eq!(test_wrapper("2,3,1", 2020), 78);
        assert_eq!(test_wrapper("3,2,1", 2020), 438);
        assert_eq!(test_wrapper("3,1,2", 2020), 1836);
    }

    #[test]
    fn test_p2() {
        assert_eq!(test_wrapper("0,3,6", 30000000), 175594);
        assert_eq!(test_wrapper("1,3,2", 30000000), 2578);
        assert_eq!(test_wrapper("2,1,3", 30000000), 3544142);
        assert_eq!(test_wrapper("1,2,3", 30000000), 261214);
        assert_eq!(test_wrapper("2,3,1", 30000000), 6895259);
        assert_eq!(test_wrapper("3,2,1", 30000000), 18);
        assert_eq!(test_wrapper("3,1,2", 30000000), 362);
    }
}
