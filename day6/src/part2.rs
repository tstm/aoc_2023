struct Race {
    time: f64,
    distance: f64,
}

impl Race {
    fn parse_race(input: &str) -> Race {
        let mut iter = input.lines().map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
        });

        let time = iter.next().unwrap();
        let distance = iter.next().unwrap();

        Race {
            time: time.parse::<f64>().unwrap(),
            distance: distance.parse::<f64>().unwrap(),
        }
    }

    fn count_winning(&self) -> usize {
        let t: f64 = self.time;
        let d: f64 = self.distance;
        let min = (t - (t * t - 4f64 * d).sqrt()) / 2f64;
        let max = (t + (t * t - 4f64 * d).sqrt()) / 2f64;

        let min = min.floor() as usize;
        let max = (max - 1f64).ceil() as usize;
        max - min
    }
}

pub fn run(input: &str) -> Result<usize, String> {
    let race = Race::parse_race(input);
    Ok(race.count_winning())
}
