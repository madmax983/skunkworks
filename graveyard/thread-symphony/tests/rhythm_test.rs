use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use thread_symphony::conductor::{Instrument, Stage};

#[test]
fn test_contention_delays_execution() {
    let stage = Arc::new(Stage::new());

    let start = Instant::now();
    let duration1 = Duration::from_millis(100);
    let duration2 = Duration::from_millis(100);

    let s1 = stage.clone();
    let t1 = thread::spawn(move || {
        let _guard = s1.lock_instrument(Instrument::Kick);
        thread::sleep(duration1);
    });

    let s2 = stage.clone();
    let t2 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10)); // Ensure t1 grabs it first
        let _guard = s2.lock_instrument(Instrument::Kick);
        thread::sleep(duration2);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let elapsed = start.elapsed();

    // If they ran in parallel, elapsed would be ~100ms.
    // Since they lock the SAME instrument (Kick), they must be serialized.
    // Total time >= 100ms + (100ms - 10ms overlap) = 190ms?
    // T1 starts at 0, ends at 100.
    // T2 starts waiting at 10, acquires at 100, ends at 200.
    // Total time ~200ms.

    assert!(
        elapsed >= Duration::from_millis(190),
        "Expected serial execution > 190ms, got {:?}",
        elapsed
    );
}
