#![allow(dead_code, unused_variables)]

use rayon::prelude::IntoParallelIterator;
use std::collections::HashMap;

use glam::IVec2;
use strum_macros::EnumIter;

use rayon::prelude::*;

#[derive(PartialEq, Debug, Copy, Clone)]
enum PipeType {
    Vertical,
    Horizontal,
    BendNorthEast,
    BendNorthWest,
    BendSouthWest,
    BendSouthEast,
    Start,
    Floor,
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
            '.' => Some(Self::Floor),
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

#[derive(Debug, Clone, Copy)]
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
            PipeType::Floor => ".",
        }
    }

    fn get_loop<'a>(
        &'a self,
        map: &'a HashMap<IVec2, PipeSegment>,
        direction: Direction,
    ) -> Option<HashMap<IVec2, &PipeSegment>> {
        use Direction::*;
        use PipeType::*;

        let mut retval = HashMap::new();
        let mut prev_coord = self.pos;
        let mut next = match map.get(&self.move_one(&direction)) {
            Some(n) => n,
            None => return None,
        };
        // dbg!(map.get(&next.next(prev_coord)));
        // dbg!(map.get(&self.move_one(&direction)));

        retval.insert(
            next.pos,
            match next.variant {
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
                Floor => return None,
            },
        );

        let last: PipeSegment;
        while next.variant != Start {
            let last = next;
            next = match map.get(&next.next(prev_coord)) {
                Some(n) => n,
                None => return None,
            };
            prev_coord = last.pos;
            retval.insert(next.pos, next);
        }

        Some(retval)
    }

    fn is_inside(&self, map: &[Vec<(IVec2, PipeSegment)>], max_x: i32, max_y: i32) -> bool {
        use PipeType::*;

        let mut crossings = 0;
        let mut first = Floor;

        for x in self.pos.x..=max_x {
            let c = match map[self.pos.y as usize].get(x as usize) {
                Some(c) => c.1.variant,
                None => continue,
            };
            match (c, first) {
                (Vertical, _) => crossings += 1,
                (BendNorthEast | BendSouthEast, _) => first = c,
                (BendSouthWest, BendNorthEast) => crossings += 1,
                (BendNorthWest, BendSouthEast) => crossings += 1,
                _ => {}
            }
        }

        crossings % 2 == 1
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
            Floor => vec![],
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
    let map = (input.lines().enumerate().flat_map(|(y, line)| {
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
    .collect::<HashMap<IVec2, PipeSegment>>();

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

    // let mut clean_map = map.clone();
    let max_x = map.keys().map(|pos| pos.x).max().unwrap();
    let max_y = map.keys().map(|pos| pos.y).max().unwrap();

    // for (x, y) in iproduct!(0..=max_x, 0..=max_y) {
    // let (extents: Vec<(i32,i32)>, clean_map: HashMap<IVec2, PipeSegment>) = (0..=max_y)
    let extents: Vec<(i32, i32)>;
    let clean_map: Vec<Vec<(IVec2, PipeSegment)>>;
    (extents, clean_map) = (0..=max_y)
        .into_par_iter()
        .map(|y| {
            let mut xmin = 0;
            let mut xmax = 0;
            let retval = (0..=max_x)
                .map(|x| {
                    let coord = IVec2::new(x, y);
                    match map.get(&coord) {
                        Some(n) => match max.get(&coord) {
                            Some(_) => {
                                if xmax < x {
                                    xmax = x;
                                }
                                (coord, *n)
                            }
                            None => {
                                if xmax == 0 {
                                    xmin = x;
                                }
                                let mut r = *n;
                                r.variant = PipeType::Floor;
                                (coord, r)
                                // (*n).variant = PipeType::Floor;
                            }
                        },
                        None => {
                            panic!("Something is wrong");
                        }
                    }
                })
                .collect::<Vec<(IVec2, PipeSegment)>>();
            ((xmin, xmax), retval)
        })
        .collect();

    // let clean_map: HashMap<IVec2, PipeSegment> = clean_map.into_iter().flatten().collect();

    // for y in 0..=max_y {
    //     let mut line = "".to_string();
    //     for x in 0..=max_x {
    //         let coord = IVec2::new(x, y);
    //         let output = match clean_map.get(&coord) {
    //             Some(n) => match n.variant {
    //                 PipeType::Floor => {
    //                     if n.is_inside(&clean_map) {
    //                         n.print().red()
    //                     } else {
    //                         n.print().bold()
    //                     }
    //                 }
    //                 _ => n.print().green(),
    //             },
    //             None => " ".bold(),
    //         };
    //         line = format!("{}{}", line, output)
    //     }
    //     println!("{}", line);
    // }
    let inside = (0..=max_y)
        .into_par_iter()
        .map(|y| {
            (extents[y as usize].0..=extents[y as usize].1)
                .filter(|&x| {
                    // let coord = IVec2::new(x, y);
                    match clean_map[y as usize].get(x as usize) {
                        Some(n) => match n.1.variant {
                            PipeType::Floor => {
                                n.1.is_inside(&clean_map, extents[y as usize].1, max_y)
                            }
                            _ => false,
                        },
                        _ => false,
                    }
                })
                .count()
        })
        .sum();

    Ok(inside)
}
