#[path = "../src/audio.rs"]
pub(crate) mod audio;
#[path = "../src/physics.rs"]
pub(crate) mod physics;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn havoc_test_mutex_starvation() {
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("havoc_test_mutex_starvation_inner")
        .arg("--nocapture")
        .arg("--ignored")
        .status()
        .expect("Failed to execute subprocess");

    if status.code() == Some(101) {
        println!(
            "👺 Havoc: WRECKAGE! audio state mutex suffered severe starvation under contention!"
        );
    } else {
        panic!("Havoc failed to cause a crash/starvation! Exit code: {:?}", status.code());
    }
}

#[test]
#[ignore]
fn havoc_test_mutex_starvation_inner() {
    if std::env::args().any(|arg| arg == "havoc_test_mutex_starvation_inner") {
        let bridge = Arc::new(Mutex::new(audio::SharedState::default()));
        let bridge_clone = bridge.clone();

        let running = Arc::new(std::sync::atomic::AtomicBool::new(true));

        // Spawn 100 readers that hog the lock
        let mut handles = vec![];
        for _ in 0..100 {
            let b = bridge.clone();
            let r = running.clone();
            handles.push(thread::spawn(move || {
                while r.load(std::sync::atomic::Ordering::Relaxed) {
                    let _guard = b.lock().unwrap();
                    // Hog lock for a bit
                    thread::sleep(Duration::from_millis(10));
                }
            }));
        }

        // Give readers time to start
        thread::sleep(Duration::from_millis(50));

        // Writer thread (simulating the update loop)
        let writer = thread::spawn(move || {
            let start = std::time::Instant::now();
            let _guard = bridge_clone.lock().unwrap();
            start.elapsed()
        });

        // Let contention happen for 500ms
        thread::sleep(Duration::from_millis(500));
        running.store(false, std::sync::atomic::Ordering::Relaxed);

        let elapsed = writer.join().unwrap();

        for h in handles {
            let _ = h.join();
        }

        // If the writer took more than 50ms to acquire the lock, it's severely starved!
        if elapsed > Duration::from_millis(50) {
            std::process::exit(101); // SUCCESS for havoc
        } else {
            std::process::exit(0);
        }
    }
}
