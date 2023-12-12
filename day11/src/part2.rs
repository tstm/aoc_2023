#![allow(dead_code, unused_variables)]
use glam::IVec2;
use itertools::Itertools;

#[derive(Debug)]
struct Galaxy {
    coord: IVec2,
}

impl Galaxy {
    fn distance(&self, other: &Galaxy) -> usize {
        let delta_x = (self.coord.x - other.coord.x).abs();
        let delta_y = (self.coord.y - other.coord.y).abs();
        delta_x as usize + delta_y as usize
    }
}

#[derive(Debug)]
struct Map {
    galaxies: Vec<Galaxy>,
    height: usize,
    width: usize,
}

impl Map {
    fn new(height: usize, width: usize) -> Map {
        Map {
            galaxies: vec![],
            height,
            width,
        }
    }

    fn print(&self) {
        for y in 0..self.height {
            let mut line = "".to_string();
            for x in 0..self.width {
                if self
                    .galaxies
                    .iter()
                    .find(|g| g.coord == IVec2::new(x as i32, y as i32))
                    .is_some()
                {
                    line += "#";
                } else {
                    line += ".";
                }
            }
            println!("{}", line);
        }
    }

    fn add_galaxy(&mut self, x: i32, y: i32) {
        let new_galaxy = Galaxy {
            coord: IVec2::new(x, y),
        };
        self.galaxies.push(new_galaxy);
    }

    fn fix_distortions(self, x_dist: Vec<i32>, y_dist: Vec<i32>, multiplier: i32) -> Map {
        let galaxies = self
            .galaxies
            .into_iter()
            .map(|mut galaxy| {
                let xmove: i32 = x_dist
                    .iter()
                    .map(|dist| {
                        if galaxy.coord.x > *dist {
                            1 * multiplier - 1
                        } else {
                            0
                        }
                    })
                    .sum();
                let ymove: i32 = y_dist
                    .iter()
                    .map(|dist| match galaxy.coord.y > *dist {
                        true => 1 * multiplier - 1,
                        false => 0,
                    })
                    .sum();
                galaxy.coord.x += xmove;
                galaxy.coord.y += ymove;
                galaxy
            })
            .collect();

        Map {
            galaxies,
            height: self.height + y_dist.len() * multiplier as usize - y_dist.len(),
            width: self.width + x_dist.len() * multiplier as usize - x_dist.len(),
        }
    }
}

pub fn run(input: &str, multiplier: i32) -> Result<usize, String> {
    let mut map = Map::new(0, 0);
    let mut x_distortions = vec![];
    let mut y_distortions = vec![];

    let raw_map: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => {
                        map.add_galaxy(x as i32, y as i32);
                        c
                    }
                    _ => c,
                })
                .collect()
        })
        .collect();

    map.width = raw_map[0].len();
    map.height = raw_map.len();

    for y in 0..map.height {
        if raw_map[y].iter().all(|c| *c != '#') {
            y_distortions.push(y as i32);
        }
    }

    for x in 0..map.width {
        if (0..map.height).all(|y| raw_map[y][x] != '#') {
            x_distortions.push(x as i32);
        }
    }
    map = map.fix_distortions(x_distortions, y_distortions, multiplier);

    let sum = map
        .galaxies
        .iter()
        .combinations(2)
        .map(|combination| combination[0].distance(combination[1]))
        .sum();

    Ok(sum)
}
