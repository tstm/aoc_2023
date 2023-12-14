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

fn count_arrangements(mut conditions: Vec<Tile>, groups: Vec<u8>) -> usize {
    use Tile::*;

    conditions.push(Operational);
    let mut cache = vec![vec![None; conditions.len()]; groups.len()];
    count_arrangements_inner(&conditions, &groups, &mut cache)
}

fn count_arrangements_inner(
    conditions: &[Tile],
    groups: &[u8],
    cache: &mut [Vec<Option<usize>>],
) -> usize {
    use Tile::*;
    let mut arrangements = 0;

    if groups.is_empty() {
        return if conditions.contains(&Damaged) { 0 } else { 1 };
    }

    if conditions.len() < groups.iter().sum::<u8>() as usize + groups.len() {
        return 0;
    }

    if let Some(cached) = cache[groups.len() - 1][conditions.len() - 1] {
        return cached;
    }

    if conditions[0] != Damaged {
        arrangements += count_arrangements_inner(&conditions[1..], groups, cache);
    }
    let next_group_size = groups[0] as usize;
    if !conditions[..next_group_size].contains(&Operational)
        && conditions[next_group_size] != Damaged
    {
        arrangements +=
            count_arrangements_inner(&conditions[next_group_size + 1..], &groups[1..], cache);
    }
    cache[groups.len() - 1][conditions.len() - 1] = Some(arrangements);
    arrangements
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
            count_arrangements(conditions, groups)
        })
        .sum();
    Ok(result)
}
