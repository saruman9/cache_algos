//! Implementation of LRU cache algorithm.
//!
//! Discards the least recently used items first. This algorithm requires keeping track of what was
//! used when, which is expensive if one wants to make sure the algorithm always discards the least
//! recently used item. General implementations of this technique require keeping "age bits" for
//! cache-lines and track the "Least Recently Used" cache-line based on age-bits. In such an
//! implementation, every time a cache-line is used, the age of all other cache-lines changes.

use slog;

use std::collections::VecDeque;

/// Implementation cache, based on a LRU algorithm.
pub struct LRUCache {
    cache: VecDeque<i32>,
    size: usize,
    logger: slog::Logger,
}

impl LRUCache {
    /// Create new cache with fix size.
    pub fn new(size: usize, logger: Option<slog::Logger>) -> Self {
        let logger = logger.unwrap_or(slog::Logger::root(slog::Discard, o!()));
        debug!(logger, "Created LRU cache with size: {}", size);
        LRUCache {
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
        if let Some(position) = self.cache.iter().position(|x| x == val) {
            debug!(self.logger, "hit";
                   "cache" => format!("{:?}", self.cache),
                   "hit" => format!("{}", val));
            let elem = self.cache.remove(position).unwrap();
            self.cache.push_front(elem);
            true
        } else {
            if self.cache.len() < self.size {
                self.cache.push_front(val.clone());
            } else {
                self.cache.pop_back();
                self.cache.push_front(val.clone());
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
