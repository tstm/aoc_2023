use core::ops::Range;

struct Seed(usize);

#[derive(Debug)]
struct ConversionMap {
    from_range: Range<usize>,
    to_range: usize,
}

impl ConversionMap {
    fn convert(&self, id: usize) -> Option<usize> {
        // if let Some(pos) = self.from_range.rposition(|s| s == id) {
        //     eprintln!(
        //         "Position {}, id {}, range {:?}",
        //         &pos, &id, &self.from_range
        //     );
        //     Some(self.to_range.start + pos)
        // } else {
        //     None
        // }

        self.from_range
            .clone()
            .enumerate()
            .find_map(|(n, from_id)| {
                if from_id == id {
                    Some(self.to_range + n)
                } else {
                    None
                }
            })
    }
}

// struct SoilToFertilizer {
//     seed_range: Range<usize>,
//     fertilizer_range: Range<usize>,
// }
//
// struct FertilizerToWater {
//     seed_range: Range<usize>,
//     fertilizer_range: Range<usize>,
// }
//
// struct WaterToLight {
//     seed_range: Range<usize>,
//     fertilizer_range: Range<usize>,
// }
//
// struct WaterToLight {
//     seed_range: Range<usize>,
//     fertilizer_range: Range<usize>,
// }
//

fn parse_seeds(input: &str) -> Vec<Seed> {
    input
        .split_once(": ")
        .expect("seeds: should be included in the string")
        .1
        .split_whitespace()
        .map(|n| Seed(n.parse::<usize>().expect("Parse number failed")))
        .collect::<Vec<_>>()
}

// Takes in just the numbers
// fn parse_conversion_map(input: &str) -> Vec<ConversionMap> {
//     input
//         .lines()
//         .map(|s| {
//             let params: Vec<usize> = s
//                 .split_whitespace()
//                 .map(|n| n.parse::<usize>().expect("Parse conversion number failed"))
//                 .collect();
//             ConversionMap {
//                 from_range: params[0]..(params[0] + params[2]),
//                 to_range: params[1]..(params[1] + params[2]),
//             }
//         })
//         .collect()
// }

fn parse_conversion_map(input: &str) -> ConversionMap {
    let params: Vec<usize> = input
        .split_whitespace()
        .map(|n| n.parse::<usize>().expect("Parse conversion number failed"))
        .collect();
    ConversionMap {
        to_range: params[0],
        from_range: params[1]..(params[1] + params[2]),
    }
}

pub fn run(input: &str) -> Result<usize, String> {
    // let seeds = lineiter.next().unwrap();
    let chunks: Vec<_> = input.split_terminator("\n\n").collect();
    let seeds = parse_seeds(chunks[0]);

    let mut maps: Vec<Vec<ConversionMap>> = Vec::new();
    for chunk in chunks[1..8].into_iter() {
        let mut iter = chunk.lines().into_iter();
        let _header = iter.next();
        maps.push(iter.map(|c| parse_conversion_map(c)).collect());
    }

    Ok(seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(seed.0, |s, m| {
                m.iter().find_map(|x| x.convert(s)).unwrap_or(s)
            })
        })
        .min()
        .expect("No value found?"))
}
