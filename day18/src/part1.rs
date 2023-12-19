#![allow(dead_code, unused_variables)]

use raster::Color;
// use rayon::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl From<&str> for Direction {
    fn from(input: &str) -> Direction {
        use Direction::*;
        match input {
            "R" => Right,
            "D" => Down,
            "U" => Up,
            "L" => Left,
            _ => panic!("Only four directions are supported"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn advance(&self, direction: &Direction, amount: usize) -> Pos {
        let amount = amount as i32;
        match direction {
            Direction::Up => Pos {
                x: self.x,
                y: self.y - amount,
            },
            Direction::Down => Pos {
                x: self.x,
                y: self.y + amount,
            },
            Direction::Right => Pos {
                x: self.x + amount,
                y: self.y,
            },
            Direction::Left => Pos {
                x: self.x - amount,
                y: self.y,
            },
        }
    }
}

#[derive(Clone, Debug)]
struct Trench {
    start: Pos,
    direction: Direction,
    color: Color,
    length: usize,
}

impl Trench {
    fn get_end(&self) -> Pos {
        self.start.advance(&self.direction, self.length)
    }
}

fn shoelace(trenches: &[Trench]) -> usize {
    let mut it = trenches.windows(2);
    let mut area = 0;
    while let Some(win) = it.next() {
        area += win[0].start.x * win[1].start.y + win[0].length as i32;
        area -= win[0].start.y * win[1].start.x;
    }
    area as usize / 2 + 1
}

pub fn run(input: &str) -> Result<usize, String> {
    const PARENTHESIS: &[char] = &['(', ')'];
    let mut start_pos = Pos { x: 0, y: 0 };
    let mut trenches: Vec<Trench> = input
        .lines()
        .map(|line| {
            let mut it = line.split(" ");
            let direction = Direction::from(it.next().unwrap());
            let length = it.next().unwrap().parse::<usize>().unwrap();
            let color = Color::hex(it.next().unwrap().trim_matches(PARENTHESIS)).unwrap();
            let trench = Trench {
                direction,
                color,
                length,
                start: start_pos,
            };
            start_pos = trench.get_end();
            trench
        })
        .collect();

    trenches.push(trenches.first().unwrap().clone());

    Ok(shoelace(&trenches))
}
