use std::collections::BTreeMap;

#[derive(Debug)]
struct Number {
    value: usize,
    x: isize,
    y: isize,
}

impl Number {
    fn new(x: isize, y: isize, value: usize) -> Number {
        Number { x, y, value }
    }
}

struct Symbol {
    x: isize,
    y: isize,
    symbol: char,
}

impl Symbol {
    fn new(x: isize, y: isize, symbol: char) -> Option<Symbol> {
        if symbol != '.' && !symbol.is_numeric() {
            Some(Symbol { x, y, symbol })
        } else {
            None
        }
    }

    fn get_gear_ratio(&self, numbers: &BTreeMap<isize, Vec<Number>>) -> Option<usize> {
        if self.symbol != '*' {
            return None;
        }
        let mut found: Vec<usize> = vec![];
        for (_, list) in numbers.range((&self.x - 1)..=(&self.x + 1)) {
            for n in list {
                let yrange = (n.y - 1)..=(n.y + n.value.to_string().len() as isize);
                // eprintln!("xrange: {:?}, yrange {:?}", xrange, yrange);
                // eprintln!("Testing gear value {}", n.value);

                if yrange.contains(&self.y) {
                    // eprintln!("Found gear value {}", n.value);
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
                c if c.is_digit(10) => {
                    numbuf.push(c);
                }
                c => {
                    if numbuf.len() != 0 {
                        if !numbers.contains_key(&line_number) {
                            numbers.insert(line_number as isize, vec![]);
                        }
                        numbers.get_mut(&line_number).unwrap().push(Number::new(
                            line_number,
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
                line_number,
                line.len() as isize - numbuf.len() as isize,
                numbuf.parse::<usize>().expect("Parsing number failed"),
            ));
        }
    }

    Ok(symbols
        .iter()
        .map(|n| n.get_gear_ratio(&numbers))
        .flatten()
        .sum())
}
