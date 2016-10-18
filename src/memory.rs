//! Implementation of RAM.
//!
//! See info about `RamBuilder` structure.

use rand;
use rand::distributions::{Range, IndependentSample};
use slog;

/// Structure for creating Vec of batches of Vec<i32>.
///
/// Vec may be generated in direct way:
///
/// `[[0, 1, 2, 3, 4, ...], [0, 1, 2, 3, 4, ...], [0, 1, 2, 3, 4, ...], ... ]`
///
/// Vec may be generated with random of all elements, with random of elements in batch e.g.:
///
/// `[[3, 0, 0, 1, 4], [3, 1, 1, 1, 1], [0, 2, 1, 4, 0], ...]`
pub struct RamBuilder {
    count_batches: usize,
    size_batch: usize,
    random: bool,
    random_local: bool,
    low: i32,
    high: i32,
    logger: slog::Logger,
}

impl RamBuilder {
    /// Create new builder with 10 batches of 10 elements without random by default.
    pub fn new(logger: Option<slog::Logger>) -> Self {
        RamBuilder {
            count_batches: 10,
            size_batch: 10,
            random: false,
            random_local: false,
            low: 0,
            high: 100,
            logger: logger.unwrap_or(slog::Logger::root(slog::Discard, o!())),
        }
    }

    /// Set count of batches.
    pub fn with_count_batches(mut self, c_batches: usize) -> Self {
        self.count_batches = c_batches;
        debug!(self.logger, format!("With {} batches.", self.count_batches));
        self
    }

    /// Set count of elements in batch.
    pub fn with_size_batch(mut self, s_batch: usize) -> Self {
        self.size_batch = s_batch;
        debug!(self.logger,
               format!("With {} elements in batch.", self.size_batch));

        self.low = 0;
        self.high = self.size_batch as i32;
        debug!(self.logger,
               format!("With random elements in batch from {} to {}.",
                       self.low,
                       self.high));
        self
    }

    /// Randomize all elements in all batches.
    pub fn with_all_random(mut self) -> Self {
        if !self.random_local {
            self.random = true;
            debug!(self.logger,
                   format!("With all random elements from {} to {}.",
                           self.low,
                           self.high));
        }
        self
    }

    /// Randomize all elements in each batch.
    ///
    /// Range of random is counts of elements in batch.
    pub fn with_local_random(mut self) -> Self {
        if !self.random {
            self.random_local = true;
            self.low = 0;
            self.high = self.size_batch as i32;
            debug!(self.logger,
                   format!("With random elements in batch from {} to {}.",
                           self.low,
                           self.high));
        }
        self
    }

    /// Set range of random for all elements in all batches.
    pub fn with_range_random(mut self, low: i32, high: i32) -> Self {
        if self.random {
            self.low = low;
            self.high = high;
            debug!(self.logger,
                   format!("With random elements from {} to {}.", self.low, self.high));
        }
        self
    }

    /// Create batches of elements.
    pub fn build(self) -> Vec<Vec<i32>> {
        let mut ram = Vec::with_capacity(self.count_batches);
        if self.random || self.random_local {
            let mut rng = rand::thread_rng();
            let between = Range::new(self.low, self.high);
            for _ in 0..self.count_batches {
                let mut batch = Vec::with_capacity(self.size_batch);
                for _ in 0..self.size_batch {
                    batch.push(between.ind_sample(&mut rng));
                }
                ram.push(batch);
            }
        } else {
            for _ in 0..self.count_batches {
                let mut batch = Vec::with_capacity(self.size_batch);
                for i in 0..self.size_batch {
                    batch.push(i as i32);
                }
                ram.push(batch);
            }
        }
        debug!(self.logger,
               format!("RAM({} batches of {} elements): {:?}",
                       self.count_batches,
                       self.size_batch,
                       ram));
        ram
    }
}
