use crossbeam_channel::bounded;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// In physics-resonance/src/main.rs, the AudioModel is guarded by a Mutex
// and passed to the audio stream callback (reader) while being potentially updated
// or polled by the fallback thread or the main loop if commands are processed.
// We simulate contention on the AudioModel Mutex itself.
// To do so we just create a dummy AudioModel, put it in a Mutex, and contend.

use resonance_audio::AudioModel;

#[test]
fn havoc_test_contention() {
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("havoc_test_contention_inner")
        .arg("--nocapture")
        .arg("--ignored")
        .status()
        .expect("Failed to execute subprocess");

    if status.code() == Some(101) {
        println!(
            "👺 Havoc: WRECKAGE! Application logic suffered severe starvation under contention!"
        );
    } else {
        panic!("Havoc failed to cause a crash! Code: {:?}", status.code());
    }
}

#[test]
#[ignore]
fn havoc_test_contention_inner() {
    if std::env::args().any(|arg| arg == "havoc_test_contention_inner") {
        let (cmd_tx, cmd_rx) = bounded(1024);
        let (snap_tx, snap_rx) = bounded(1);

        let model = AudioModel::new(60, 30, cmd_rx, snap_tx, None);
        let model_lock = Arc::new(Mutex::new(model));

        let mut handles = vec![];
        let running = Arc::new(std::sync::atomic::AtomicBool::new(true));

        // Spawn 100 threads that represent the audio thread repeatedly locking the data
        for _ in 0..100 {
            let data_clone = model_lock.clone();
            let running_clone = running.clone();
            handles.push(thread::spawn(move || {
                while running_clone.load(std::sync::atomic::Ordering::Relaxed) {
                    let mut _lock = data_clone.lock().unwrap();
                    // Hog lock
                    thread::sleep(Duration::from_millis(10));
                }
            }));
        }

        // Give readers time to start
        thread::sleep(Duration::from_millis(50));

        let data_clone = model_lock.clone();

        // Spawn writer thread (update function)
        let writer_handle = thread::spawn(move || {
            let start = std::time::Instant::now();
            let mut _lock = data_clone.lock().unwrap();
            start.elapsed()
        });

        thread::sleep(Duration::from_millis(500));
        running.store(false, std::sync::atomic::Ordering::Relaxed);

        let elapsed = writer_handle.join().unwrap();

        for h in handles {
            let _ = h.join();
        }

        // If it took more than 50ms to acquire the lock, it's starved!
        if elapsed > Duration::from_millis(50) {
            std::process::exit(101); // 101 means havoc succeeded
        } else {
            std::process::exit(0);
        }
    }
}
