//! Implementation of MRU cache algorithm.
//!
//! Discards, in contrast to LRU, the most recently used items first. MRU algorithms are most useful
//! in situations where the older an item is, the more likely it is to be accessed.

use slog;

/// Implementation cache, based on a LRU algorithm.
pub struct MRUCache {
    cache: Vec<i32>,
    size: usize,
    logger: slog::Logger,
}

impl MRUCache {
    /// Create new cache with fix size.
    pub fn new(size: usize, logger: Option<slog::Logger>) -> Self {
        let logger = logger.unwrap_or(slog::Logger::root(slog::Discard, o!()));
        debug!(logger, "Created MRU cache with size: {}", size);
        MRUCache {
            cache: Vec::with_capacity(size),
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
            let elem = self.cache.remove(position);
            self.cache.push(elem);
            true
        } else {
            if self.cache.len() < self.size {
                self.cache.push(val.clone());
            } else {
                self.cache.pop();
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
