use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[test]
fn test_thread_contention() {
    let instrument = Arc::new(Mutex::new(()));
    let instrument_clone = instrument.clone();

    let t1 = thread::spawn(move || {
        let _guard = instrument_clone.lock().unwrap();
        thread::sleep(Duration::from_millis(500));
    });

    thread::sleep(Duration::from_millis(100)); // Ensure T1 has the lock

    let instrument_clone2 = instrument.clone();
    let t2 = thread::spawn(move || {
        let lock_start = Instant::now();
        let _guard = instrument_clone2.lock().unwrap();
        let duration = lock_start.elapsed();
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
