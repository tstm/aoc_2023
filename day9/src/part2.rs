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
    let mut it = vector.iter().peekable();
    let mut result: Vec<_> = vec![];
    while let Some(value) = it.next() {
        match it.peek() {
            Some(p) => result.push(*p - value),
            None => break,
        }
    }
    result
}

fn extrapolate_first(history: Vec<isize>) -> isize {
    match history.iter().all(|n| *n == 0) {
        true => 0,
        false => {
            history.first().expect("There should be something to keep")
                - extrapolate_first(diff_vec(&history))
        }
    }
}

pub fn run(input: &str) -> Result<isize, String> {
    let sum = input
        .par_lines()
        .map(|line| {
            let report = Report::new(line);
            extrapolate_first(report.history)
        })
        .sum();
    Ok(sum)
}
