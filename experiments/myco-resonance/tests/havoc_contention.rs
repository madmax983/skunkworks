use myco_transit::World;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// We simulate Mutex starvation using an inner subprocess.
#[test]
fn test_audio_model_contention() {
    if std::env::var("RUN_HAVOC_INNER").is_ok() {
        // Mock a world lock which is heavily contended in myco-resonance
        let world = World::new(64, 64);
        let myco_world = Arc::new(Mutex::new(world));

        // Thread 1: Main thread logic heavily locking to render or update.
        let main_lock = Arc::clone(&myco_world);
        let main_thread = thread::spawn(move || {
            for _ in 0..100 {
                let mut _world = main_lock.lock().unwrap();
                // Simulate holding the lock for TUI rendering / processing.
                thread::sleep(Duration::from_millis(5));
            }
        });

        // Thread 2: Logic thread that starves.
        let logic_lock = Arc::clone(&myco_world);
        let logic_thread = thread::spawn(move || {
            let mut lock_failures = 0;
            let start = std::time::Instant::now();

            while start.elapsed() < Duration::from_millis(500) {
                match logic_lock.try_lock() {
                    Ok(_) => {}
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
        logic_thread.join().unwrap();
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
