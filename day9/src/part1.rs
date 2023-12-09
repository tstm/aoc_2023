#![allow(dead_code, unused_variables)]

use rayon::prelude::*;

struct Report {
    history: Vec<isize>,
}

impl Report {
    fn new(input: &str) -> Report {
        Report {
            history: input
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
        }
    }
}

fn diff_vec(vector: &Vec<isize>) -> Vec<isize> {
    vector.windows(2).map(|n| n[1] - n[0]).collect()
}

fn extrapolate_next(history: Vec<isize>) -> isize {
    match history.iter().all(|n| *n == 0) {
        true => 0,
        false => {
            history.last().expect("There should be something to keep")
                + extrapolate_next(diff_vec(&history))
        }
    }
}

pub fn run(input: &str) -> Result<isize, String> {
    let sum = input
        .par_lines()
        .map(|line| {
            let report = Report::new(line);
            extrapolate_next(report.history)
        })
        .sum();
    Ok(sum)
}
