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

    fn new_from_vec(input: Vec<isize>) -> Report {
        Report { history: input }
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
        // eprintln!("Diffvec: {:?}", &result);
        result
    }

    fn extrapolate_first(history: Vec<isize>) -> isize {
        match history.iter().all(|n| *n == 0) {
            true => 0,
            false => {
                history.first().expect("There should be something to keep")
                    - Self::extrapolate_first(Self::diff_vec(&history))
            }
        }
    }
}

pub fn run(input: &str) -> Result<isize, String> {
    let sum = input
        .par_lines()
        .map(|line| {
            let report = Report::new(line);
            Report::extrapolate_first(report.history)
        })
        .sum();
    Ok(sum)
}
