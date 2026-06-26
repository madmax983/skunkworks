#[path = "../src/audio.rs"]
pub(crate) mod audio;
#[path = "../src/rhythm.rs"]
mod rhythm;

use crossbeam::channel;
use rhythm::Musician;
use std::sync::{atomic::AtomicBool, Arc, Mutex};
use std::thread;
use std::time::Duration;

// 👺 Havoc: Prove that `turbulent-rhythms` Musicians can suffer starvation!
// The thread logic loops infinitely. If we set parameters such that threads compete
// and starve each other, or if they take multiple locks, they could deadlock.
#[test]
fn havoc_test_contention() {
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("havoc_test_contention_inner")
        .arg("--nocapture")
        .arg("--ignored")
        .status()
        .expect("Failed to execute subprocess");

    if !status.success() {
        println!(
            "👺 Havoc: WRECKAGE! Application logic suffered severe starvation under contention!"
        );
    } else {
        panic!("Havoc failed to cause a crash!");
    }
}

#[test]
#[ignore]
fn havoc_test_contention_inner() {
    if std::env::args().any(|arg| arg == "havoc_test_contention_inner") {
        let beat_lock = Arc::new(Mutex::new(()));
        let (audio_tx, _audio_rx) = channel::unbounded();
        let (rhythm_tx, _rhythm_rx) = channel::unbounded();
        let running = Arc::new(AtomicBool::new(true));

        // Spawn 10 threads all hammering the same lock with 0ms loop duration!
        let mut handles = vec![];
        for i in 0..10 {
            let musician = Musician {
                id: i,
                pitch: 440.0,
                loop_duration_ms: 0,
                hold_duration_ms: 100, // Hog the lock
            };
            handles.push(musician.start(
                beat_lock.clone(),
                audio_tx.clone(),
                rhythm_tx.clone(),
                running.clone(),
            ));
        }

        // Let them fight for 200ms
        thread::sleep(Duration::from_millis(200));

        // Send stop
        running.store(false, std::sync::atomic::Ordering::Relaxed);

        let (tx, rx) = std::sync::mpsc::channel();
        thread::spawn(move || {
            for handle in handles {
                let _ = handle.join();
            }
            let _ = tx.send(());
        });

        // If it doesn't join within 500ms, they are stuck/starved!
        let res = rx.recv_timeout(Duration::from_millis(500));

        // Havoc wants the test to FAIL when the bug is found.
        assert!(
            res.is_ok(),
            "👺 Havoc SUCCESS: Application logic suffered severe starvation under contention!"
        );
        std::process::exit(0);
    }
}
