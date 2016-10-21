//! Implementation of RR (Random Replacement) cache algorithm.
//!
//! Randomly selects a candidate item and discards it to make space when necessary. This algorithm
//! does not require keeping any information about the access history.

use slog;
use rand::{self, Rng};
use rand::distributions::{IndependentSample, Range};

/// Implementation cache, based on a RR algorithm.
pub struct RRCache {
    cache: Vec<i32>,
    size: usize,
    logger: slog::Logger,
}

impl RRCache {
    /// Create new cache with fix size.
    pub fn new(size: usize, logger: Option<slog::Logger>) -> Self {
        let logger = logger.unwrap_or(slog::Logger::root(slog::Discard, o!()));
        debug!(logger, "Created RR cache with size: {}", size);
        RRCache {
            cache: Vec::with_capacity(size),
            size: size,
            logger: logger,
        }
    }

    /// Checks the cache to find element. If the cache don't have element, checks the cache size
    /// and removes old element for pushing new element.
    ///
    /// Return `true`, if the cache have element and `false` otherwise.
    pub fn hit<R: Rng>(&mut self, val: &i32, rng: &mut R, range: &Range<usize>) -> bool {
        if self.cache.contains(val) {
            debug!(self.logger, "hit";
                   "cache" => format!("{:?}", self.cache),
                   "hit" => format!("{}", val));
            true
        } else {
            if self.cache.len() < self.size {
                self.cache.push(val.clone());
            } else {
                let random_index = range.ind_sample(rng);
                debug!(self.logger, "random index is {}", random_index);
                self.cache.remove(random_index);

                self.cache.push(val.clone());
            }
            debug!(self.logger, "miss";
                   "cache" => format!("{:?}", self.cache),
                   "hit" => format!("{}", val));
            false
        }
    }

    /// Run process of checking algorithm.
    ///
    /// Return tuple with statistic: `(cache hit, cache miss)`.
    pub fn run(&mut self, ram: &Vec<Vec<i32>>) -> (i32, i32) {
        let mut statistic = (0, 0);
        let mut rng = rand::thread_rng();
        let range = Range::new(0, self.size);
        for batch in ram {
            for elem in batch {
                if self.hit(elem, &mut rng, &range) {
                    statistic.0 += 1;
                } else {
                    statistic.1 += 1;
                }
            }
        }
        statistic
    }
}
