//! Main file for execution, testing, benchmarking of Cache Algorithms.
//!
//! TODO Write documentation.

extern crate cache_algos;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_envlogger;

use slog::DrainExt;
use cache_algos::fifo::FifoCache;
use cache_algos::memory::RamBuilder;
use cache_algos::belady::BeladyCache;
use cache_algos::lru::LRUCache;
use cache_algos::lfu::LFUCache;
use cache_algos::rr::RRCache;

fn main() {
    let term_log = slog_term::streamer().build();
    let drain_log = slog_envlogger::EnvLogger::new(term_log);
    let logger = slog::Logger::root(drain_log.fuse(), o!());

    let ram = RamBuilder::new(Some(logger.clone()))
        .with_count_batches(100)
        .with_all_random()
        .with_range_random(0, 1000)
        .build();

    let cache_size = 5;

    let mut fifo_cache = FifoCache::new(cache_size, Some(logger.clone()));
    let mut belady_cache = BeladyCache::new(cache_size, Some(logger.clone()));
    let mut lru_cache = LRUCache::new(cache_size, Some(logger.clone()));
    let mut lfu_cache = LFUCache::new(cache_size, Some(logger.clone()));
    let mut rr_cache = RRCache::new(cache_size, Some(logger.clone()));

    println!("RR: {:?}", rr_cache.run(&ram));
    println!("FIFO: {:?}", fifo_cache.run(&ram));
    println!("LRU: {:?}", lru_cache.run(&ram));
    println!("LFU: {:?}", lfu_cache.run(&ram));
    println!("Bélády: {:?}", belady_cache.run(&ram));
}
