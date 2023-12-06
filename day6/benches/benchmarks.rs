fn main() {
    // rayon::ThreadPoolBuilder::new()
    //     .num_threads(6)
    //     .build_global()
    //     .unwrap();
    divan::main();
}

use day6::part1::part1;
use day6::part2::part2;
use divan::Bencher;
use std::time::Duration;

mod part1 {
    use super::*;
    #[divan::bench(min_time = Duration::from_millis(500), max_time = Duration::from_secs(10))]
    fn main(bencher: Bencher) {
        bencher
            .with_inputs(|| include_str!("../input.txt"))
            .bench_values(part1)
    }
}

mod part2 {
    use super::*;
    #[divan::bench(min_time = Duration::from_millis(500), max_time = Duration::from_secs(10))]
    fn main(bencher: Bencher) {
        bencher
            .with_inputs(|| include_str!("../input.txt"))
            .bench_values(part2)
    }
}
