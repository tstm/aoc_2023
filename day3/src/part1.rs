#[derive(Debug)]
struct Number {
    value: usize,
    x: isize,
    y: isize,
    next_to_number: bool,
}

impl Number {
    fn new(x: usize, y: usize, value: usize) -> Number {
        Number {
            x: x as isize,
            y: y as isize,
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
    fn new(x: usize, y: usize, symbol: char) -> Option<Symbol> {
        if symbol != '.' && !symbol.is_numeric() {
            Some(Symbol {
                x: x as isize,
                y: y as isize,
            })
        } else {
            None
        }
    }

    fn set_nearby(self, numbers: &mut Vec<Number>) {
        for n in numbers {
            let xrange = (n.x - 1)..=(n.x + 1);
            let yrange = (n.y - 1)..=(n.y + n.value.to_string().len() as isize);

            if xrange.contains(&self.x) && yrange.contains(&self.y) {
                n.next_to_number = true;
            }
        }
    }
}

pub fn part1(input: &str) -> Result<usize, String> {
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    for (line_number, line) in input.lines().enumerate() {
        let mut numbuf = String::new();
        for (char_number, character) in line.chars().enumerate() {
            match character {
                c if c.is_digit(10) => {
                    numbuf.push(c);
                }
                c => {
                    if numbuf.len() != 0 {
                        numbers.push(Number::new(
                            line_number,
                            char_number - numbuf.len(),
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
            numbers.push(Number::new(
                line_number,
                line.len() - numbuf.len(),
                numbuf.parse::<usize>().expect("Parsing number failed"),
            ));
        }
    }

    for s in symbols {
        s.set_nearby(&mut numbers);
    }

    Ok(numbers
        .iter()
        .filter(|n| n.next_to_number)
        .map(|n| n.value)
        .sum())
}
