struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn parse_race(input: &str) -> Race {
        let mut iter = input.lines();
        let time = iter
            .next()
            .unwrap()
            .chars()
            .map(|s| if s.is_ascii_digit() { Some(s) } else { None })
            .flatten()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let distance = iter
            .next()
            .unwrap()
            .chars()
            .map(|s| if s.is_ascii_digit() { Some(s) } else { None })
            .flatten()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        Race { time, distance }
    }

    fn count_winning(&self) -> usize {
        let t: f64 = self.time as f64;
        let d: f64 = self.distance as f64;
        let min = ((t - (t * t - 4 as f64 * d).sqrt()) / 2 as f64).floor() as usize;
        let max = (((t * t - 4 as f64 * d).sqrt() + t) / 2 as f64).floor() as usize;
        max - min
    }
}

pub fn part2(input: &str) -> Result<usize, String> {
    let race = Race::parse_race(input);
    Ok(race.count_winning())
}
