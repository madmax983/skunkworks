use quipu::Cord;
use std::time::Instant;

#[test]
fn bench_cord_creation() {
    let start = Instant::now();
    let count = 100_000;

    // Use a simple LCG to avoid rand dependency
    let mut seed: u64 = 12345;
    for _ in 0..count {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let _ = Cord::from(seed);
    }

    let duration = start.elapsed();
    println!("Time for {} Cords: {:?}", count, duration);
}
