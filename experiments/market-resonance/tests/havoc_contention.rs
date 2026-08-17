use resonance_audio::AudioModel;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// We simulate Mutex starvation using an inner subprocess.
#[test]
fn test_audio_model_contention() {
    if std::env::var("RUN_HAVOC_INNER").is_ok() {
        let (_cmd_tx, cmd_rx) = crossbeam_channel::bounded(1024);
        let (snap_tx, _snap_rx) = crossbeam_channel::bounded(1);

        let model = AudioModel::new(64, 64, cmd_rx, snap_tx, None);
        let model_lock = Arc::new(Mutex::new(model));

        // Thread 1: Main thread logic heavily locking to render or update.
        let main_lock = Arc::clone(&model_lock);
        let main_thread = thread::spawn(move || {
            for _ in 0..100 {
                let _lock = main_lock.lock().unwrap();
                // Simulate holding the lock for TUI rendering / processing.
                thread::sleep(Duration::from_millis(5));
            }
        });

        // Thread 2: High frequency audio callback that starves.
        let audio_lock = Arc::clone(&model_lock);
        let audio_thread = thread::spawn(move || {
            let mut buffer = vec![0.0; 256];
            let mut lock_failures = 0;
            let start = std::time::Instant::now();

            while start.elapsed() < Duration::from_millis(500) {
                match audio_lock.try_lock() {
                    Ok(mut model) => {
                        model.process(&mut buffer);
                    }
                    Err(_) => {
                        lock_failures += 1;
                    }
                }
                thread::sleep(Duration::from_micros(100)); // frequent callback
            }
            if lock_failures > 500 {
                println!(
                    "HAVOC: Severe AudioModel starvation detected: {} failed locks",
                    lock_failures
                );
                std::process::exit(101); // Trigger Red Phase test failure in inner proc
            }
        });

        main_thread.join().unwrap();
        audio_thread.join().unwrap();
        return;
    }

    let output = process::Command::new(std::env::current_exe().unwrap())
        .arg("--nocapture")
        .env("RUN_HAVOC_INNER", "1")
        .output()
        .unwrap();

    assert!(
        !output.status.success(),
        "Havoc expected Mutex starvation crash, but it passed safely?!"
    );
}
