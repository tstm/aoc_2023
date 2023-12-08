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
    let mut characters = Either::Left(s.chars());
    if rev {
        characters = Either::Right(s.chars().rev());
    }
    for c in characters {
        match c.to_digit(10) {
            Some(f) => {
                return f;
            }
            None => (),
        };
    }
    0
}

pub fn run(input: &str) -> Result<u32, std::io::Error> {
    Ok(input
        .par_lines()
        .map(|l| CalibrationLine(l.to_string()).get_value())
        .sum())
}
