use std::slice::Iter;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> (u64, Vec<Option<u64>>) {
    let lines = input.lines().collect::<Vec<_>>();

    (
        lines[0].parse().unwrap(),
        lines[1].split(",").map(|x| x.parse().ok()).collect(),
    )
}

#[aoc(day13, part1)]
pub fn solve_p1((target, buses): &(u64, Vec<Option<u64>>)) -> u64 {
    let (time, bus) = buses
        .iter()
        .filter_map(|i| *i)
        .map(|i| (i - (target % i), i))
        .min_by_key(|(extra, _)| *extra)
        .unwrap();

    time * bus
}

fn chinese_remainder_theorem(eqns: Vec<(u64, u64)>) -> u64 {
    let mut it = eqns.iter();
    it.next();
    crt_internal(eqns[0].0, eqns[0].1, it)
}

fn crt_internal(start: u64, iter: u64, mut remaining: Iter<'_, (u64, u64)>) -> u64 {
    match remaining.next() {
        Some((a, n)) => {
            let mut check = start;

            while check % n != *a {
                check += iter;
            }
            crt_internal(check, iter * n, remaining)
        }
        None => start,
    }
}

#[aoc(day13, part2)]
pub fn solve_p2((_, buses): &(u64, Vec<Option<u64>>)) -> u64 {
    let mut eqns = buses
        .iter()
        .enumerate()
        .filter_map(|(ind, v)| v.map(|bus| ((bus - (ind as u64 % bus)) % bus, bus)))
        .collect::<Vec<_>>();
    // eqns.sort_by_cached_key(|(_, x)| -1 * *x as i64);
    println!("{:?}", eqns);

    assert!(eqns.iter().all(|(a, n)| 1058443396696792 % n == *a));

    chinese_remainder_theorem(eqns)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = "939\n7,13,x,x,59,x,31,19";
        let parsed = crate::day13::input_generator(input);
        assert_eq!(crate::day13::solve_p2(&parsed), 1068781);

        let input = "939\n17,x,13,19";
        let parsed = crate::day13::input_generator(input);
        assert_eq!(crate::day13::solve_p2(&parsed), 3417);

        let input = "939\n67,7,59,61";
        let parsed = crate::day13::input_generator(input);
        assert_eq!(crate::day13::solve_p2(&parsed), 754018);

        let input = "939\n67,x,7,59,61";
        let parsed = crate::day13::input_generator(input);
        assert_eq!(crate::day13::solve_p2(&parsed), 779210);

        let input = "939\n67,7,x,59,61";
        let parsed = crate::day13::input_generator(input);
        assert_eq!(crate::day13::solve_p2(&parsed), 1261476);

        let input = "939\n1789,37,47,1889";
        let parsed = crate::day13::input_generator(input);
        assert_eq!(crate::day13::solve_p2(&parsed), 1202161486);
    }
}
