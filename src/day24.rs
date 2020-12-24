use std::{collections::HashSet, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub enum Direction {
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
    Northeast,
}

impl Direction {
    fn apply(&self, (x, y): (i8, i8)) -> (i8, i8) {
        match self {
            Direction::East => (x + 1, y),
            Direction::Southeast => (x, y + 1),
            Direction::Southwest => (x - 1, y + 1),
            Direction::West => (x - 1, y),
            Direction::Northwest => (x, y - 1),
            Direction::Northeast => (x + 1, y - 1),
        }
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "e" => Ok(Direction::East),
            "se" => Ok(Direction::Southeast),
            "sw" => Ok(Direction::Southwest),
            "w" => Ok(Direction::West),
            "nw" => Ok(Direction::Northwest),
            "ne" => Ok(Direction::Northeast),
            _ => Err(s.into()),
        }
    }
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<Vec<Direction>> {
    input
        .lines()
        .map(|line| {
            line.split_inclusive(|e| e == 'e' || e == 'w')
                .map(|i| i.parse().unwrap())
                .collect()
        })
        .collect()
}

fn get_initial_state(input: &[Vec<Direction>]) -> HashSet<(i8, i8)> {
    let tiles: Vec<(i8, i8)> = input
        .iter()
        .map(|directions| {
            directions
                .iter()
                .fold((0, 0), |coord, direction| direction.apply(coord))
        })
        .collect();
    let mut black_tiles = HashSet::new();
    tiles.iter().for_each(|tile| {
        if black_tiles.contains(tile) {
            black_tiles.remove(tile);
        } else {
            black_tiles.insert(*tile);
        }
    });

    black_tiles
}

#[aoc(day24, part1)]
pub fn solve_p1(input: &[Vec<Direction>]) -> usize {
    get_initial_state(input).len()
}

fn iter(tiles: HashSet<(i8, i8)>) -> HashSet<(i8, i8)> {
    let neighbors = vec![
        Direction::East,
        Direction::Southeast,
        Direction::Southwest,
        Direction::West,
        Direction::Northwest,
        Direction::Northeast,
    ];
    let candidates = tiles
        .iter()
        .flat_map(|tile| {
            neighbors
                .iter()
                .map(move |direction| direction.apply(*tile))
        })
        .collect::<HashSet<_>>();

    candidates
        .iter()
        .filter(|tile| {
            let nearby = neighbors
                .iter()
                .map(|direction| direction.apply(**tile))
                .filter(|tile| tiles.contains(tile))
                .count();

            if tiles.contains(tile) {
                // black tile
                !(nearby == 0 || nearby > 2)
            } else {
                // white tile
                nearby == 2
            }
        })
        .copied()
        .collect()
}

fn run(input: &HashSet<(i8, i8)>, times: u16) -> HashSet<(i8, i8)> {
    let tiles = input.clone();
    (0..times).fold(tiles, |tiles, _| iter(tiles))
}

#[aoc(day24, part2)]
pub fn solve_p2(input: &[Vec<Direction>]) -> usize {
    let tiles = get_initial_state(input);

    run(&tiles, 100).len()
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
        let tiles = get_initial_state(&parsed);

        assert_eq!(run(&tiles, 1).len(), 15);
        assert_eq!(run(&tiles, 2).len(), 12);
        assert_eq!(run(&tiles, 3).len(), 25);
        assert_eq!(run(&tiles, 4).len(), 14);
        assert_eq!(run(&tiles, 5).len(), 23);
        assert_eq!(run(&tiles, 6).len(), 28);
        assert_eq!(run(&tiles, 7).len(), 41);
        assert_eq!(run(&tiles, 8).len(), 37);
        assert_eq!(run(&tiles, 9).len(), 49);
        assert_eq!(run(&tiles, 10).len(), 37);
        assert_eq!(run(&tiles, 20).len(), 132);
        assert_eq!(run(&tiles, 30).len(), 259);
        assert_eq!(run(&tiles, 40).len(), 406);
        assert_eq!(run(&tiles, 50).len(), 566);
        assert_eq!(run(&tiles, 60).len(), 788);
        assert_eq!(run(&tiles, 70).len(), 1106);
        assert_eq!(run(&tiles, 80).len(), 1373);
        assert_eq!(run(&tiles, 90).len(), 1844);

        solve_p2(&parsed)
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1_wrapper("sesenwnenenewseeswwswswwnenewsewsw\nneeenesenwnwwswnenewnwwsewnenwseswesw\nseswneswswsenwwnwse\nnwnwneseeswswnenewneswwnewseswneseene\nswweswneswnenwsewnwneneseenw\neesenwseswswnenwswnwnwsewwnwsene\nsewnenenenesenwsewnenwwwse\nwenwwweseeeweswwwnwwe\nwsweesenenewnwwnwsenewsenwwsesesenwne\nneeswseenwwswnwswswnw\nnenwswwsewswnenenewsenwsenwnesesenew\nenewnwewneswsewnwswenweswnenwsenwsw\nsweneswneswneneenwnewenewwneswswnese\nswwesenesewenwneswnwwneseswwne\nenesenwswwswneneswsenwnewswseenwsese\nwnwnesenesenenwwnenwsewesewsesesew\nnenewswnwewswnenesenwnesewesw\neneswnwswnwsenenwnwnwwseeswneewsenese\nneswnwewnwnwseenwseesewsenwsweewe\nwseweeenwnesenwwwswnew"), 10);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2_wrapper("sesenwnenenewseeswwswswwnenewsewsw\nneeenesenwnwwswnenewnwwsewnenwseswesw\nseswneswswsenwwnwse\nnwnwneseeswswnenewneswwnewseswneseene\nswweswneswnenwsewnwneneseenw\neesenwseswswnenwswnwnwsewwnwsene\nsewnenenenesenwsewnenwwwse\nwenwwweseeeweswwwnwwe\nwsweesenenewnwwnwsenewsenwwsesesenwne\nneeswseenwwswnwswswnw\nnenwswwsewswnenenewsenwsenwnesesenew\nenewnwewneswsewnwswenweswnenwsenwsw\nsweneswneswneneenwnewenewwneswswnese\nswwesenesewenwneswnwwneseswwne\nenesenwswwswneneswsenwnewswseenwsese\nwnwnesenesenenwwnenwsewesewsesesew\nnenewswnwewswnenesenwnesewesw\neneswnwswnwsenenwnwnwwseeswneewsenese\nneswnwewnwnwseenwseesewsenwsweewe\nwseweeenwnesenwwwswnew"), 2208);
    }
}
