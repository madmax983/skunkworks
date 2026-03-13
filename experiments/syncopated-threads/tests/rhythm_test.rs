#[cfg(not(feature = "loom"))]
use std::sync::{Arc, Mutex};
#[cfg(feature = "loom")]
use loom::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
// use syncopated_threads::model::ThreadState; // This would fail compilation if I uncommented it, but I can't run cargo test if it doesn't compile at all?
// Rust TDD is tricky because compilation failure stops everything.

#[test]
fn test_thread_contention() {
    let instrument = Arc::new(Mutex::new(()));
    let _start_time = Instant::now();
    let instrument_clone = instrument.clone();

    // Thread 1: Holds lock for 500ms
    let t1 = thread::spawn(move || {
        let _guard = instrument_clone.lock().unwrap();
        thread::sleep(Duration::from_millis(500));
    });

    thread::sleep(Duration::from_millis(100)); // Ensure T1 has the lock

    let instrument_clone2 = instrument.clone();
    // Thread 2: Tries to acquire lock, should be blocked
    let t2 = thread::spawn(move || {
        let lock_start = Instant::now();
        let _guard = instrument_clone2.lock().unwrap();
        let duration = lock_start.elapsed();
        // Should have waited at least 400ms (500 - 100)
        assert!(
            duration >= Duration::from_millis(390),
            "Wait was too short: {:?}",
            duration
        );
    });

    t1.join().unwrap();
    t2.join().unwrap();
}

#[test]
fn test_channel_communication() {
    use crossbeam_channel::unbounded;
    let (s, r) = unbounded();
    s.send("Waiting").unwrap();
    assert_eq!(r.recv().unwrap(), "Waiting");
}
