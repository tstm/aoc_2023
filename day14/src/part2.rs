#![allow(dead_code, unused_variables)]
use std::collections::HashMap;

// use rayon::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Rock,
    Wall,
}

impl Tile {
    fn from(item: u8) -> Option<Self> {
        match item {
            b'O' => Some(Self::Rock),
            b'#' => Some(Self::Wall),
            _ => None,
        }
    }

    fn char(&self) -> &str {
        match self {
            Self::Rock => "O",
            Self::Wall => "#",
        }
    }
}

#[derive(PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn support_weight(
    map: &HashMap<(usize, usize), Tile>,
    direction: &Direction,
    rows: &usize,
    cols: &usize,
) -> usize {
    match direction {
        Direction::North => {
            map.iter()
                // Fetch all the rocks with filter
                .filter_map(|(coord, tile)| {
                    if tile == &Tile::Rock {
                        Some(coord)
                    } else {
                        None
                    }
                })
                .map(|(row, col)| rows - row)
                .sum()
        }
        Direction::South => todo!(),
        Direction::West => {
            map.iter()
                // Fetch all the rocks with filter
                .filter_map(|(coord, tile)| {
                    if tile == &Tile::Rock {
                        Some(coord)
                    } else {
                        None
                    }
                })
                .map(|(row, col)| cols - col)
                .sum()
        }
        Direction::East => todo!(),
    }
}

fn next_free(
    coord: &(usize, usize),
    map: &HashMap<(usize, usize), Tile>,
    direction: &Direction,
    rows: &usize,
    cols: &usize,
) -> (usize, usize) {
    let row = coord.0;
    let column = coord.1;

    let new_row = rows - 1;
    let new_col = cols - 1;

    match direction {
        Direction::North => match (0..=row).rev().find(|r| match map.get(&(*r, column)) {
            Some(_) => true,
            None => false,
        }) {
            Some(r) => (r + 1, column),
            None => (0, column),
        },
        Direction::South => match ((row + 1)..*rows).find(|r| match map.get(&(*r, column)) {
            Some(_) => true,
            None => false,
        }) {
            Some(r) => (r - 1, column),
            None => (new_row, column),
        },
        Direction::East => match (column..*cols).find(|c| match map.get(&(row, *c)) {
            Some(_) => true,
            None => false,
        }) {
            Some(c) => (row, c - 1),
            None => (row, new_col),
        },
        Direction::West => match (0..=column).rev().find(|c| match map.get(&(row, *c)) {
            Some(_) => true,
            None => false,
        }) {
            Some(c) => (row, c + 1),
            None => (row, 0),
        },
    }
}

fn print_map(map: &HashMap<(usize, usize), Tile>, rows: &usize, cols: &usize) {
    for row in 0..*rows {
        let mut line = "".to_string();
        for col in 0..*cols {
            match map.get(&(row, col)) {
                Some(c) => line += c.char(),
                None => line += ".",
            }
        }
        println!("{}", line);
    }
}

fn move_stones(
    map: &mut HashMap<(usize, usize), Tile>,
    direction: &Direction,
    rows: &usize,
    cols: &usize,
) {
    let mut rocks = map
        .iter()
        // Fetch all the rocks with filter
        .filter_map(|(coord, tile)| {
            if tile == &Tile::Rock {
                Some(coord.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    match direction {
        Direction::North => rocks.sort_by(|(r1, c1), (r2, c2)| r1.cmp(r2)),
        Direction::South => rocks.sort_by(|(r1, c1), (r2, c2)| r2.cmp(r1)),
        Direction::East => rocks.sort_by(|(r1, c1), (r2, c2)| c2.cmp(c1)),
        Direction::West => rocks.sort_by(|(r1, c1), (r2, c2)| c1.cmp(c2)),
    };
    // dbg!(&rocks);

    // Remove all rocks first
    for coord in &rocks {
        map.remove(coord);
    }

    for coord in rocks {
        // Move the stones by inserting and removing to the hashmap
        let next = next_free(&coord, map, direction, rows, cols);
        // eprintln!("Moved rock from {:?} to {:?}", &coord, &next);
        map.insert(next, Tile::Rock);
    }
}

fn cycle_return_weights(
    map: &mut HashMap<(usize, usize), Tile>,
    rows: &usize,
    cols: &usize,
) -> [usize; 2] {
    move_stones(map, &Direction::North, &rows, &cols);
    move_stones(map, &Direction::West, &rows, &cols);
    move_stones(map, &Direction::South, &rows, &cols);
    move_stones(map, &Direction::East, &rows, &cols);

    [
        support_weight(&map, &Direction::North, &rows, &cols),
        support_weight(&map, &Direction::West, &rows, &cols),
    ]
}

fn brent_search(map: &HashMap<(usize, usize), Tile>, rows: &usize, cols: &usize) -> (usize, usize) {
    let mut tortoise_map;
    let mut hare_map = map.clone();

    let mut power = 1;
    let mut lam = 1;
    let mut tortoise = [
        support_weight(map, &Direction::North, &rows, &cols),
        support_weight(map, &Direction::West, &rows, &cols),
    ];
    let mut hare = cycle_return_weights(&mut hare_map, rows, cols); // f(x0) is the element/node next to x0.
    while tortoise != hare {
        if power == lam {
            // tortoise_map = hare_map.clone();
            tortoise = hare;
            power = power * 2;
            lam = 0;
        }
        hare = cycle_return_weights(&mut hare_map, rows, cols);
        lam += 1;
    }

    // Find the position of the first repetition of length λ
    tortoise_map = map.clone();
    hare_map = map.clone();
    tortoise = [
        support_weight(map, &Direction::North, &rows, &cols),
        support_weight(map, &Direction::West, &rows, &cols),
    ];
    hare = [
        support_weight(map, &Direction::North, &rows, &cols),
        support_weight(map, &Direction::West, &rows, &cols),
    ];

    for i in 0..lam {
        hare = cycle_return_weights(&mut hare_map, rows, cols);
    }

    // Next, the hare and tortoise move at same speed until they agree
    let mut mu = 0;
    while tortoise != hare {
        tortoise = cycle_return_weights(&mut tortoise_map, rows, cols);
        hare = cycle_return_weights(&mut hare_map, rows, cols);
        mu += 1;
    }

    (lam, mu)
}

pub fn run(input: &str) -> Result<usize, String> {
    let mut rows: usize = 0;
    let mut cols: usize = 0;

    let mut map = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            rows += 1;
            let line: Vec<_> = line
                .bytes()
                .enumerate()
                .flat_map(|(col, b)| {
                    if col >= cols {
                        cols = col + 1;
                    }
                    match Tile::from(b) {
                        Some(tile) => Some((col, tile)),
                        None => None,
                    }
                })
                .collect();
            line.into_iter()
                .map(|(col, tile)| ((row, col), tile))
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<(usize, usize), Tile>>();

    // print_map(&map, &rows, &cols);
    let mut cycles = 0;

    let (lam, mu) = brent_search(&map, &rows, &cols);
    let remaining_cycles = ((1000000000 - mu) % lam) + mu;

    for i in 0..remaining_cycles {
        move_stones(&mut map, &Direction::North, &rows, &cols);
        move_stones(&mut map, &Direction::West, &rows, &cols);
        move_stones(&mut map, &Direction::South, &rows, &cols);
        move_stones(&mut map, &Direction::East, &rows, &cols);

        cycles += 1;
    }
    let weight = support_weight(&map, &Direction::North, &rows, &cols);
    Ok(weight)
}
