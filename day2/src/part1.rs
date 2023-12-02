struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn new(input: &str) -> Game {
        let (game, rounds_string) = input.split_once(": ").unwrap();
        let id: u32 = game.split_once(" ").unwrap().1.parse::<u32>().unwrap();
        let mut rounds: Vec<Round> = vec![];
        for rs in rounds_string.split("; ") {
            rounds.push(Round::new(rs));
        }

        Game { id, rounds }
    }

    fn is_possible(&self) -> u32 {
        match self.rounds.iter().find(|r| !r.is_possible()) {
            Some(_) => 0,
            None => self.id,
        }
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

    fn is_possible(&self) -> bool {
        if self.red <= 12 && self.green <= 13 && self.blue <= 14 {
            true
        } else {
            false
        }
    }
}

use rayon::prelude::*;

pub fn part1(input: &str) -> Result<u32, String> {
    // let mut sum = 0;
    // for line in input.lines() {
    //     let game = Game::new(line);
    //     if game.is_possible() {
    //         sum += game.id;
    //     }
    // }
    Ok(input
        .par_lines()
        .map(|l| Game::new(l).is_possible())
        .sum::<u32>())
}
