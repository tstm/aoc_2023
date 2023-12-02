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

    fn find_max(&self) -> (u32, u32, u32) {
        (self.max_r(), self.max_g(), self.max_b())
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

pub fn part2(input: &str) -> Result<u32, String> {
    let mut sum = 0;
    for line in input.lines() {
        let game = Game::new(line);
        let (red, green, blue) = game.find_max();
        sum += red * green * blue
    }
    Ok(sum)
}

pub fn main() {
    let input = include_str!("../../input.txt");
    let result = part2(input).unwrap();
    println!("Output: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = include_str!("../../example.txt");
        let result = part2(input).unwrap();
        assert_eq!(result, 2286u32);
    }
}
