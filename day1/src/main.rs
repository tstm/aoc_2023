use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct CalibrationLine(String);

impl CalibrationLine {
    fn get_value(self) -> u32 {
        let first = get_digit(&self.0, false);
        let last = get_digit(&self.0.chars().rev().collect::<String>(), true);

        first * 10 + last
    }
}

fn get_digit(s: &String, rev: bool) -> u32 {
    let mut substr = "".to_string();
    for c in s.chars() {
        match c.to_digit(10) {
            Some(f) => {
                return f;
            }
            None => {
                if rev {
                    substr.insert(0, c);
                } else {
                    substr.push(c);
                }
                if let Some(num) = match_substring(&substr) {
                    return num;
                }
            }
        };
    }
    0
}

fn match_substring(substring: &str) -> Option<u32> {
    [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .find_map(|(s, n)| substring.contains(s).then_some(*n))
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> Result<(), std::io::Error> {
    if let Ok(lines) = read_lines("input.txt") {
        let mut sum: u32 = 0;
        for line in lines {
            let c = CalibrationLine(line?);
            sum += c.get_value();
        }

        println!("Calibration Sum: {}", sum);
    }

    Ok(())
}
