#![allow(dead_code, unused_variables)]

// use rayon::prelude::*;

use derive_more::Deref;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(PartialEq, Eq, EnumIter, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn reverse(&self) -> Self {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Right => Left,
            Left => Right,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn advance(&self, direction: &Direction, height: &usize, width: &usize) -> Option<Self> {
        let pos = match direction {
            Direction::Up if self.y > 0 => Pos {
                y: self.y - 1,
                x: self.x,
            },
            Direction::Down if self.y < (height - 1) => Pos {
                y: self.y + 1,
                x: self.x,
            },
            Direction::Left if self.x > 0 => Pos {
                y: self.y,
                x: self.x - 1,
            },
            Direction::Right if self.x < (width - 1) => Pos {
                y: self.y,
                x: self.x + 1,
            },
            _ => return None,
        };
        Some(pos)
    }
}

#[derive(PartialEq, Eq)]
struct Cart {
    pos: Pos,
    direction: Direction,
    steps_direction: u8,
    cost: usize,
}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Cart {
    fn successors(&self, grid: &Grid) -> Vec<Self> {
        let height = grid.len();
        let width = grid[0].len();

        let mut successors = vec![];

        for direction in Direction::iter() {
            // Minimum of 4 steps to the same direction
            if self.direction != direction && self.steps_direction < 4 {
                continue;
            }

            if self.direction == direction && self.steps_direction == 10 {
                continue;
            }

            // Cannot go back
            if self.direction.reverse() == direction {
                continue;
            }

            if let Some(pos) = self.pos.advance(&direction, &height, &width) {
                let cost = self.cost + grid[pos.y][pos.x] as usize;
                let steps_direction = if self.direction == direction {
                    self.steps_direction + 1
                } else {
                    1
                };

                successors.push(Self {
                    pos,
                    cost,
                    direction,
                    steps_direction,
                })
            }
        }

        successors
    }
}

#[derive(Deref)]
struct Grid(Vec<Vec<u8>>);

pub fn run(input: &str) -> Result<usize, String> {
    let grid = Grid(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|b| b.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect(),
    );

    let height = grid.len();
    let width = grid[0].len();

    let mut priority_queue = BinaryHeap::new();
    let mut seen = HashSet::new();

    let goal = Pos {
        x: width - 1,
        y: height - 1,
    };

    let right = Cart {
        cost: grid[0][1] as usize,
        direction: Direction::Right,
        steps_direction: 1,
        pos: Pos { x: 1, y: 0 },
    };
    let down = Cart {
        cost: grid[1][0] as usize,
        direction: Direction::Down,
        steps_direction: 1,
        pos: Pos { x: 0, y: 1 },
    };
    priority_queue.push(right);
    priority_queue.push(down);

    while let Some(cart) = priority_queue.pop() {
        if cart.pos == goal && cart.steps_direction >= 4 {
            return Ok(cart.cost);
        }
        for cart in cart.successors(&grid) {
            if seen.insert((cart.pos, cart.direction, cart.steps_direction)) {
                priority_queue.push(cart);
            }
        }
    }

    // let height = grid.0.len();
    // let width = grid.0[0].len();
    // for y in 0..height {
    //     let mut line = "".to_string();
    //     for x in 0..width {
    //         line += &grid.0[y][x].to_string();
    //     }
    //     println!("{}", line);
    // }
    Err("Failed to find a route".to_string())
}
