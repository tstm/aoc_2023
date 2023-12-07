use derive_more::Deref;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::ops::Range;

use itertools::Itertools;
use rayon::prelude::*;

#[derive(Deref, Eq, PartialEq, Debug, Clone)]
struct SortRange(Range<isize>);

impl Ord for SortRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for SortRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct ConversionMap {
    start: isize,
    end: isize,
    diff: isize,
}

#[derive(Debug)]
struct RangeSet {
    ranges: BTreeSet<SortRange>,
}

impl RangeSet {
    fn new(ranges: BTreeSet<SortRange>) -> RangeSet {
        RangeSet { ranges }
    }

    fn merge(&mut self, other: RangeSet) {
        self.ranges.append(&mut other.ranges.clone())
    }

    fn split_off(&mut self, split: isize) -> Self {
        RangeSet {
            ranges: match &self.ranges.clone().iter().find(|r| r.contains(&split)) {
                Some(cutoff) => {
                    let lrange = SortRange(cutoff.start..split);
                    let rrange = SortRange(split..cutoff.end);
                    self.ranges.remove(&cutoff);
                    let mut r = self.ranges.split_off(&cutoff);
                    if lrange.len() > 0 {
                        self.ranges.insert(lrange);
                    }
                    if rrange.len() > 0 {
                        r.insert(rrange);
                    }
                    r
                }
                None => self.ranges.split_off(&SortRange(split..split)),
            },
        }
    }
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

    fn convert(&self, range: RangeSet) -> (RangeSet, RangeSet) {
        let mut bottom = range;
        let mut mid = bottom.split_off(self.start);
        let top = mid.split_off(self.end);
        let mapped = RangeSet {
            ranges: mid
                .ranges
                .into_iter()
                .map(|r| SortRange((r.start + self.diff)..(r.end + self.diff)))
                .collect::<BTreeSet<SortRange>>(),
        };
        bottom.merge(top);
        (bottom, mapped)
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

    fn convert(&self, input: RangeSet) -> RangeSet {
        let mut current = input;
        let mut output = RangeSet::new(BTreeSet::new());
        for map in self.maps.iter() {
            let (remains, mapped) = map.convert(current);
            output.merge(mapped);
            current = remains;
        }
        // output.iter().map(|&o| current.merge(o));
        current.merge(output);
        current
        // output
    }
}

fn parse_seeds(input: &str) -> RangeSet {
    // let mut seeds: Vec<Range<isize>> = vec![];
    let mut seeds = RangeSet::new(BTreeSet::new());
    let mut values = input
        .split_once(": ")
        .expect("seeds: should be included in the string")
        .1
        .split_whitespace()
        .map(|n| n.parse::<isize>().expect("Parse number failed"))
        // .collect::<Vec<_>>()
        .tuples();

    while let Some((start, length)) = values.next() {
        seeds.ranges.insert(SortRange(start..(start + length)));
    }
    seeds
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

    let rangesets: Vec<_> = seeds
        .ranges
        .into_par_iter()
        .map(|seed| {
            let mut rs = RangeSet::new(BTreeSet::new());
            rs.ranges.insert(seed);
            let soil = layers[0].convert(rs);
            let fertilizer = layers[1].convert(soil);
            let water = layers[2].convert(fertilizer);
            let light = layers[3].convert(water);
            let temperature = layers[4].convert(light);
            let humidity = layers[5].convert(temperature);
            layers[6].convert(humidity)
        })
        .collect();

    let minimum = rangesets
        .into_iter()
        .map(|rs| rs.ranges.first().unwrap().start)
        .min()
        .unwrap();

    Ok(minimum)
}
