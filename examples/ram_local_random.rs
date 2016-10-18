extern crate cache_algos;
#[macro_use]
extern crate slog;
extern crate slog_term;

use slog::DrainExt;

use cache_algos::memory::RamBuilder;

/// Checking mistakes of reinitialization (see log output for checking).
fn main() {
    let logger = slog::Logger::root(slog_term::streamer().full().build().fuse(), o!());
    let ram = RamBuilder::new(Some(logger))
        .with_local_random()
        .with_size_batch(3)
        .with_count_batches(10)
        .with_range_random(0, 1000)
        .build();
}
