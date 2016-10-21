//! Main file for execution, testing, benchmarking of Cache Algorithms.
//!
//! TODO Write documentation.

extern crate cache_algos;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_envlogger;

use slog::DrainExt;

use std::env;

use cache_algos::memory::RamBuilder;
use cache_algos::fifo::FifoCache;
use cache_algos::belady::BeladyCache;
use cache_algos::lru::LRUCache;
use cache_algos::lfu::LFUCache;
use cache_algos::rr::RRCache;
use cache_algos::mru::MRUCache;
use cache_algos::slru::SLRUCache;

struct Options {
    file: bool,
    builder: bool,
    path_file: Option<String>,
    algo: Option<String>,
    count_of_batches: Option<usize>,
    size_of_batch: Option<usize>,
    all_random: bool,
    local_random: bool,
    low_range: Option<i32>,
    high_range: Option<i32>,
    cache_size: Option<usize>,
}

fn main() {
    let term_log = slog_term::streamer().build();
    let drain_log = slog_envlogger::EnvLogger::new(term_log);
    let logger = slog::Logger::root(drain_log.fuse(), o!());

    let mut options = Options {
        file: false,
        builder: false,
        path_file: None,
        algo: None,
        count_of_batches: None,
        size_of_batch: None,
        all_random: false,
        local_random: false,
        low_range: None,
        high_range: None,
        cache_size: None,
    };
    let usage = format!("Usage: {} -f PATH | -b COUNT_OF_BATCHES | -s SIZE_OF_BATCH \
                 [-L | -R [-r RANDOM_LOW RANDOM_HIGH]] [-S SIZE_CACHE] ALGO\n\
                 \n\
                 Create RAM from FILE:\n\
                 -f PATH                Create RAM from file\n\
                 \n\
                 Create RAM from BUILDER:\n\
                 -b COUNT_OF_BATCHES    Set count of batches\n\
                 -s SIZE_OF_BATCH       Set size of one BATCH\n\
                 -R                     Set random for all batches\n\
                 -L                     Set random for one batch (range random is size of batch)\n\
                 -r                     Set range limit of random\n\
                 -S                     Set size of cache\n\
                 \n\
                 Algorithms (ALGO variable):\n\
                 BELADY, FIFO, LRU, LFU, RR, MRU, SLRU", env::args().nth(0).unwrap());
    if env::args().len() < 2 {
        println!("{}", usage);
        return
    } else {
        let args: Vec<String> = env::args().collect();
        for (i, argument) in args.iter().enumerate() {
            match argument.as_ref() {
                "-f" => {
                    if options.builder {
                        println!("{}", usage);
                        return
                    }
                    options.file = true;
                    options.path_file = Some(env::args().nth(i + 1).unwrap());
                    debug!(logger, format!("Create from file {}", options.path_file.clone().unwrap()));
                },
                "-b" => {
                    if options.file {
                        println!("{}", usage);
                        return
                    }
                    options.builder = true;
                    options.count_of_batches = env::args().nth(i + 1).map(|x| x.parse().unwrap());
                    debug!(logger, format!("Set count of batches: {}", options.count_of_batches.unwrap()));
                },
                "-s" => {
                    if options.file {
                        println!("{}", usage);
                        return
                    }
                    options.builder = true;
                    options.size_of_batch = env::args().nth(i + 1).map(|x| x.parse().unwrap());
                    debug!(logger, format!("Set size of bath: {}", options.size_of_batch.unwrap()));
                },
                "-R" => {
                    if !(options.file || options.builder) {
                        println!("{}", usage);
                        return
                    }
                    options.all_random = true;
                    debug!(logger, "Set all random");
                },
                "-L" => {
                    if !(options.file || options.builder) {
                        println!("{}", usage);
                        return
                    }
                    options.local_random = true;
                    debug!(logger, "Set local random");
                },
                "-r" => {
                    if !(options.file || options.builder) {
                        println!("{}", usage);
                        return
                    }
                    options.low_range = env::args().nth(i + 1).map(|x| x.parse().unwrap());
                    options.high_range = env::args().nth(i + 2).map(|x| x.parse().unwrap());
                    debug!(logger, format!("Set range of random {} - {}",
                                           options.low_range.unwrap(), options.high_range.unwrap()));
                },
                "-S" => {
                    if !(options.file || options.builder) {
                        println!("{}", usage);
                        return
                    }
                    options.cache_size = env::args().nth(i + 1).map(|x| x.parse().unwrap());
                    if options.cache_size.unwrap() == 0 {
                        panic!("Cache size must be greater than 0!");
                    }
                },
                a @ "BELADY" | a @ "FIFO" | a @ "LRU" | a @ "LFU" | a @ "RR" | a @ "MRU" | a @ "SLRU" => {
                    if !(options.file || options.builder) {
                        println!("{}", usage);
                        return
                    }
                    options.algo = Some(a.to_string());
                    debug!(logger, format!("Algo is {}", options.algo.clone().unwrap()));
                },
                _ => {},
            }
        }
    }
    if options.algo.clone().is_none() || !(options.builder || options.file) {
        println!("{}", usage);
        return
    }

    let mut ram: Vec<Vec<i32>> = Vec::new();

    if options.builder {
        let mut ram_builder = RamBuilder::new(Some(logger.clone()));
        if let Some(count) = options.count_of_batches {
            ram_builder = ram_builder.with_count_batches(count);
        }
        if let Some(size) = options.size_of_batch {
            ram_builder = ram_builder.with_size_batch(size);
        }
        if options.all_random {
            ram_builder = ram_builder.with_all_random();
        }
        if options.local_random {
            ram_builder = ram_builder.with_local_random();
        }
        if options.low_range.is_some() && options.high_range.is_some() {
            ram_builder = ram_builder.with_range_random(options.low_range.unwrap(),
                                                        options.high_range.unwrap());
        }
        ram = ram_builder.build();
    }
    if options.file {
        ram = RamBuilder::from_file(Some(logger.clone()), &options.path_file.unwrap());
    }

    match options.algo.unwrap().as_str() {
        "BELADY" => {
            let mut belady_cache = BeladyCache::new(options.cache_size.unwrap_or(100), Some(logger.clone()));
            let result = belady_cache.run(&ram);
            println!("{:.3}", result.0 as f32 / (result.0 + result.1) as f32);
        },
        "FIFO" => {
            let mut cache = FifoCache::new(options.cache_size.unwrap_or(100), Some(logger.clone()));
            let result = cache.run(&ram);
            println!("{:.3}", result.0 as f32 / (result.0 + result.1) as f32);
        }
        "LRU" => {
            let mut cache = LRUCache::new(options.cache_size.unwrap_or(100), Some(logger.clone()));
            let result = cache.run(&ram);
            println!("{:.3}", result.0 as f32 / (result.0 + result.1) as f32);
        }
        "LFU" => {
            let mut cache = LFUCache::new(options.cache_size.unwrap_or(100), Some(logger.clone()));
            let result = cache.run(&ram);
            println!("{:.3}", result.0 as f32 / (result.0 + result.1) as f32);
        }
        "RR" => {
            let mut cache = RRCache::new(options.cache_size.unwrap_or(100), Some(logger.clone()));
            let result = cache.run(&ram);
            println!("{:.3}", result.0 as f32 / (result.0 + result.1) as f32);
        }
        "MRU" => {
            let mut cache = MRUCache::new(options.cache_size.unwrap_or(100), Some(logger.clone()));
            let result = cache.run(&ram);
            println!("{:.3}", result.0 as f32 / (result.0 + result.1) as f32);
        }
        "SLRU" => {
            let mut cache = SLRUCache::new(options.cache_size.unwrap_or(100), Some(logger.clone()));
            let result = cache.run(&ram);
            println!("{:.3}", result.0 as f32 / (result.0 + result.1) as f32);
        }
        _ => {
            println!("{}", usage);
            return
        }
    }
}
