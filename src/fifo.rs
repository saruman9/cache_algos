//! Implementation of FIFO cache algorithm.
//!
//! This algorithm implement simple First In, First Out method for organizing and manipulating a
//! data buffer, where the oldest (first) entry, or 'head' of the queue, is processed first.

use slog;

use std::collections::VecDeque;

/// Implementation cache, based on a FIFO algorithm.
///
/// Cache data is double-ended queue, which has limit of number of elements.
pub struct FifoCache {
    cache: VecDeque<i32>,
    size: usize,
    logger: slog::Logger,
}

impl FifoCache {
    /// Create new cache with fix size.
    pub fn new(size: usize, logger: Option<slog::Logger>) -> Self {
        let logger = logger.unwrap_or(slog::Logger::root(slog::Discard, o!()));
        debug!(logger, "Created FIFO cache with size: {}", size);
        FifoCache {
            cache: VecDeque::with_capacity(size),
            size: size,
            logger: logger,
        }
    }

    /// Checks the cache to find element. If the cache don't have element, checks the cache size
    /// and removes old element for pushing new element.
    ///
    /// Return `true`, if the cache have element and `false` otherwise.
    pub fn hit(&mut self, val: &i32) -> bool {
        if self.cache.contains(&val) {
            true
        } else {
            if self.cache.len() < self.size {
                self.cache.push_front(val.clone());
            } else {
                self.cache.pop_back();
                self.cache.push_front(val.clone());
            }
            false
        }
    }

    /// Run process of checking algorithm.
    ///
    /// Return tuple with statistic: `(cache hit, cache miss)`.
    pub fn run(&mut self, ram: &Vec<Vec<i32>>) -> (i32, i32) {
        let mut statistic = (0, 0);
        for batch in ram {
            for elem in batch {
                if self.hit(elem) {
                    statistic.0 += 1;
                } else {
                    statistic.1 += 1;
                }
            }
        }
        statistic
    }
}
