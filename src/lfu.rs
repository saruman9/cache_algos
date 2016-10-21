//! Implementation of LFU (Least-Frequently Used) cache algorithm.
//!
//! Counts how often an item is needed. Those that are used least often are discarded first.

use slog;

/// Implementation cache, based on a LFU algorithm.
pub struct LFUCache {
    cache: Vec<(i32, usize)>,
    size: usize,
    logger: slog::Logger,
}

impl LFUCache {
    /// Create new cache with fix size.
    pub fn new(size: usize, logger: Option<slog::Logger>) -> Self {
        let logger = logger.unwrap_or(slog::Logger::root(slog::Discard, o!()));
        debug!(logger, "Created LFU cache with size: {}", size);
        LFUCache {
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
        if let Some(pos) = self.cache.iter().position(|x| &x.0 == val) {
            debug!(self.logger, "hit";
                   "cache" => format!("{:?}", self.cache),
                   "hit" => format!("{}", val));
            self.cache[pos].1 += 1;
            true
        } else {
            if self.cache.len() < self.size {
                self.cache.push((val.clone(), 1));
            } else {
                let elem = self.cache.iter()
                    .position(|x| {
                        x == self.cache.iter()
                            .min_by_key(|y| y.1).unwrap()
                    })
                    .unwrap();
                self.cache.remove(elem);

                self.cache.push((val.clone(), 1));
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
