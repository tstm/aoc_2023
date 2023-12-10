#![allow(dead_code, unused_variables)]

use colored::Colorize;
use std::collections::HashMap;

use glam::IVec2;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use rayon::prelude::*;

#[derive(PartialEq, Debug)]
enum PipeType {
    Vertical,
    Horizontal,
    BendNorthEast,
    BendNorthWest,
    BendSouthWest,
    BendSouthEast,
    Start,
}

impl PipeType {
    fn from_char(c: char) -> Option<PipeType> {
        match c {
            '|' => Some(Self::Vertical),
            '-' => Some(Self::Horizontal),
            'L' => Some(Self::BendNorthEast),
            'J' => Some(Self::BendNorthWest),
            '7' => Some(Self::BendSouthWest),
            'F' => Some(Self::BendSouthEast),
            'S' => Some(Self::Start),
            _ => None,
        }
    }
}

#[derive(PartialEq, Debug, EnumIter)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug)]
struct PipeSegment {
    pos: IVec2,
    variant: PipeType,
}

impl PipeSegment {
    fn print(&self) -> &str {
        match self.variant {
            PipeType::Vertical => "║",
            PipeType::Horizontal => "═",
            PipeType::BendNorthEast => "╚",
            PipeType::BendNorthWest => "╝",
            PipeType::BendSouthWest => "╗",
            PipeType::BendSouthEast => "╔",
            PipeType::Start => "S",
        }
    }

    fn get_loop<'a>(
        &'a self,
        map: &'a HashMap<IVec2, PipeSegment>,
        direction: Direction,
    ) -> Option<Vec<&PipeSegment>> {
        use Direction::*;
        use PipeType::*;

        let mut retval = vec![];
        let mut prev_coord = self.pos;
        let mut next = match map.get(&self.move_one(&direction)) {
            Some(n) => n,
            None => return None,
        };
        // dbg!(map.get(&next.next(prev_coord)));
        // dbg!(map.get(&self.move_one(&direction)));

        retval.push(match next.variant {
            Vertical => match direction {
                North | South => next,
                _ => return None,
            },
            Horizontal => match direction {
                East | West => next,
                _ => return None,
            },
            BendNorthEast => match direction {
                South | West => next,
                _ => return None,
            },
            BendNorthWest => match direction {
                South | East => next,
                _ => return None,
            },
            BendSouthWest => match direction {
                North | East => next,
                _ => return None,
            },
            BendSouthEast => match direction {
                West | North => next,
                _ => return None,
            },
            Start => todo!(),
        });

        while next.variant != Start {
            // dbg!(map.get(&next.next(prev_coord)));
            next = match map.get(&next.next(prev_coord)) {
                Some(n) => n,
                None => return None,
            };
            prev_coord = retval.last().unwrap().pos;
            retval.push(next);
        }

        Some(retval)
    }

    fn connections(&self) -> Vec<IVec2> {
        use PipeType::*;

        match self.variant {
            Vertical => vec![
                IVec2::new(self.pos.x, self.pos.y - 1),
                IVec2::new(self.pos.x, self.pos.y + 1),
            ],
            Horizontal => vec![
                IVec2::new(self.pos.x - 1, self.pos.y),
                IVec2::new(self.pos.x + 1, self.pos.y),
            ],
            BendNorthEast => vec![
                IVec2::new(self.pos.x, self.pos.y - 1),
                IVec2::new(self.pos.x + 1, self.pos.y),
            ],
            BendNorthWest => vec![
                IVec2::new(self.pos.x, self.pos.y - 1),
                IVec2::new(self.pos.x - 1, self.pos.y),
            ],
            BendSouthWest => vec![
                IVec2::new(self.pos.x, self.pos.y + 1),
                IVec2::new(self.pos.x - 1, self.pos.y),
            ],
            BendSouthEast => vec![
                IVec2::new(self.pos.x, self.pos.y + 1),
                IVec2::new(self.pos.x + 1, self.pos.y),
            ],
            Start => vec![
                IVec2::new(self.pos.x, self.pos.y),
                IVec2::new(self.pos.x, self.pos.y),
            ],
        }
    }

    fn next(&self, prev: IVec2) -> IVec2 {
        // dbg!(&prev);
        // dbg!(self.connections().into_iter().find(|p| &prev != p).unwrap());
        self.connections().into_iter().find(|p| &prev != p).unwrap()
    }

    fn move_one(&self, direction: &Direction) -> IVec2 {
        match direction {
            Direction::North => IVec2::new(self.pos.x, self.pos.y - 1),
            Direction::South => IVec2::new(self.pos.x, self.pos.y + 1),
            Direction::West => IVec2::new(self.pos.x - 1, self.pos.y),
            Direction::East => IVec2::new(self.pos.x + 1, self.pos.y),
        }
    }
}

pub fn run(input: &str) -> Result<usize, String> {
    let mut start_position = IVec2::new(0, 0);
    let map = (input.lines().enumerate().map(|(y, line)| {
        let y = y as i32;
        line.chars()
            .enumerate()
            .flat_map(|(x, c)| {
                let x = x as i32;
                let pipe_type = match PipeType::from_char(c) {
                    None => return None,
                    Some(p) => p,
                };
                if pipe_type == PipeType::Start {
                    start_position = IVec2::new(x, y);
                }
                Some((
                    IVec2::new(x, y),
                    PipeSegment {
                        pos: IVec2::new(x, y),
                        variant: pipe_type,
                    },
                ))
            })
            .collect::<Vec<(IVec2, PipeSegment)>>()
    }))
    .flatten()
    .collect::<HashMap<IVec2, PipeSegment>>();

    // dbg!(map);
    // dbg!(start_position);
    // for direction in Direction::iter() {
    //     let start = map.get(&start_position).unwrap();
    //     dbg!(start.get_loop(&map, direction));
    // }
    // let max = Direction::iter()
    //     .filter(|x| x == &Direction::South)
    //     .map(|direction| {
    //         let start = map.get(&start_position).unwrap();
    //         start.get_loop(&map, direction)
    //     })
    //     .flatten()
    //     .map(|l| {
    //         println!("");
    //         println!("");
    //         let max_x = map.keys().map(|pos| pos.x).max().unwrap();
    //         let max_y = map.keys().map(|pos| pos.y).max().unwrap();
    //         // println!("Max x: {} Max y: {}", max_x, max_y);
    //         for y in 0..=max_y {
    //             // let mut line = ColoredString::from("");
    //             let mut line = "".to_string();
    //             // print!("{} ", y);
    //             // std::io::stdout().flush().unwrap();
    //             for x in 0..=max_x {
    //                 let coord = IVec2::new(x, y);
    //                 let output = match map.get(&coord) {
    //                     Some(n) => match l.iter().find(|pipe| pipe.pos == coord) {
    //                         Some(_) => n.print().green(),
    //                         None => n.print().bold(),
    //                     },
    //                     None => " ".bold(),
    //                 };
    //                 line = format!("{}{}", line, output);
    //                 // print!("{}", output);
    //                 // std::io::stdout().flush().unwrap();
    //             }
    //             println!("{}", line);
    //         }
    //         println!("Length: {}", l.len());
    //         l.len()
    //     })
    //     .max()
    //     .unwrap();
    let max = [
        Direction::West,
        Direction::East,
        Direction::North,
        Direction::South,
    ]
    .into_par_iter()
    .flat_map(|direction| {
        let start = map.get(&start_position).unwrap();
        start.get_loop(&map, direction)
    })
    .max_by_key(|x| x.len())
    .unwrap();

    Ok(max.len() / 2)
}
