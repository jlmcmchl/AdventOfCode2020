use std::{cmp::Ordering, collections::{HashSet, VecDeque}};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> (VecDeque<u8>, VecDeque<u8>) {
    let mut input = input.split("\n\n");

    let first = input
        .next()
        .map(|player| {
            player
                .lines()
                .skip(1)
                .map(|e| e.parse::<u8>().unwrap())
                .collect::<VecDeque<_>>()
        })
        .unwrap();
    let second = input
        .next()
        .map(|player| {
            player
                .lines()
                .skip(1)
                .map(|e| e.parse::<u8>().unwrap())
                .collect::<VecDeque<_>>()
        })
        .unwrap();

    (first, second)
}

enum RoundWinner {
    P1(u8, u8),
    P2(u8, u8),
}

enum GameWinner {
    None,
    P1,
    P2,
}

fn check_winner(p1: &VecDeque<u8>, p2: &VecDeque<u8>) -> GameWinner {
    if p1.is_empty() {
        GameWinner::P2
    } else if p2.is_empty(){
        GameWinner::P1
    } else {
        GameWinner::None
    }
}

fn iter(p1: &mut VecDeque<u8>, p2: &mut VecDeque<u8>) -> RoundWinner {
    let first = p1.pop_front();
    let second = p2.pop_front();

    match first {
        Some(first) => match second {
            Some(second) => {
                match first.cmp(&second) {
                    Ordering::Greater => RoundWinner::P1(first, second),
                    Ordering::Less => RoundWinner::P2(first, second),
                    _ => unreachable!()
                }
            }
            None => unreachable!(),
        },
        None => unreachable!(),
    }
}

fn play(p1: &mut VecDeque<u8>, p2: &mut VecDeque<u8>) -> usize {
    loop {
        match check_winner(p1, p2) {
            GameWinner::P1 => {
                return p1
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(i, j)| (i + 1) * *j as usize)
                    .sum()
            }
            GameWinner::P2 => {
                return p2
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(i, j)| (i + 1) * *j as usize)
                    .sum()
            }
            _ => {}
        }

        match iter(p1, p2) {
            RoundWinner::P1(first, second) => {
                p1.push_back(first);
                p1.push_back(second);
            }
            RoundWinner::P2(first, second) => {
                p2.push_back(second);
                p2.push_back(first);
            }
        }
    }
}

#[aoc(day22, part1)]
pub fn solve_p1((p1, p2): &(VecDeque<u8>, VecDeque<u8>)) -> usize {
    let mut p1 = p1.clone();
    let mut p2 = p2.clone();

    play(&mut p1, &mut p2)
}

fn iter_recursive(p1: &mut VecDeque<u8>, p2: &mut VecDeque<u8>) -> RoundWinner {
    let first = p1.pop_front();
    let second = p2.pop_front();

    // println!("Player 1 plays: {:?}", first);
    // println!("Player 2 plays: {:?}", second);

    match first {
        Some(first) => match second {
            Some(second) => {
                if first <= p1.len() as u8 && second <= p2.len() as u8 {
                    let mut subgame_p1 = p1.iter().copied().take(first as usize).collect();
                    let mut subgame_p2 = p2.iter().copied().take(second as usize).collect();

                    // println!("Playing a subgame to determine the winner");
                    match play_recursive(&mut subgame_p1, &mut subgame_p2) {
                        GameWinner::P1 => {
                            // println!("Player 1 won the subgame");
                            RoundWinner::P1(first, second)
                        }
                        GameWinner::P2 => {
                            // println!("Player 2 won the subgame");
                            RoundWinner::P2(first, second)
                        }
                        _ => unreachable!(),
                    }
                } else if first > second {
                    RoundWinner::P1(first, second)
                } else if first < second {
                    RoundWinner::P2(first, second)
                } else {
                    unreachable!()
                }
            }
            None => unreachable!(),
        },
        None => unreachable!(),
    }
}

fn check_winner2(
    p1: &VecDeque<u8>,
    p2: &VecDeque<u8>,
    states: &mut HashSet<(Vec<u8>, Vec<u8>)>,
) -> GameWinner {
    if !states.insert((p1.iter().copied().collect(), p2.iter().copied().collect())) || p2.is_empty()
    {
        GameWinner::P1
    } else if p1.is_empty() {
        GameWinner::P2
    } else {
        GameWinner::None
    }
}

fn play_recursive(p1: &mut VecDeque<u8>, p2: &mut VecDeque<u8>) -> GameWinner {
    let mut states = HashSet::new();
    loop {
        match check_winner2(p1, p2, &mut states) {
            GameWinner::None => {}
            a => return a,
        }

        // println!("Next Round:");
        // println!("Player 1: {:?}", p1);
        // println!("Player 2: {:?}", p2);

        match iter_recursive(p1, p2) {
            RoundWinner::P1(first, second) => {
                // println!("Player 1 wins this round");
                p1.push_back(first);
                p1.push_back(second);
            }
            RoundWinner::P2(first, second) => {
                // println!("Player 2 wins this round");
                p2.push_back(second);
                p2.push_back(first);
            }
        }
    }
}

#[aoc(day22, part2)]
pub fn solve_p2((p1, p2): &(VecDeque<u8>, VecDeque<u8>)) -> usize {
    let mut p1 = p1.clone();
    let mut p2 = p2.clone();

    match play_recursive(&mut p1, &mut p2) {
        GameWinner::P1 => {
            return p1
                .iter()
                .rev()
                .enumerate()
                .map(|(i, j)| (i + 1) * *j as usize)
                .sum()
        }
        GameWinner::P2 => {
            return p2
                .iter()
                .rev()
                .enumerate()
                .map(|(i, j)| (i + 1) * *j as usize)
                .sum()
        }
        _ => unreachable!(),
    }
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
        assert_eq!(
            p1_wrapper("Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10"),
            306
        )
    }

    #[test]
    fn test_p2() {
        assert_eq!(
            p2_wrapper("Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10"),
            291
        )
    }
}
