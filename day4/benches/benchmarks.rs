fn main() {
    divan::main();
}

use day4::part1::part1;
use day4::part2::part2;
use divan::Bencher;
use std::time::Duration;

mod part1 {
    use super::*;
    #[divan::bench(min_time = Duration::from_millis(500))]
    fn main(bencher: Bencher) {
        bencher
            .with_inputs(|| include_str!("../input.txt"))
            .bench_values(part1)
    }
}

mod part2 {
    use super::*;
    #[divan::bench(min_time = Duration::from_millis(500))]
    fn main(bencher: Bencher) {
        bencher
            .with_inputs(|| include_str!("../input.txt"))
            .bench_values(part2)
    }
}
