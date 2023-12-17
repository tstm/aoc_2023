#![allow(dead_code, unused_variables)]

// use rayon::prelude::*;

fn calculate_hash(input: char, acc: usize) -> usize {
    ((input as usize + acc) * 17) % 256
}

fn hash(input: &str) -> usize {
    input.chars().fold(0, |acc, c| calculate_hash(c, acc))
}

struct Lens {
    label: String,
    value: usize,
}

impl Lens {
    fn new(label: String, contents: usize) -> Self {
        Self {
            label,
            value: contents,
        }
    }
}

pub fn run(input: &str) -> Result<usize, String> {
    const SIZE: usize = 256;
    let mut boxes: [Vec<Lens>; SIZE] = std::array::from_fn(|_| vec![]);

    input
        .trim()
        .split(",")
        .for_each(|s| match s.ends_with("-") {
            true => {
                let label = s.strip_suffix("-").unwrap();
                let hash = hash(label);
                if let Some(pos) = boxes[hash].iter().position(|b| b.label == label) {
                    boxes[hash].remove(pos);
                }
            }
            false => {
                let (label, value) = s.split_once("=").unwrap();
                let value = value.parse::<usize>().unwrap();
                let hash = hash(label);
                if let Some(pos) = boxes[hash].iter().position(|b| b.label == label) {
                    boxes[hash][pos].value = value;
                } else {
                    boxes[hash].push(Lens::new(label.into(), value));
                }
            }
        });

    let sum = boxes
        .into_iter()
        .enumerate()
        .map(|(bn, b)| {
            b.iter()
                .enumerate()
                .map(|(n, lens)| (bn + 1) * (n + 1) * lens.value)
                .sum::<usize>()
        })
        .sum();

    // .inspect(|num| println!("{}", num))
    Ok(sum)
}
