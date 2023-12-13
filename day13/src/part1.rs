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

impl From<u8> for Tile {
    fn from(item: u8) -> Self {
        match item {
            b'#' => Self::Rocks,
            b'.' => Self::Ash,
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
                .map(|line| line.bytes().map(Tile::from).collect())
                .collect()
        })
        .collect();

    let retval = maps
        .into_par_iter()
        .map(|map| match find_mirror(&map) {
            Some(line) => (line + 1) * 100,
            None => match find_mirror(&transpose(map)) {
                Some(row) => row + 1,
                None => panic!("All maps should have mirrors?"),
            },
        })
        .sum();

    Ok(retval)
}
