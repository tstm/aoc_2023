use rayon::prelude::*;
use std::collections::BTreeSet;

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
                .map(|n| n.parse::<isize>().expect("Parse number failed"))
                .collect::<BTreeSet<isize>>();

            let had_numbers = had_row
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<isize>().expect("Parse number failed"))
                .collect::<BTreeSet<isize>>();

            match winning_numbers.intersection(&had_numbers).count() {
                0 => 0,
                count => 2usize.pow(count as u32 - 1),
            }
        })
        .sum())
}
