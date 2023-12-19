// #![allow(dead_code, unused_variables)]

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
            "0" => Right,
            "1" => Down,
            "3" => Up,
            "2" => Left,
            _ => panic!("Only four directions are supported, given: {}", input),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn advance(&self, direction: &Direction, amount: usize) -> Pos {
        let amount = amount as i64;
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
        area += win[0].start.x * win[1].start.y + win[0].length as i64;
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
            let mut it = line.split(" ").skip(2);
            let instructions = it.next().unwrap().trim_matches(PARENTHESIS);
            let length = usize::from_str_radix(&instructions[1..6], 16).unwrap();
            let direction = Direction::from(&instructions[6..=6]);
            let trench = Trench {
                direction,
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
