use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day16)]
pub fn input_generator(_: &str) -> () {
}

#[aoc(day16, part1)]
pub fn solve_p1(_: &()) -> usize {
    0
}

#[aoc(day16, part2)]
pub fn solve_p2(_: &()) -> usize {
    0
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
        assert!(true);
    }

    #[test]
    fn test_p2() {
        assert!(true);
    }
}