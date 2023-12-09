#![allow(dead_code, unused_variables)]

// use std::cell::RefCell;
// use prime_factorization::Factorization;
use core::num::NonZeroU64;
use gcd::euclid_nonzero_u64;
use std::collections::HashMap;

use rayon::prelude::*;

#[derive(Debug)]
struct Next {
    left: String,
    right: String,
}

impl Next {
    const PARENTHESIS: &[char] = &['(', ')'];
    // Takes in "(foo, bar)"
    fn new(input: &str) -> Next {
        let (left, right) = input
            .trim_matches(Next::PARENTHESIS)
            .split_once(", ")
            .unwrap();
        Next {
            left: left.into(),
            right: right.into(),
        }
    }

    fn get_dir(&self, d: &char) -> &String {
        if *d == 'L' {
            &self.left
        } else {
            &self.right
        }
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / euclid_nonzero_u64(NonZeroU64::new(a).unwrap(), NonZeroU64::new(b).unwrap())
}

pub fn run(input: &str) -> Result<usize, String> {
    let mut it = input.lines();
    // let mut nodes = HashMap::new();

    let instructions = it.next().unwrap();
    let nodes = it
        .skip(1)
        .map(|line| {
            let (node_id, rest) = line.split_once(" = ").unwrap();
            let next = Next::new(rest);
            // nodes.insert(node_id, next);
            (node_id.to_string(), next)
        })
        .collect::<HashMap<String, Next>>();

    let steps = 0;
    let nodestate: Vec<&Next> = nodes
        .par_iter()
        .map(|(node, next)| {
            if node.ends_with("A") {
                Some(next)
            } else {
                None
            }
        })
        .flatten()
        .collect();

    let numsteps: Vec<_> = nodestate
        .into_par_iter()
        .map(|mut node| {
            let mut cycles = 0;
            // let mut it = instructions.chars().cycle();
            loop {
                cycles += 1;
                let mut next_node = "";
                for direction in instructions.chars() {
                    next_node = node.get_dir(&direction);
                    node = nodes
                        .get(next_node)
                        .expect("There should always be a node to find");
                }
                if next_node.ends_with("Z") {
                    break cycles;
                }
            }
        })
        .collect();

    let cycles = numsteps.into_iter().fold(1, |acc, item| lcm(acc, item));

    Ok(cycles as usize * instructions.len())
}
