#![allow(dead_code, unused_variables)]

struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn parse_races(input: &str) -> Vec<Race> {
        let mut iter = input.lines();
        let times: Vec<_> = iter
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let distances: Vec<_> = iter
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        assert_eq!(times.len(), distances.len());

        times
            .iter()
            .enumerate()
            .map(|(n, time)| Race {
                time: *time,
                distance: distances[n],
            })
            .collect()
    }

    fn count_winning(&self) -> usize {
        let t: f64 = self.time as f64;
        let d: f64 = self.distance as f64;
        let min = ((t - (t * t - 4 as f64 * d).sqrt()) / 2 as f64).floor() as usize;
        let max = (((t * t - 4 as f64 * d).sqrt() + t) / 2 as f64).floor() as usize;
        max - min
    }
}

pub fn run(input: &str) -> Result<usize, String> {
    let races = Race::parse_races(input);
    Ok(races.iter().map(|race| race.count_winning()).product())
}
