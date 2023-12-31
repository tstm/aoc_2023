use rayon::prelude::*;

pub fn part2(input: &str) -> Result<usize, String> {
    let line_count = input.lines().count();
    let mut copies: Vec<usize> = vec![0; line_count];
    let counts: Vec<usize> = input.par_lines().map(|line| get_winnings(&line)).collect();

    Ok(counts
        .iter()
        .enumerate()
        .map(|(n, count)| {
            for lineid in (n + 1)..=(n + count) {
                copies[lineid] += 1 + copies[n];
            }
            1 + copies[n]
        })
        .sum())
}

fn get_winnings(line: &str) -> usize {
    let (winning_row, my_row) = line
        .split_once(": ")
        .expect("There should be :")
        .1
        .split_once(" | ")
        .expect("Row split failed");

    let winning_numbers = winning_row
        .split_whitespace()
        .map(|n| n.parse::<usize>().expect("Parse number failed"))
        .collect::<Vec<usize>>();

    my_row
        .split_whitespace()
        .filter(|n| winning_numbers.contains(&n.parse::<usize>().expect("Parse number failed")))
        .count()
}
