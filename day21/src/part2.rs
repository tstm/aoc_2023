#![allow(dead_code, unused_variables)]

use rayon::prelude::*;
use std::collections::BTreeSet;

// use derive_more::Deref;

#[derive(PartialEq, Eq)]
enum Tile {
    Start,
    Garden,
    Rock,
}

impl From<u8> for Tile {
    fn from(item: u8) -> Self {
        match item {
            b'.' => Self::Garden,
            b'#' => Self::Rock,
            b'S' => Self::Start,
            _ => panic!("We only support three tiles {}", item),
        }
    }
}
// #[derive(Deref)]
struct Grid {
    grid: Vec<Vec<Tile>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(grid: Vec<Vec<Tile>>) -> Self {
        let height = grid.len();
        let width = grid[0].len();
        Self {
            grid,
            height,
            width,
        }
    }

    fn get_tile(&self, pos: &Pos) -> &Tile {
        &self.grid[pos.y.rem_euclid(self.height as i64) as usize]
            [pos.x.rem_euclid(self.width as i64) as usize]
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn neighbors(&self) -> [Pos; 4] {
        [
            Pos {
                x: self.x,
                y: self.y - 1,
            },
            Pos {
                x: self.x,
                y: self.y + 1,
            },
            Pos {
                x: self.x + 1,
                y: self.y,
            },
            Pos {
                x: self.x - 1,
                y: self.y,
            },
        ]
    }

    fn successors(self, grid: &Grid) -> Vec<Self> {
        let mut successors = vec![];
        for tile_position in self.neighbors() {
            match grid.get_tile(&tile_position) {
                Tile::Garden => {
                    successors.push(tile_position);
                }
                Tile::Rock => {}
                Tile::Start => panic!("There should be no star tile"),
            }
        }

        successors
    }
}

pub fn run(input: (&str, usize)) -> Result<usize, String> {
    let mut start = Pos { x: 0, y: 0 };
    let steps = input.1;
    let input = input.0;

    let grid = Grid::new(
        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.bytes()
                    .enumerate()
                    .map(|(x, b)| match Tile::from(b) {
                        Tile::Start => {
                            start = Pos {
                                x: x as i64,
                                y: y as i64,
                            };
                            Tile::Garden
                        }
                        tile => tile,
                    })
                    .collect()
            })
            .collect(),
    );
    let mut plots = BTreeSet::new();
    plots.insert(start);

    // let printout = (-21..20)
    //     .map(|y| {
    //         let mut line = (-21..20)
    //             .map(|x| match grid.get_tile(&Pos { x, y }) {
    //                 Tile::Start => 'S',
    //                 Tile::Garden => '.',
    //                 Tile::Rock => '#',
    //             })
    //             .collect::<String>();
    //
    //         line.insert_str(0, &format!("{} ", y));
    //         line
    //     })
    //     .collect::<Vec<String>>()
    //     .join("\n");
    // println!("{}", printout);

    let mut resultset = Vec::new();
    let size = grid.height as usize;
    let to_edge = size / 2;

    for count in 1.. {
        plots = plots
            .into_iter()
            .flat_map(|plot| plot.successors(&grid))
            .collect();

        if count == to_edge + size * resultset.len() {
            resultset.push(plots.len());

            if resultset.len() == 3 {
                let n = steps / size;

                let delta0 = resultset[0];
                let delta1 = resultset[1] - delta0;
                let delta2 = resultset[2] - resultset[1] - delta1;

                return Ok(delta0 + delta1 * n + delta2 * ((n * (n - 1)) / 2));
            }
        }
    }

    Err("Broken bruh".to_string())
}
