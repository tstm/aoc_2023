fn main() {
    divan::main();
}

use day2::part1::part1;
use day2::part2::part2;

#[divan::bench]
pub fn run_part1() {
    let input = include_str!("../input.txt");
    let _ = part1(input).unwrap();
}

#[divan::bench]
pub fn run_part2() {
    let input = include_str!("../input.txt");
    let _ = part2(input).unwrap();
}
