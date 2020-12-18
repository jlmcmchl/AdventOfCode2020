use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> HashSet<(i8, i8, i8)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, chr)| {
                if chr == '.' {
                    None
                } else {
                    Some((x as i8, y as i8, 0))
                }
            })
        })
        .collect()
}

fn step(state: &HashSet<(i8, i8, i8)>) -> HashSet<(i8, i8, i8)> {
    let mut nearby: HashMap<(i8, i8, i8), u8> = HashMap::new();
    state.iter().for_each(|point| {
        (0..27)
            .filter(|i| *i != 13)
            .map(move |e| {
                (
                    point.0 - 1 + (e % 3),
                    point.1 - 1 + (e / 3 % 3),
                    point.2 - 1 + (e / 9),
                )
            })
            .for_each(|point| {
                let counter = nearby.entry(point).or_insert(0);
                *counter += 1;
            });
    });

    nearby
        .iter()
        .filter_map(|(point, count)| {
            if state.contains(point) {
                if (2..=3).contains(count) {
                    Some(point)
                } else {
                    None
                }
            } else if *count == 3 {
                Some(point)
            } else {
                None
            }
        })
        .copied()
        .collect()
}

fn render(state: &HashSet<(i8, i8, i8)>) {
    let min_x = state.iter().min_by_key(|(x, _, _)| x).unwrap();
    let max_x = state.iter().max_by_key(|(x, _, _)| x).unwrap();
    let min_y = state.iter().min_by_key(|(_, y, _)| y).unwrap();
    let max_y = state.iter().max_by_key(|(_, y, _)| y).unwrap();
    let min_z = state.iter().min_by_key(|(_, _, z)| z).unwrap();
    let max_z = state.iter().max_by_key(|(_, _, z)| z).unwrap();

    let mut out: String = format!(
        "x: {} -> {}\ny: {} -> {}\nz: {} -> {}\n",
        min_x.0, max_x.0, min_y.1, max_y.1, min_z.2, max_z.2
    );

    for z in min_z.2..=max_z.2 {
        out += &format!("z={}\n", z);
        for y in min_y.1..=max_y.1 {
            for x in min_x.0..=max_x.0 {
                out += if state.contains(&(x, y, z)) { "#" } else { "." };
            }
            out += "\n";
        }
        out += "\n";
    }

    println!("{}", out);
}

#[aoc(day17, part1)]
pub fn solve_p1(input: &HashSet<(i8, i8, i8)>) -> usize {
    // println!("{:?}", input);
    let mut state = input.clone();

    // println!("0: {:?}", state.len());
    // render(&state);
    for _i in 0..6 {
        state = step(&state);

        // println!("{}: {}", _i+1, state.len());
        // render(&state);
    }

    state.len()
}

fn step2(state: &HashSet<(i8, i8, i8, i8)>) -> HashSet<(i8, i8, i8, i8)> {
    let mut nearby: HashMap<(i8, i8, i8, i8), u8> = HashMap::new();
    state.iter().for_each(|point| {
        (0..81)
            .filter(|i| *i != 40)
            .map(move |e| {
                (
                    point.0 - 1 + (e % 3),
                    point.1 - 1 + (e / 3 % 3),
                    point.2 - 1 + (e / 9 % 3),
                    point.3 - 1 + (e / 27),
                )
            })
            .for_each(|point| {
                let counter = nearby.entry(point).or_insert(0);
                *counter += 1;
            });
    });

    nearby
        .iter()
        .filter_map(|(point, count)| {
            if state.contains(point) {
                if (2..=3).contains(count) {
                    Some(point)
                } else {
                    None
                }
            } else if *count == 3 {
                Some(point)
            } else {
                None
            }
        })
        .copied()
        .collect()
}

#[aoc(day17, part2)]
pub fn solve_p2(input: &HashSet<(i8, i8, i8)>) -> usize {
    let mut state = input.iter().map(|(x, y, z)| (*x, *y, *z, 0)).collect();
    assert_eq!(
        (0..81)
            .map(|i| (
                -1 + (i % 3),
                -1 + (i / 3 % 3),
                -1 + (i / 9 % 3),
                -1 + (i / 27)
            ))
            .collect::<HashSet<_>>()
            .len(),
        81
    );

    for _i in 0..6 {
        state = step2(&state);
    }

    state.len()
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
        assert_eq!(p1_wrapper(".#.\n..#\n###"), 112);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2_wrapper(".#.\n..#\n###"), 848);
    }
}
