// use core::ops::Range;
use itertools::Itertools;
use range_set_blaze::prelude::*;

// struct Seed(Range<usize>);

#[derive(Debug)]
struct ConversionMap {
    start: isize,
    end: isize,
    diff: isize,
}

impl ConversionMap {
    pub fn parse(input: &str) -> ConversionMap {
        let params: Vec<isize> = input
            .split_whitespace()
            .map(|n| n.parse::<isize>().expect("Parse conversion number failed"))
            .collect();
        ConversionMap {
            start: params[1],
            end: params[1] + params[2],
            diff: params[0] - params[1],
        }
    }

    fn convert(&self, range: RangeSetBlaze<isize>) -> (RangeSetBlaze<isize>, RangeSetBlaze<isize>) {
        let mut bottom = range;
        let mut mid = bottom.split_off(self.start);
        let top = mid.split_off(self.end);
        let mapped = mid
            .ranges()
            .map(|r| ((r.start() + self.diff)..=(r.end() + self.diff)))
            .collect();
        let remains = top | bottom;
        (remains, mapped)
    }
}

struct ConversionLayer {
    maps: Vec<ConversionMap>,
    _name: String,
}

impl ConversionLayer {
    fn new(input: &str) -> ConversionLayer {
        let mut iter = input.lines().into_iter();
        let name = iter.next().expect("Should be topic string").to_string();
        let maps = iter.map(|c| ConversionMap::parse(c)).collect();

        ConversionLayer { _name: name, maps }
    }

    fn convert(&self, input: RangeSetBlaze<isize>) -> RangeSetBlaze<isize> {
        let mut current = input;
        let mut output = vec![];
        for map in self.maps.iter() {
            let (remains, mapped) = map.convert(current);
            output.push(mapped);
            current = remains;
        }
        output.push(current);
        output.union()
    }
}

fn parse_seeds(input: &str) -> RangeSetBlaze<isize> {
    let mut seeds: Vec<RangeSetBlaze<isize>> = vec![];
    let mut values = input
        .split_once(": ")
        .expect("seeds: should be included in the string")
        .1
        .split_whitespace()
        .map(|n| n.parse::<isize>().expect("Parse number failed"))
        // .collect::<Vec<_>>()
        .tuples();

    while let Some((start, length)) = values.next() {
        seeds.push(RangeSetBlaze::from_iter(start..(start + length)))
    }
    seeds.union()
}

pub fn run(input: &str) -> Result<isize, String> {
    // let seeds = lineiter.next().unwrap();
    let chunks: Vec<_> = input.split_terminator("\n\n").collect();
    let seeds = parse_seeds(chunks[0]);
    let mut layers: Vec<ConversionLayer> = vec![];

    for chunk in chunks[1..8].into_iter() {
        let l = ConversionLayer::new(chunk);
        layers.push(l);
    }

    assert_eq!(layers.len(), 7);

    let soil = layers[0].convert(seeds.clone());
    let fertilizer = layers[1].convert(soil);
    let water = layers[2].convert(fertilizer);
    let light = layers[3].convert(water);
    let temperature = layers[4].convert(light);
    let humidity = layers[5].convert(temperature);
    let location = layers[6].convert(humidity);

    let minimum = location.ranges().map(|range| *range.start()).min().unwrap();

    Ok(minimum)
}
