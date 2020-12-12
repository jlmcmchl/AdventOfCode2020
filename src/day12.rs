use std::{num::ParseIntError, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(u32),
    Right(u32),
    Forward(i32),
}

pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rotate(&self, angle: u32) -> Self {
        match self {
            Direction::North => match angle {
                90 => Direction::West,
                180 => Direction::South,
                270 => Direction::East,
                _ => unreachable!(),
            },
            Direction::South => match angle {
                90 => Direction::East,
                180 => Direction::North,
                270 => Direction::West,
                _ => unreachable!(),
            },
            Direction::East => match angle {
                90 => Direction::North,
                180 => Direction::West,
                270 => Direction::South,
                _ => unreachable!(),
            },
            Direction::West => match angle {
                90 => Direction::South,
                180 => Direction::East,
                270 => Direction::North,
                _ => unreachable!(),
            },
        }
    }

    fn to_instruction(&self, num: i32) -> Instruction {
        match self {
            Direction::North => Instruction::North(num),
            Direction::South => Instruction::South(num),
            Direction::East => Instruction::East(num),
            Direction::West => Instruction::West(num),
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::East
    }
}

#[derive(Default)]
pub struct State {
    x: i32,
    y: i32,
    waypoint: (i32, i32),
    facing: Direction,
}

impl State {
    fn apply(&mut self, instr: Instruction) {
        match instr {
            Instruction::North(num) => {
                self.y += num;
            }
            Instruction::South(num) => {
                self.y -= num;
            }
            Instruction::East(num) => self.x += num,
            Instruction::West(num) => {
                self.x -= num;
            }
            Instruction::Left(num) => {
                self.facing = self.facing.rotate(num);
            }
            Instruction::Right(num) => {
                self.facing = self.facing.rotate(360 - num);
            }
            Instruction::Forward(num) => {
                self.apply(self.facing.to_instruction(num));
            }
        }
    }

    fn apply2(&mut self, instr: Instruction) {
        match instr {
            Instruction::North(num) => self.waypoint.1 += num,
            Instruction::South(num) => self.waypoint.1 -= num,
            Instruction::East(num) => self.waypoint.0 += num,
            Instruction::West(num) => self.waypoint.0 -= num,

            Instruction::Left(num) => {
                self.rotate_waypoint(num);
            }
            Instruction::Right(num) => {
                self.rotate_waypoint(360 - num);
            }
            Instruction::Forward(num) => {
                self.x += self.waypoint.0 * num;
                self.y += self.waypoint.1 * num;
            }
        }
    }

    fn rotate_waypoint(&mut self, angle: u32) {
        match angle {
            90 => self.waypoint = (-self.waypoint.1, self.waypoint.0),
            180 => self.waypoint = (-self.waypoint.0, -self.waypoint.1),
            270 => self.waypoint = (self.waypoint.1, -self.waypoint.0),
            _ => unreachable!(),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, rest) = s.split_at(1);
        match first {
            "N" => rest.parse().map(|i| Instruction::North(i)),
            "S" => rest.parse().map(|i| Instruction::South(i)),
            "E" => rest.parse().map(|i| Instruction::East(i)),
            "W" => rest.parse().map(|i| Instruction::West(i)),
            "L" => rest.parse().map(|i| Instruction::Left(i)),
            "R" => rest.parse().map(|i| Instruction::Right(i)),
            "F" => rest.parse().map(|i| Instruction::Forward(i)),
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    // let input = "F10\nN3\nF7\nR90\nF11";
    input.lines().map(|instr| instr.parse().unwrap()).collect()
}

#[aoc(day12, part1)]
pub fn solve_p1(input: &Vec<Instruction>) -> i32 {
    // println!("{:?}", input);

    let mut state: State = Default::default();
    input.iter().for_each(|i| state.apply(*i));
    state.x.abs() + state.y.abs()
}

#[aoc(day12, part2)]
pub fn solve_p2(input: &Vec<Instruction>) -> i32 {
    let mut state = State {
        waypoint: (10, 1),
        ..Default::default()
    };
    input.iter().for_each(|i| state.apply2(*i));
    state.x.abs() + state.y.abs()
}
