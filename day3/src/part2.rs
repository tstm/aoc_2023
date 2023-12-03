use rayon::prelude::*;
use std::collections::BTreeMap;

#[derive(Debug)]
struct Number {
    value: usize,
    y: isize,
}

impl Number {
    fn new(y: isize, value: usize) -> Number {
        Number { y, value }
    }
}

struct Symbol {
    x: isize,
    y: isize,
}

impl Symbol {
    fn new(x: isize, y: isize, symbol: char) -> Option<Symbol> {
        if symbol == '*' {
            Some(Symbol { x, y })
        } else {
            None
        }
    }

    fn get_gear_ratio(&self, numbers: &BTreeMap<isize, Vec<Number>>) -> Option<usize> {
        let mut found: Vec<usize> = vec![];
        for (_, list) in numbers.range((&self.x - 1)..=(&self.x + 1)) {
            for n in list {
                let yrange =
                    (n.y - 1)..=(n.y + (n.value.checked_ilog10().unwrap_or(0) + 1) as isize);

                if yrange.contains(&self.y) {
                    found.push(n.value);
                }
            }
        }
        if found.len() == 2 {
            Some(found[0] * found[1])
        } else {
            None
        }
    }
}

pub fn part2(input: &str) -> Result<usize, String> {
    let mut numbers: BTreeMap<isize, Vec<Number>> = BTreeMap::new();
    let mut symbols: Vec<Symbol> = vec![];

    for (line_number, line) in input.lines().enumerate() {
        let line_number = line_number as isize;
        let mut numbuf = String::new();
        for (char_number, character) in line.chars().enumerate() {
            let char_number = char_number as isize;
            match character {
                c if c.is_ascii_digit() => {
                    numbuf.push(c);
                }
                c => {
                    if !numbuf.is_empty() {
                        numbers.entry(line_number).or_insert_with(Vec::new);
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
        if !numbuf.is_empty() {
            numbers.entry(line_number).or_insert_with(Vec::new);
            numbers.get_mut(&line_number).unwrap().push(Number::new(
                line.len() as isize - numbuf.len() as isize,
                numbuf.parse::<usize>().expect("Parsing number failed"),
            ));
        }
    }

    Ok(symbols
        .par_iter()
        .flat_map(|n| n.get_gear_ratio(&numbers))
        .sum())
}
