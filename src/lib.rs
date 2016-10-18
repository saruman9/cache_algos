//! Cache algorithms.
//!
//! TODO Write documentation.

extern crate rand;
#[macro_use] extern crate slog;
extern crate slog_term;

use slog::DrainExt;

pub mod memory;
pub mod fifo;

/// List of cache algorithms.
///
/// See info about each algorithm in structure of algorithm.
#[derive(Debug)]
pub enum Algos {
    Belady,
    FIFO,
    LRU,
    MRU,
    PLRU,
    SLRU,
    LFU,
    RR,
    TwoQ,
    ARC,
    CAR,
    MQ,
}

pub struct CacheAlgo {
    logger: slog::Logger,
    ram: Vec<Vec<i32>>,
    cache_algo: fifo::FifoCache,
}

impl CacheAlgo {
    pub fn init(algo: Algos, size: usize, ram: Vec<Vec<i32>>, logger: Option<slog::Logger>) -> Self {
        let logger = logger.unwrap_or(slog::Logger::root(slog_term::streamer().full().build().fuse(), o!()));
        debug!(logger, "Created CacheAlgo with {:?} cache algorithm.", algo);

        let algo = match algo {
            Algos::Belady => unimplemented!(),
            Algos::FIFO => fifo::FifoCache::new(size, Some(logger.clone())),
            Algos::LRU => unimplemented!(),
            Algos::MRU => unimplemented!(),
            Algos::PLRU => unimplemented!(),
            Algos::SLRU => unimplemented!(),
            Algos::LFU => unimplemented!(),
            Algos::RR => unimplemented!(),
            Algos::TwoQ => unimplemented!(),
            Algos::ARC => unimplemented!(),
            Algos::CAR => unimplemented!(),
            Algos::MQ => unimplemented!(),
        };

        CacheAlgo {
            logger: logger,
            ram: ram,
            cache_algo: algo,
        }
    }

    pub fn with_ram(mut self, ram: Vec<Vec<i32>>) -> Self {
        self.ram = ram;
        self
    }

    pub fn run(&mut self) -> (i32, i32) {
        self.cache_algo.run(&self.ram)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
