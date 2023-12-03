fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(3)
        .build_global()
        .unwrap();
    divan::main();
}

use day3::part1::part1;
use day3::part2::part2;
use divan::Bencher;
use std::time::Duration;

mod part1 {
    use super::*;
    #[divan::bench(min_time = Duration::from_millis(500))]
    fn main(bencher: Bencher) {
        bencher
            .with_inputs(|| include_str!("../input.txt"))
            .bench_values(|input| part1(input))
    }
}

mod part2 {
    use super::*;
    #[divan::bench(min_time = Duration::from_millis(500))]
    fn main(bencher: Bencher) {
        bencher
            .with_inputs(|| include_str!("../input.txt"))
            .bench_values(|input| part2(input))
    }
}
