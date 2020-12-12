use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Floor,
    EmptySeat,
    FilledSeat,
}

impl State {
    fn value(&self) -> u8 {
        match self {
            State::FilledSeat => 1,
            _ => 0,
        }
    }

    fn apply(&self, nearby: u8) -> Self {
        match self {
            State::Floor => State::Floor,
            State::EmptySeat => {
                if nearby == 0 {
                    State::FilledSeat
                } else {
                    State::EmptySeat
                }
            }
            State::FilledSeat => {
                if nearby >= 4 {
                    State::EmptySeat
                } else {
                    State::FilledSeat
                }
            }
        }
    }

    fn apply2(&self, nearby: u8) -> Self {
        match self {
            State::Floor => State::Floor,
            State::EmptySeat => {
                if nearby == 0 {
                    State::FilledSeat
                } else {
                    State::EmptySeat
                }
            }
            State::FilledSeat => {
                if nearby > 4 {
                    State::EmptySeat
                } else {
                    State::FilledSeat
                }
            }
        }
    }
}

impl Into<State> for u8 {
    fn into(self) -> State {
        match self {
            b'.' => State::Floor,
            b'L' => State::EmptySeat,
            b'#' => State::FilledSeat,
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> ((usize, usize), Vec<State>) {
    // let input = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";
    let height = input.as_bytes().iter().filter(|c| **c == b'\n').count() + 1;
    let width = input.as_bytes().iter().take_while(|c| **c != b'\n').count();
    let layout = input
        .as_bytes()
        .iter()
        .filter(|c| **c != b'\n')
        .map(|e| (*e).into())
        .collect();
    ((height, width), layout)
}

fn index_to_coord(ind: usize, (_, width): &(usize, usize)) -> (usize, usize) {
    let ht = ind / width;
    let wd = ind % width;
    (ht, wd)
}

fn coord_to_index(y: usize, x: usize, (_, width): &(usize, usize)) -> usize {
    y * width + x
}

fn nearby_filled_seats((y, x): (usize, usize), spec: &(usize, usize), layout: &Vec<State>) -> u8 {
    let mut cnt = 0;

    if x > 0 && y > 0 {
        cnt = cnt + layout[coord_to_index(y - 1, x - 1, spec)].value();
    }

    if y > 0 {
        cnt = cnt + layout[coord_to_index(y - 1, x, spec)].value();
    }

    if x < spec.1 - 1 && y > 0 {
        cnt = cnt + layout[coord_to_index(y - 1, x + 1, spec)].value();
    }

    if x > 0 {
        cnt = cnt + layout[coord_to_index(y, x - 1, spec)].value();
    }

    if x < spec.1 - 1 {
        cnt = cnt + layout[coord_to_index(y, x + 1, spec)].value();
    }

    if x > 0 && y < spec.0 - 1 {
        cnt = cnt + layout[coord_to_index(y + 1, x - 1, spec)].value();
    }

    if y < spec.0 - 1 {
        cnt = cnt + layout[coord_to_index(y + 1, x, spec)].value();
    }

    if x < spec.1 - 1 && y < spec.0 - 1 {
        cnt = cnt + layout[coord_to_index(y + 1, x + 1, spec)].value();
    }

    cnt
}

fn nearest_seat_in_direction(
    (y, x): (usize, usize),
    (dy, dx): (isize, isize),
    spec: &(usize, usize),
    layout: &Vec<State>,
) -> State {
    let mut curr_y = y as isize;
    let mut curr_x = x as isize;

    loop {
        curr_y += dy;
        curr_x += dx;

        if curr_x == spec.1 as isize || curr_y == spec.0 as isize || curr_x == -1 || curr_y == -1 {
            return State::Floor;
        }

        match layout[coord_to_index(curr_y as usize, curr_x as usize, spec)] {
            State::Floor => continue,
            a => return a,
        }
    }
}

fn nearby_filled_seats2((y, x): (usize, usize), spec: &(usize, usize), layout: &Vec<State>) -> u8 {
    let mut cnt = 0;

    if x > 0 && y > 0 {
        cnt = cnt + nearest_seat_in_direction((y, x), (-1, -1), spec, layout).value();
    }

    if y > 0 {
        cnt = cnt + nearest_seat_in_direction((y, x), (-1, 0), spec, layout).value();
    }

    if x < spec.1 - 1 && y > 0 {
        cnt = cnt + nearest_seat_in_direction((y, x), (-1, 1), spec, layout).value();
    }

    if x > 0 {
        cnt = cnt + nearest_seat_in_direction((y, x), (0, -1), spec, layout).value();
    }

    if x < spec.1 - 1 {
        cnt = cnt + nearest_seat_in_direction((y, x), (0, 1), spec, layout).value();
    }

    if x > 0 && y < spec.0 - 1 {
        cnt = cnt + nearest_seat_in_direction((y, x), (1, -1), spec, layout).value();
    }

    if y < spec.0 - 1 {
        cnt = cnt + nearest_seat_in_direction((y, x), (1, 0), spec, layout).value();
    }

    if x < spec.1 - 1 && y < spec.0 - 1 {
        cnt = cnt + nearest_seat_in_direction((y, x), (1, 1), spec, layout).value();
    }

    cnt
}

#[aoc(day11, part1)]
pub fn solve_p1((spec, layout): &((usize, usize), Vec<State>)) -> usize {
    let mut last = layout.clone();

    loop {
        let step = (0..layout.len())
            .map(|i| nearby_filled_seats(index_to_coord(i, spec), spec, &last))
            .zip(&last)
            .map(|(a, b)| b.apply(a))
            .collect::<Vec<_>>();

        if step.iter().zip(&last).all(|(a, b)| *a == *b) {
            break;
        }

        last = step;
    }

    last.iter().filter(|a| **a == State::FilledSeat).count()
}

#[aoc(day11, part2)]
pub fn solve_p2((spec, layout): &((usize, usize), Vec<State>)) -> usize {
    let mut last = layout.clone();

    loop {
        let step = (0..layout.len())
            .map(|i| nearby_filled_seats2(index_to_coord(i, spec), spec, &last))
            .zip(&last)
            .map(|(a, b)| b.apply2(a))
            .collect::<Vec<_>>();

        if step.iter().zip(&last).all(|(a, b)| *a == *b) {
            break;
        }

        last = step;
    }

    last.iter().filter(|a| **a == State::FilledSeat).count()
}
