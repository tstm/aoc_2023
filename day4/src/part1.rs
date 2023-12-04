use rayon::prelude::*;
// use std::collections::{BTreeSet, HashMap, HashSet};

pub fn part1(input: &str) -> Result<usize, String> {
    Ok(input
        .par_lines()
        .map(|line| {
            let (winning_row, had_row) = line
                .split_once(": ")
                .expect("There should be :")
                .1
                .split_once(" | ")
                .expect("Row split failed");

            let winning_numbers = winning_row
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<usize>().expect("Parse number failed"))
                .collect::<Vec<usize>>();

            let count = had_row
                .trim()
                .split_whitespace()
                .filter(|n| {
                    winning_numbers.contains(&n.parse::<usize>().expect("Parse number failed"))
                })
                .count();

            match count {
                0 => 0,
                count => 2usize.pow(count as u32 - 1),
            }
        })
        .sum())
}
