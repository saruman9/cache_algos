//! Main file for execution, testing, benchmarking of Cache Algorithms.
//!
//! TODO Write documentation.

extern crate cache_algos;
#[macro_use] extern crate slog;
extern crate slog_term;

use slog::DrainExt;
use cache_algos::{CacheAlgo, Algos};
use cache_algos::memory::RamBuilder;

fn main() {
    let logger = slog::Logger::root(slog_term::streamer().full().build().fuse(), o!());

    let ram = RamBuilder::new(Some(logger.clone()))
        .with_count_batches(100)
        .with_all_random()
        .with_range_random(0, 1000)
        .build();
    let mut test = CacheAlgo::init(Algos::FIFO, 100, ram, Some(logger));

    println!("{:?}", test.run());
}
