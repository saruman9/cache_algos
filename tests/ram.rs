extern crate cache_algos;

use cache_algos::memory::RamBuilder;

#[test]
fn ram_direct() {
    let ram = RamBuilder::new(None).build();

    for batch in ram {
        for elem in batch.iter().enumerate() {
            assert_eq!(elem.0 as i32, elem.1.clone());
        }
    }
}

#[test]
fn ram_with_10_batches() {
    let ram = RamBuilder::new(None)
        .with_count_batches(10)
        .build();

    assert_eq!(ram.len(), 10);
}

#[test]
fn ram_with_43_batches_4_elements() {
    let ram = RamBuilder::new(None)
        .with_count_batches(43)
        .with_size_batch(4)
        .build();

    assert_eq!(ram.len(), 43);
    for batch in ram {
        assert_eq!(batch.len(), 4);
    }
}
