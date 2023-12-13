#![allow(dead_code, unused_variables)]
// use glam::IVec2;
// use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Tile {
    fn from(item: char) -> Self {
        match item {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!("We only support three tiles"),
        }
    }
}

fn check_match(conditions: &Vec<Tile>, groups: &Vec<u8>) -> bool {
    let mut continuous: u8 = 0;
    let mut matchgroups: Vec<_> = vec![];

    for tile in conditions {
        if tile == &Tile::Damaged {
            continuous += 1;
        } else if continuous != 0 {
            matchgroups.push(continuous);
            continuous = 0;
        }
    }
    if continuous != 0 {
        matchgroups.push(continuous);
    }
    groups == &matchgroups
}

fn count_arrangements(conditions: Vec<Tile>, groups: &Vec<u8>) -> usize {
    use Tile::*;

    // let mut c = conditions.clone();
    if let Some(first_index) = conditions.iter().position(|tile| tile == &Unknown) {
        let mut c1 = conditions.clone();
        let mut c2 = conditions.clone();
        c1[first_index] = Operational;
        c2[first_index] = Damaged;
        count_arrangements(c1, groups) + count_arrangements(c2, groups)
    } else {
        match check_match(&conditions, groups) {
            true => 1,
            false => 0,
        }
    }
}

pub fn run(input: &str) -> Result<usize, String> {
    let result = input
        .lines()
        .map(|line| {
            let (conditions, groups) = line.split_once(" ").unwrap();

            let conditions: Vec<_> = conditions.chars().map(|c| Tile::from(c)).collect();
            let groups: Vec<_> = groups
                .split(",")
                .map(|c| c.parse::<u8>().unwrap())
                .collect();
            count_arrangements(conditions, &groups)
        })
        .sum();
    Ok(result)
}
