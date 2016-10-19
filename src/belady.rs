//! Implementation of Bélády's (OPT) cache algorithm.
//!
//! The most efficient caching algorithm would be to always discard the information that will not be
//! needed for the longest time in the future. This optimal result is referred to as Bélády's
//! optimal algorithm or the clairvoyant algorithm. Since it is generally impossible to predict how
//! far in the future information will be needed, this is generally not implementable in practice.
//! The practical minimum can be calculated only after experimentation, and one can compare the
//! effectiveness of the actually chosen cache algorithm.

use slog;

use std::collections::VecDeque;

/// Implementation cache, based on a Bélády's algorithm.
pub struct BeladyCache {
    cache: Vec<i32>,
    size: usize,
    logger: slog::Logger,
}

impl BeladyCache {
    /// Create new cache with fix size.
    pub fn new(size: usize, logger: Option<slog::Logger>) -> Self {
        let logger = logger.unwrap_or(slog::Logger::root(slog::Discard, o!()));
        debug!(logger, "Created Belady cache with size: {}", size);
        BeladyCache {
            cache: Vec::with_capacity(size),
            size: size,
            logger: logger,
        }
    }

    /// Checks the cache to find element. If the cache don't have element, checks the cache size
    /// and removes old element for pushing new element.
    ///
    /// Return `true`, if the cache have element and `false` otherwise.
    pub fn hit(&mut self, val: &i32, future: &VecDeque<i32>) -> bool {
        if self.cache.contains(&val) {
            debug!(self.logger, "hit";
                   "cache" => format!("{:?}", self.cache),
                   "hit" => format!("{}", val));
            true
        } else {
            if self.cache.len() < self.size {
                self.cache.push(val.clone());
            } else {
                self.look_into_the_future(future);

                self.cache.push(val.clone());
            }
            debug!(self.logger, "miss";
                   "cache" => format!("{:?}", self.cache),
                   "hit" => format!("{}", val));
            false
        }
    }

    // Remove the element that will not be needed for the longest time in the future.
    fn look_into_the_future(&mut self, future: &VecDeque<i32>) {
        let mut usage: Vec<Option<usize>> = Vec::with_capacity(self.cache.len());
        for elem in self.cache.iter() {
            let position_in_future = future.iter().position(|x| x == elem);
            usage.push(position_in_future);
        }
        debug!(self.logger, "future usage of cache"; "usage" => format!("{:?}", usage));
        let elem_for_remove = usage.iter().position(|x| x.is_none());
        if let Some(index) = elem_for_remove {
            self.cache.remove(index);
        } else {
            let index = usage.iter().position(|x| x == usage.iter().max().unwrap());
            self.cache.remove(index.unwrap());
        }
    }

    /// Run process of checking algorithm.
    ///
    /// Return tuple with statistic: `(cache hit, cache miss)`.
    pub fn run(&mut self, ram: &Vec<Vec<i32>>) -> (i32, i32) {
        let mut statistic = (0, 0);
        let mut future: VecDeque<i32> = ram.iter()
            .flat_map(|batch| {
                batch.iter()
                    .map(|elem| elem.clone())
            })
            .collect();
        for batch in ram {
            for elem in batch {
                future.pop_front();
                if self.hit(elem, &future) {
                    statistic.0 += 1;
                } else {
                    statistic.1 += 1;
                }
            }
        }
        statistic
    }
}
