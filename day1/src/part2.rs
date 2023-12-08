use itertools::Either;
use rayon::prelude::*;

struct CalibrationLine(String);

impl CalibrationLine {
    fn get_value(self) -> u32 {
        let first = get_digit(&self.0, false);
        let last = get_digit(&self.0, true);

        first * 10 + last
    }
}

fn get_digit(s: &String, rev: bool) -> u32 {
    let mut substr = "".to_string();
    let mut characters = Either::Left(s.chars());
    if rev {
        characters = Either::Right(s.chars().rev());
    }
    for c in characters {
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

pub fn run(input: &str) -> Result<u32, std::io::Error> {
    Ok(input
        .par_lines()
        .map(|l| CalibrationLine(l.to_string()).get_value())
        .sum())
}
