use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> (usize, usize) {
    let input = input
        .lines()
        .map(|i| i.parse().unwrap())
        .collect::<Vec<_>>();
    (input[0], input[1])
}

#[aoc(day25, part1)]
pub fn solve_p1((card_key, door_key): &(usize, usize)) -> usize {
    // let card_loops = solve_for_loops(7, *card_key);
    let door_loops = solve_for_loops(7, *door_key);
    // println!("{} {}", card_loops, door_loops);
    // let enc_key = transform(*card_key, door_loops);
    // assert_eq!(enc_key, transform(*card_key, door_loops));
    // enc_key
    transform(*card_key, door_loops)
}

fn transform(subject: usize, loops: usize) -> usize {
    (0..loops).fold(1, |val, _| val * subject % 20201227)
}

fn solve_for_loops(subject: usize, public: usize) -> usize {
    let mut current = 1;
    for i in 0.. {
        current *= subject;
        current %= 20201227;
        if current == public {
            return i + 1;
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(solve_for_loops(7, 5764801), 8);
        assert_eq!(solve_for_loops(7, 17807724), 11);
        assert_eq!(transform(5764801, 11), 14897079);
        assert_eq!(transform(17807724, 8), 14897079);
        assert_eq!(solve_p1(&(5764801, 17807724)), 14897079);
    }
}
