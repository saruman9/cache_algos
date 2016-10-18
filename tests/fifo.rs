extern crate cache_algos;

use cache_algos::memory::RamBuilder;
use cache_algos::fifo::FifoCache;

#[test]
fn check_simple_hit() {
    let ram = create_10_01_batches_with_2_elements();

    let mut fifo_cache = FifoCache::new(2, None);
    assert_eq!(fifo_cache.run(&ram), (18, 2));
}

#[test]
fn check_normal_hit() {
    let ram = create_100_batches_with_100_elements();

    let mut fifo_cache = FifoCache::new(150, None);
    assert_eq!(fifo_cache.run(&ram), (9900, 100));
}

fn create_10_01_batches_with_2_elements() -> Vec<Vec<i32>> {
    RamBuilder::new(None)
        .with_count_batches(10)
        .with_size_batch(2)
        .with_range_random(0, 1)
        .build()
}

fn create_100_batches_with_100_elements() -> Vec<Vec<i32>> {
    RamBuilder::new(None)
        .with_count_batches(100)
        .with_size_batch(100)
        .build()
}
