#![allow(dead_code, unused_variables)]
use std::collections::HashMap;

// use rayon::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Tile {
    Empty,
    RightMirror,
    LeftMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl Tile {
    fn from(item: u8) -> Self {
        match item {
            b'.' => Self::Empty,
            b'/' => Self::RightMirror,
            b'\\' => Self::LeftMirror,
            b'|' => Self::VerticalSplitter,
            b'-' => Self::HorizontalSplitter,
            _ => panic!("There should always be a parseable tile"),
        }
    }

    fn char(&self) -> &str {
        match self {
            Tile::Empty => ".",
            Tile::RightMirror => "/",
            Tile::LeftMirror => "\\",
            Tile::VerticalSplitter => "|",
            Tile::HorizontalSplitter => "-",
        }
    }
}

#[derive(Clone, Debug)]
struct LightRay {
    x: i32,
    y: i32,
    direction: Direction,
    history: Option<Vec<LightRay>>,
}
impl PartialEq for LightRay {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.direction == other.direction
    }
}

impl LightRay {
    fn is_moving(&self, grid: &Grid) -> bool {
        let height = grid.map.len() as i32;
        let width = grid.map[0].len() as i32;

        (0..width).contains(&self.x) && (0..height).contains(&self.y)
    }

    fn advance(
        &mut self,
        grid: &Grid,
        seen: &mut HashMap<(usize, usize), Vec<Direction>>,
    ) -> Option<Self> {
        // let height = grid.len();
        // let width = grid[0].len();
        //
        // if self.x >= width as i32 || self.y >= height as i32 || self.x < 0 || self.y < 0 {
        //     return None;
        // }

        match grid.map[self.y as usize][self.x as usize] {
            Tile::Empty => self.move_light(seen),
            Tile::RightMirror => {
                match self.direction {
                    Direction::Down => self.direction = Direction::Left,
                    Direction::Right => self.direction = Direction::Up,
                    Direction::Up => self.direction = Direction::Right,
                    Direction::Left => self.direction = Direction::Down,
                }
                self.move_light(seen);
            }
            Tile::LeftMirror => {
                match self.direction {
                    Direction::Down => self.direction = Direction::Right,
                    Direction::Right => self.direction = Direction::Down,
                    Direction::Up => self.direction = Direction::Left,
                    Direction::Left => self.direction = Direction::Up,
                }
                self.move_light(seen);
            }
            Tile::VerticalSplitter => match self.direction {
                Direction::Left | Direction::Right => {
                    self.direction = Direction::Up;
                    let mut new_ray = LightRay {
                        x: self.x,
                        y: self.y,
                        direction: Direction::Down,
                        history: Some(vec![]),
                    };
                    self.move_light(seen);
                    new_ray.move_light(seen);
                    return Some(new_ray);
                }
                _ => self.move_light(seen),
            },
            Tile::HorizontalSplitter => match self.direction {
                Direction::Down | Direction::Up => {
                    self.direction = Direction::Right;
                    let mut new_ray = LightRay {
                        x: self.x,
                        y: self.y,
                        direction: Direction::Left,
                        history: Some(vec![]),
                    };
                    self.move_light(seen);
                    new_ray.move_light(seen);
                    return Some(new_ray);
                }
                _ => self.move_light(seen),
            },
        };
        None
    }

    fn move_light(&mut self, seen: &mut HashMap<(usize, usize), Vec<Direction>>) {
        // self.history.as_mut().unwrap().push(LightRay {
        //     x: self.x.clone(),
        //     y: self.y.clone(),
        //     direction: self.direction.clone(),
        //     history: None,
        // });
        let coords = (self.x as usize, self.y as usize);
        match seen.get_mut(&coords) {
            Some(seen) => seen.push(self.direction),
            None => {
                seen.insert(coords, vec![self.direction]);
            }
        };
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

struct Grid {
    map: Vec<Vec<Tile>>,
}

pub fn run(input: &str) -> Result<usize, String> {
    let mut rays: Vec<LightRay> = vec![];
    let mut seen = HashMap::new();

    let grid = Grid {
        map: input
            .lines()
            .map(|line| line.bytes().map(Tile::from).collect())
            .collect(),
    };

    rays.push(LightRay {
        x: 0,
        y: 0,
        direction: Direction::Right,
        history: Some(vec![]),
    });

    loop {
        // let moving: usize = rays.iter().filter(|r| r.is_moving(&grid)).count();
        if rays.len() > 0 {
            let new_rays: Vec<LightRay> = rays
                .iter_mut()
                .filter(|r| r.is_moving(&grid))
                .flat_map(|r| r.advance(&grid, &mut seen))
                .collect();

            while let Some(ray) =
                rays.iter()
                    .position(|r| match seen.get(&(r.x as usize, r.y as usize)) {
                        Some(directions) => directions.contains(&r.direction),
                        None => false,
                    } || !r.is_moving(&grid))
            {
                rays.remove(ray);
            }

            new_rays.into_iter().for_each(|r| rays.push(r));
        } else {
            break;
        }
    }

    let height = grid.map.len();
    let width = grid.map[0].len();
    for y in 0..height {
        let mut line = "".to_string();
        for x in 0..width {
            if grid.map[y][x] != Tile::Empty {
                line += grid.map[y][x].char();
            } else {
                match seen.get(&(x, y)) {
                    Some(d) => {
                        if d.len() == 1 {
                            match d.first().unwrap() {
                                Direction::Left => line += "<",
                                Direction::Right => line += ">",
                                Direction::Up => line += "^",
                                Direction::Down => line += "v",
                            }
                        } else {
                            line += &d.len().to_string();
                        }
                    }
                    None => line += ".",
                }
            }
        }
        println!("{}", line);
    }
    match seen.keys().find(|k| k.0 >= width || k.1 >= height) {
        Some(_) => println!("Found a culprit!"),
        None => {}
    }

    // dbg!(&seen);
    // let mut coords: Vec<_> = rays
    //     .into_iter()
    //     .flat_map(|r| {
    //         r.history
    //             .unwrap()
    //             .iter()
    //             .map(|h| (h.x, h.y))
    //             .collect::<Vec<_>>()
    //     })
    //     .collect();
    // coords.sort();
    // coords.dedup();
    //
    // let energized = coords.len();
    let energized = seen.len();

    Ok(energized)
}
