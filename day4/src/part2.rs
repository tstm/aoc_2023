use std::collections::{BTreeMap, BTreeSet};

pub fn part2(input: &str) -> Result<usize, String> {
    let mut copies = BTreeMap::<usize, usize>::new();
    Ok(input
        .lines()
        .enumerate()
        .map(|(n, line)| {
            let count = get_winnings(&line);
            let num_copies = *copies.get(&n).unwrap_or(&0);
            for lineid in (n + 1)..=(n + count) {
                copies
                    .entry(lineid)
                    .and_modify(|e| *e += 1 + num_copies)
                    .or_insert_with(|| 1 + num_copies);
            }
            1 + num_copies
        })
        .sum())
}

fn get_winnings(line: &str) -> usize {
    let (winning_row, had_row) = line
        .split_once(": ")
        .expect("There should be :")
        .1
        .split_once(" | ")
        .expect("Row split failed");

    let winning_numbers = winning_row
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<isize>().expect("Parse number failed"))
        .collect::<BTreeSet<isize>>();

    let had_numbers = had_row
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<isize>().expect("Parse number failed"))
        .collect::<BTreeSet<isize>>();

    winning_numbers.intersection(&had_numbers).count()
}
