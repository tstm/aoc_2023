use rayon::prelude::*;

#[derive(PartialEq, Eq)]
enum Tile {
    Rocks,
    Ash,
}

impl From<char> for Tile {
    fn from(item: char) -> Self {
        match item {
            '#' => Self::Rocks,
            '.' => Self::Ash,
            _ => panic!("We only support two tiles {}", item),
        }
    }
}

fn transpose<T>(original: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!original.is_empty());
    let mut transposed = (0..original[0].len()).map(|_| vec![]).collect::<Vec<_>>();

    for original_row in original {
        for (item, transposed_row) in original_row.into_iter().zip(&mut transposed) {
            transposed_row.push(item);
        }
    }

    transposed
}

fn is_mirroring(map: &[Vec<Tile>], line: &usize) -> bool {
    let mut distance = 0;
    loop {
        distance += 1;
        if line + 1 < distance {
            break true;
        }
        let top = map.get(line + 1 - distance);
        let bottom = map.get(line + distance);

        if let (Some(top), Some(bottom)) = (top, bottom) {
            if top == bottom {
                continue;
            } else {
                break false;
            }
        } else {
            break true;
        }
    }
}

fn find_mirror(map: &[Vec<Tile>]) -> Option<usize> {
    let height = map.len();
    (0..(height - 1)).find(|line| is_mirroring(map, line))
}

pub fn run(input: &str) -> Result<usize, String> {
    let maps: Vec<Vec<Vec<Tile>>> = input
        .split("\n\n")
        .map(|map| {
            map.lines()
                .map(|line| line.chars().map(Tile::from).collect())
                .collect()
        })
        .collect();

    let retval = maps
        .into_par_iter()
        .map(|map| {
            if let Some(line) = find_mirror(&map) {
                (line + 1) * 100
            } else {
                let transposed = transpose(map);
                if let Some(row) = find_mirror(&transposed) {
                    row + 1
                } else {
                    panic!("All maps should have mirrors?")
                }
            }
        })
        .sum();

    Ok(retval)
}
