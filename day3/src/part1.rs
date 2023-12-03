use rayon::prelude::*;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Number {
    value: usize,
    y: isize,
    next_to_number: bool,
}

impl Number {
    fn new(y: isize, value: usize) -> Number {
        Number {
            y,
            value,
            next_to_number: false,
        }
    }
}

struct Symbol {
    x: isize,
    y: isize,
}

impl Symbol {
    fn new(x: isize, y: isize, symbol: char) -> Option<Symbol> {
        if symbol != '.' && !symbol.is_numeric() {
            Some(Symbol { x, y })
        } else {
            None
        }
    }

    fn _set_nearby(self, numbers: &mut BTreeMap<isize, Vec<Number>>) {
        for (_, lines) in numbers.range_mut((&self.x - 1)..=(&self.x + 1)) {
            for n in lines {
                let yrange = (n.y - 1)..=(n.y + n.value.to_string().len() as isize);

                if yrange.contains(&self.y) {
                    n.next_to_number = true;
                }
            }
        }
    }

    fn get_nearby<'a>(&'a self, numbers: &'a BTreeMap<isize, Vec<Number>>) -> BTreeSet<&Number> {
        let mut nearby = BTreeSet::new();
        for (_, lines) in numbers.range((&self.x - 1)..=(&self.x + 1)) {
            for n in lines {
                let yrange =
                    (n.y - 1)..=(n.y + (n.value.checked_ilog10().unwrap_or(0) + 1) as isize);

                if yrange.contains(&self.y) {
                    nearby.insert(n);
                }
            }
        }
        nearby
    }
}

pub fn part1(input: &str) -> Result<usize, String> {
    let mut numbers: BTreeMap<isize, Vec<Number>> = BTreeMap::new();
    let mut symbols: Vec<Symbol> = vec![];

    for (line_number, line) in input.lines().enumerate() {
        let line_number = line_number as isize;
        let mut numbuf = String::new();
        for (char_number, character) in line.chars().enumerate() {
            let char_number = char_number as isize;
            match character {
                c if c.is_digit(10) => {
                    numbuf.push(c);
                }
                c => {
                    if numbuf.len() != 0 {
                        if !numbers.contains_key(&line_number) {
                            numbers.insert(line_number as isize, vec![]);
                        }
                        numbers.get_mut(&line_number).unwrap().push(Number::new(
                            char_number - numbuf.len() as isize,
                            numbuf.parse::<usize>().expect("Parsing number failed"),
                        ));
                        numbuf.clear();
                    }
                    match Symbol::new(line_number, char_number, c) {
                        Some(s) => symbols.push(s),
                        None => continue,
                    }
                }
            }
        }
        if numbuf.len() != 0 {
            if !numbers.contains_key(&line_number) {
                numbers.insert(line_number as isize, vec![]);
            }
            numbers.get_mut(&line_number).unwrap().push(Number::new(
                line.len() as isize - numbuf.len() as isize,
                numbuf.parse::<usize>().expect("Parsing number failed"),
            ));
        }
    }

    Ok(symbols
        .par_iter()
        .flat_map(|s| s.get_nearby(&numbers))
        .map(|n| n.value)
        .sum())
}
