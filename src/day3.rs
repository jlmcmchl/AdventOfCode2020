use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    // let input = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";
    input
        .lines()
        .map(|e| {
            e.chars()
                .map(|c| if c == '#' { 1 } else { 0 })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_p1(input: &[Vec<u8>]) -> u64 {
    ski(input, 3, 1)
}

fn ski(mountain: &[Vec<u8>], x_slope: usize, y_slope: usize) -> u64 {
    let mut x_index = 0;
    let mut y_index = 0;
    let mut acc = 0;
    while y_index < mountain.len() {
        let row = &mountain[y_index];
        acc += row[x_index] as u64;
        x_index += x_slope;
        x_index %= row.len();
        y_index += y_slope;
    }

    acc
}

#[aoc(day3, part2)]
pub fn solve_p2(input: &[Vec<u8>]) -> u64 {
    ski(input, 1, 1) * ski(input, 3, 1) * ski(input, 5, 1) * ski(input, 7, 1) * ski(input, 1, 2)
}
