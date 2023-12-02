struct Game {
    rounds: Vec<Round>,
}

impl Game {
    fn new(input: &str) -> Game {
        let (_, rounds_string) = input.split_once(": ").unwrap();
        let mut rounds: Vec<Round> = vec![];
        for rs in rounds_string.split("; ") {
            rounds.push(Round::new(rs));
        }

        Game { rounds }
    }

    fn max_r(&self) -> u32 {
        self.rounds.iter().map(|r| r.red).max().unwrap_or(0)
    }

    fn max_g(&self) -> u32 {
        self.rounds.iter().map(|r| r.green).max().unwrap_or(0)
    }

    fn max_b(&self) -> u32 {
        self.rounds.iter().map(|r| r.blue).max().unwrap_or(0)
    }

    fn find_power(&self) -> u32 {
        self.max_r() * self.max_g() * self.max_b()
    }
}

struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn new(input: &str) -> Round {
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for set in input.split(", ") {
            let (num, color) = set.split_once(' ').unwrap();
            let num = num.parse::<u32>().unwrap();
            match color {
                "red" => {
                    red += num;
                }
                "green" => {
                    green += num;
                }
                "blue" => {
                    blue += num;
                }
                _ => continue,
            }
        }
        Round { red, green, blue }
    }
}

use rayon::prelude::*;

pub fn part2(input: &str) -> Result<u32, String> {
    Ok(input
        .par_lines()
        .map(|l| Game::new(l).find_power())
        .sum::<u32>())
}
