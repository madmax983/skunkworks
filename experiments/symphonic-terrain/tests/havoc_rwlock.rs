#[path = "../src/audio.rs"]
pub(crate) mod audio;

use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

#[test]
fn havoc_test_rwlock_contention() {
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("havoc_test_rwlock_contention_inner")
        .arg("--nocapture")
        .arg("--ignored")
        .status()
        .expect("Failed to execute subprocess");

    if status.code() == Some(101) {
        println!(
            "👺 Havoc: WRECKAGE! Application logic suffered severe starvation under contention!"
        );
    } else {
        panic!("Havoc failed to cause a crash!");
    }
}

#[test]
#[ignore]
fn havoc_test_rwlock_contention_inner() {
    if std::env::args().any(|arg| arg == "havoc_test_rwlock_contention_inner") {
        let state = Arc::new(RwLock::new(audio::SharedState::new()));

        let mut handles = vec![];
        let running = Arc::new(std::sync::atomic::AtomicBool::new(true));

        for _ in 0..100 {
            let state_clone = state.clone();
            let running_clone = running.clone();
            handles.push(thread::spawn(move || {
                while running_clone.load(std::sync::atomic::Ordering::Relaxed) {
                    let _read_lock = state_clone.read().unwrap();
                    // Readers hog the lock intensely
                    thread::sleep(Duration::from_millis(50));
                }
            }));
        }

        // Give readers time to start
        thread::sleep(Duration::from_millis(50));

        let state_clone = state.clone();

        // Spawn a thread to try to acquire write lock, and wait for it
        let writer_handle = thread::spawn(move || {
            let start = std::time::Instant::now();
            let _write_lock = state_clone.write().unwrap();
            start.elapsed()
        });

        thread::sleep(Duration::from_millis(200));
        running.store(false, std::sync::atomic::Ordering::Relaxed);

        let elapsed = writer_handle.join().unwrap();

        // If it took more than 5ms to acquire the write lock, it's starved!
        if elapsed > Duration::from_millis(5) {
            std::process::exit(101); // 101 means havoc succeeded
        } else {
            std::process::exit(0);
        }
    }
}
