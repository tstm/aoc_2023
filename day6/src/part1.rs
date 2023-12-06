#![allow(dead_code, unused_variables)]

struct Race {
    time: f64,
    distance: f64,
}

impl Race {
    fn parse_races(input: &str) -> Vec<Race> {
        let mut iter = input.lines().map(|line| line.split_whitespace().skip(1));

        std::iter::zip(
            iter.next().expect("to get a time"),
            iter.next().expect("to get a distance"),
        )
        .map(|(t, d)| Race {
            time: t.parse::<f64>().unwrap(),
            distance: d.parse::<f64>().unwrap(),
        })
        .collect()
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
    let races = Race::parse_races(input);
    Ok(races.iter().map(|race| race.count_winning()).product())
}
