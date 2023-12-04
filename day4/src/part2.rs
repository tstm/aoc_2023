pub fn part2(input: &str) -> Result<usize, String> {
    // let mut copies = BTreeMap::<usize, usize>::new();
    let line_count = input.lines().count();
    let mut copies: Vec<usize> = vec![0; line_count];
    Ok(input
        .lines()
        .enumerate()
        .map(|(n, line)| {
            let count = get_winnings(&line);
            let num_copies = copies[n];
            for lineid in (n + 1)..=(n + count) {
                copies[lineid] += 1 + num_copies;
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
        .map(|n| n.parse::<usize>().expect("Parse number failed"))
        .collect::<Vec<usize>>();

    had_row
        .trim()
        .split_whitespace()
        .filter(|n| winning_numbers.contains(&n.parse::<usize>().expect("Parse number failed")))
        .count()
}
