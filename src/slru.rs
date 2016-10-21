//! Implementation of SLRU (Segmented LRU) cache algorithm.
//!
//! An SLRU cache is divided into two segments, a probationary segment and a protected segment.
//! Lines in each segment are ordered from the most to the least recently accessed. Data from misses
//! is added to the cache at the most recently accessed end of the probationary segment. Hits are
//! removed from wherever they currently reside and added to the most recently accessed end of the
//! protected segment. Lines in the protected segment have thus been accessed at least twice. The
//! protected segment is finite, so migration of a line from the probationary segment to the
//! protected segment may force the migration of the LRU line in the protected segment to the most
//! recently used (MRU) end of the probationary segment, giving this line another chance to be
//! accessed before being replaced. The size limit on the protected segment is an SLRU parameter
//! that varies according to the I/O workload patterns. Whenever data must be discarded from the
//! cache, lines are obtained from the LRU end of the probationary segment.

use slog;

use std::collections::VecDeque;

/// Implementation cache, based on a LRU algorithm.
pub struct SLRUCache {
    prob_cache: VecDeque<i32>,
    prot_cache: VecDeque<i32>,
    size_prob_cache: usize,
    size_prob_cache_min: usize,
    size_prot_cache: usize,
    logger: slog::Logger,
}

impl SLRUCache {
    /// Create new cache with fix size.
    pub fn new(size: usize, logger: Option<slog::Logger>) -> Self {
        let logger = logger.unwrap_or(slog::Logger::root(slog::Discard, o!()));
        debug!(logger, "Created SLRU cache with size: {}", size);
        SLRUCache {
            prob_cache: VecDeque::with_capacity(size),
            prot_cache: VecDeque::with_capacity(size / 2),
            size_prob_cache: size,
            size_prob_cache_min: size - size / 2,
            size_prot_cache: size / 2,
            logger: logger,
        }
    }

    /// Checks the cache to find element. If the cache don't have element, checks the cache size
    /// and removes old element for pushing new element.
    ///
    /// Return `true`, if the cache have element and `false` otherwise.
    pub fn hit(&mut self, val: &i32) -> bool {
        if let Some(position) = self.prob_cache.iter().position(|x| x == val) {
            debug!(self.logger, "hit";
                   "prob cache" => format!("{:?}", self.prob_cache),
                   "prot cache" => format!("{:?}", self.prot_cache),
                   "hit" => format!("{}", val));
            let elem = self.prob_cache.remove(position).unwrap();
            self.add_to_protected_cache(elem);
            true
        } else {
            if let Some(position) = self.prot_cache.iter().position(|x| x == val) {
                debug!(self.logger, "hit";
                       "prob cache" => format!("{:?}", self.prob_cache),
                       "prot cache" => format!("{:?}", self.prot_cache),
                       "hit" => format!("{}", val));
                let elem = self.prot_cache.remove(position).unwrap();
                self.prot_cache.push_front(elem);
                true
            } else {
                if self.prob_cache.len() < self.size_prob_cache {
                    self.prob_cache.push_front(val.clone());
                } else {
                    self.prob_cache.pop_back();
                    self.prob_cache.push_front(val.clone());
                }
                debug!(self.logger, "miss";
                       "prob cache" => format!("{:?}", self.prob_cache),
                       "prot cache" => format!("{:?}", self.prot_cache),
                       "hit" => format!("{}", val));
                false
            }
        }
    }

    // Auxiliary function for adding to protected cache in right way.
    fn add_to_protected_cache(&mut self, val: i32) {
        if self.prot_cache.len() < self.size_prot_cache {
            self.prot_cache.push_front(val);
            if self.size_prob_cache_min != self.size_prob_cache {
                self.size_prob_cache -= 1;
            }
        } else {
            let to_probationary = self.prot_cache.pop_back().unwrap();
            self.prot_cache.push_front(val);
            if self.size_prob_cache_min != self.size_prob_cache {
                self.size_prob_cache -= 1;
            }
            if self.prob_cache.len() < self.size_prot_cache {
                self.prob_cache.push_front(to_probationary);
            } else {
                self.prob_cache.pop_back();
                self.prob_cache.push_front(to_probationary);
            }
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
