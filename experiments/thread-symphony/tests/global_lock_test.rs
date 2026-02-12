use thread_symphony::conductor::Stage;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

#[test]
fn test_global_lock_contention() {
    let stage = Arc::new(Stage::new());
    let start = Instant::now();
    let duration = Duration::from_millis(100);

    // Thread 1 locks GLOBAL
    let s1 = stage.clone();
    let t1 = thread::spawn(move || {
        let _guard = s1.lock_global();
        thread::sleep(duration);
    });

    // Thread 2 locks GLOBAL (should contend)
    let s2 = stage.clone();
    let t2 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        let _guard = s2.lock_global();
        thread::sleep(duration);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    assert!(start.elapsed() >= Duration::from_millis(190), "Global lock contention failed, elapsed: {:?}", start.elapsed());
}
