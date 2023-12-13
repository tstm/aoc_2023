fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build_global()
        .unwrap();
    divan::main();
}

use day13::part1::run as run1;
use day13::part2::run as run2;
use divan::Bencher;
use std::time::Duration;

mod part1 {
    use super::*;
    #[divan::bench(min_time = Duration::from_millis(500), max_time = Duration::from_secs(10))]
    fn main(bencher: Bencher) {
        bencher
            .with_inputs(|| include_str!("../input.txt"))
            .bench_values(run1)
    }
}

mod part2 {
    use super::*;
    #[divan::bench(min_time = Duration::from_millis(500), max_time = Duration::from_secs(10))]
    fn main(bencher: Bencher) {
        bencher
            .with_inputs(|| include_str!("../input.txt"))
            .bench_values(run2)
    }
}
