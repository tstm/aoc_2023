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

    fn get_walls(&self) -> Vec<Pos> {
        let end = self.get_end();

        match self.direction {
            Direction::Up => (end.y..self.start.y)
                .into_iter()
                .map(|y| Pos { x: self.start.x, y })
                .collect(),
            Direction::Down => ((self.start.y)..=end.y)
                .into_iter()
                .map(|y| Pos { x: self.start.x, y })
                .collect(),
            Direction::Right => ((self.start.x + 1)..=end.x)
                .into_iter()
                .map(|x| Pos { x, y: self.start.y })
                .collect(),
            Direction::Left => (end.x..self.start.x)
                .into_iter()
                .map(|x| Pos { x, y: self.start.y })
                .collect(),
        }
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

fn get_interior_size(grid: &Vec<Vec<char>>) -> usize {
    let mut interior_size = 0;
    for line in grid {
        let mut wall_count = 0;
        let mut prev_wall = false;
        for c in line {
            match c {
                '.' => {
                    if wall_count % 2 == 1 {
                        interior_size += 1;
                    }
                    prev_wall = false;
                }
                '#' => {
                    if !prev_wall {
                        wall_count += 1;
                    }
                    interior_size += 1;
                    prev_wall = true;
                }
                _ => panic!("Went out of bounds?"),
            }
        }
    }
    interior_size
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

    // let walls: Vec<Pos> = trenches.iter().flat_map(Trench::get_walls).collect();
    //
    // let min_x = walls.iter().min_by_key(|t| t.x).unwrap().x;
    // let min_y = walls.iter().min_by_key(|t| t.y).unwrap().y;
    // let max_x = walls.iter().max_by_key(|t| t.x).unwrap().x;
    // let max_y = walls.iter().max_by_key(|t| t.y).unwrap().y;
    //
    // let x_compensation = if min_x < 0 { min_x.abs() } else { 0 };
    // let y_compensation = if min_y < 0 { min_y.abs() } else { 0 };
    //
    // println!(
    //     "Min x: {}, Min y: {}, Max x: {}, Max y: {}",
    //     min_x, min_y, max_x, max_y
    // );
    //
    // let mut grid: Vec<Vec<char>> = (0..=(max_y + y_compensation))
    //     .map(|_| vec!['.'; max_x as usize + 1 + x_compensation as usize])
    //     .collect();
    // walls
    //     .iter()
    //     .for_each(|w| grid[(w.y + y_compensation) as usize][(w.x + x_compensation) as usize] = '#');
    //
    // let printout = grid
    //     .iter()
    //     .map(|line| line.iter().map(|c| c).collect::<String>())
    //     .collect::<Vec<String>>()
    //     .join("\n");
    // println!("{}", printout);

    // Add the last part again to get shoelace to wrap
    trenches.push(trenches.first().unwrap().clone());

    Ok(shoelace(&trenches))
}
