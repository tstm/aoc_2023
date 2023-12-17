#![allow(dead_code, unused_variables)]

// use rayon::prelude::*;

fn calculate_hash(input: char, acc: usize) -> usize {
    ((input as usize + acc) * 17) % 256
}

fn hash(input: &str) -> usize {
    input.chars().fold(0, |acc, c| calculate_hash(c, acc))
}

pub fn run(input: &str) -> Result<usize, String> {
    let sum = input.trim().split(",").map(|s| hash(s)).sum();

    // .inspect(|num| println!("{}", num))
    Ok(sum)
}
