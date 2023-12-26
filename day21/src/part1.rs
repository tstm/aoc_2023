#![allow(dead_code, unused_variables)]

use std::collections::HashSet;

use derive_more::Deref;

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
#[derive(Deref)]
struct Grid(Vec<Vec<Tile>>);

impl Grid {
    fn get_tile(&self, pos: &Pos) -> &Tile {
        let height = self.len();
        let width = self[0].len();
        let y = pos.y % height;
        let x = pos.x % width;
        &self[pos.y][pos.x]
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Pos {
    x: usize,
    y: usize,
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

    fn successors(&self, grid: &Grid) -> Vec<Self> {
        let mut successors = vec![];
        for tile_position in self.neighbors() {
            match grid.get_tile(&tile_position) {
                Tile::Garden => successors.push(tile_position),
                Tile::Rock => {}
                Tile::Start => panic!("There should be no star tile"),
            }
        }

        successors
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

pub fn run(input: (&str, usize)) -> Result<usize, String> {
    let mut start = Pos { x: 0, y: 0 };
    let steps = input.1;
    let input = input.0;

    let grid = Grid(
        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.bytes()
                    .enumerate()
                    .map(|(x, b)| match Tile::from(b) {
                        Tile::Start => {
                            start = Pos { x, y };
                            Tile::Garden
                        }
                        tile => tile,
                    })
                    .collect()
            })
            .collect(),
    );
    let mut plots = HashSet::new();
    plots.insert(start);

    for _ in 0..steps {
        plots = plots
            .into_iter()
            .flat_map(|plot| plot.successors(&grid))
            .collect();
    }

    Ok(plots.len())
}
