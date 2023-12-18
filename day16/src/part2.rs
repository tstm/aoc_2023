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
        seen: &mut HashMap<(i32, i32), Vec<Direction>>,
    ) -> Option<Self> {
        // let height = grid.len();
        // let width = grid[0].len();
        //
        // if self.x >= width as i32 || self.y >= height as i32 || self.x < 0 || self.y < 0 {
        //     return None;
        // }

        let coords = (self.x.clone(), self.y.clone());
        match seen.get_mut(&coords) {
            Some(seen) => seen.push(self.direction),
            None => {
                seen.insert(coords, vec![self.direction]);
            }
        };

        match grid.map[self.y as usize][self.x as usize] {
            Tile::Empty => self.move_light(seen),
            Tile::RightMirror => {
                match self.direction {
                    Direction::Down => self.direction = Direction::Left,
                    Direction::Right => self.direction = Direction::Up,
                    Direction::Up => self.direction = Direction::Right,
                    Direction::Left => self.direction = Direction::Down,
                };
                self.move_light(seen);
            }
            Tile::LeftMirror => {
                match self.direction {
                    Direction::Down => self.direction = Direction::Right,
                    Direction::Right => self.direction = Direction::Down,
                    Direction::Up => self.direction = Direction::Left,
                    Direction::Left => self.direction = Direction::Up,
                };
                self.move_light(seen);
            }
            Tile::VerticalSplitter => match self.direction {
                Direction::Left | Direction::Right => {
                    self.direction = Direction::Up;
                    let mut new_ray = LightRay {
                        x: self.x,
                        y: self.y,
                        direction: Direction::Down,
                    };
                    self.move_light(seen);
                    // new_ray.move_light(seen);
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
                    };
                    self.move_light(seen);
                    // new_ray.move_light(seen);
                    return Some(new_ray);
                }
                _ => self.move_light(seen),
            },
        };
        None
    }

    fn move_light(&mut self, seen: &mut HashMap<(i32, i32), Vec<Direction>>) {
        // self.history.as_mut().unwrap().push(LightRay {
        //     x: self.x.clone(),
        //     y: self.y.clone(),
        //     direction: self.direction.clone(),
        //     history: None,
        // });
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
    let grid = Grid {
        map: input
            .lines()
            .map(|line| line.bytes().map(Tile::from).collect())
            .collect(),
    };

    let height = grid.map.len();
    let width = grid.map[0].len();

    let mut initial_rays: Vec<LightRay> = vec![];
    (0..width).for_each(|x| {
        initial_rays.push(LightRay {
            x: x as i32,
            y: 0,
            direction: Direction::Down,
        });
        initial_rays.push(LightRay {
            x: x as i32,
            y: height as i32 - 1,
            direction: Direction::Up,
        });
    });

    (0..height).for_each(|y| {
        initial_rays.push(LightRay {
            x: 0,
            y: y as i32,
            direction: Direction::Right,
        });
        initial_rays.push(LightRay {
            x: width as i32 - 1,
            y: y as i32,
            direction: Direction::Left,
        });
    });

    let max: usize = initial_rays
        .into_iter()
        .map(|initial_ray| {
            let mut rays: Vec<LightRay> = vec![];
            let mut seen = HashMap::new();

            rays.push(initial_ray);

            while rays.len() > 0 {
                let mut new_rays: Vec<LightRay> = rays
                    .iter_mut()
                    .filter(|r| r.is_moving(&grid))
                    .flat_map(|r| r.advance(&grid, &mut seen))
                    .collect();

                rays = rays.into_iter().filter(|r| match seen.get(&(r.x, r.y)) {
            Some(directions) => !directions.contains(&r.direction),
            None => true
        }&& r.is_moving(&grid)).collect();
                rays.append(&mut new_rays);
            }

            seen.len()
        })
        .max()
        .unwrap();

    // let height = grid.map.len();
    // let width = grid.map[0].len();
    // for y in 0..height {
    //     let mut line = "".to_string();
    //     for x in 0..width {
    //         if grid.map[y][x] != Tile::Empty {
    //             line += grid.map[y][x].char();
    //         } else {
    //             match seen.get(&(x as i32, y as i32)) {
    //                 Some(d) => {
    //                     if d.len() == 1 {
    //                         match d.first().unwrap() {
    //                             Direction::Left => line += "<",
    //                             Direction::Right => line += ">",
    //                             Direction::Up => line += "^",
    //                             Direction::Down => line += "v",
    //                         }
    //                     } else {
    //                         line += &d.len().to_string();
    //                     }
    //                 }
    //                 None => line += ".",
    //             }
    //         }
    //     }
    //     println!("{}", line);
    // }
    // let energized = seen.len();

    Ok(max)
}
