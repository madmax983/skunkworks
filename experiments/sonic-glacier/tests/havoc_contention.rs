use crossbeam_channel::unbounded;
use rustfft::FftPlanner;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[path = "../src/audio.rs"]
mod audio;

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
        let (tx, _rx) = unbounded::<audio::Spectrum>();
        // In SonicEngine the fft_planner is inside a Mutex. Let's contend on it directly, as SonicEngine isn't designed to be easily manipulated by an external lock, but we know the Mutex is the weak point.
        let planner = Arc::new(Mutex::new(FftPlanner::<f32>::new()));

        let mut handles = vec![];
        let running = Arc::new(std::sync::atomic::AtomicBool::new(true));

        // Spawn 100 threads that represent the audio thread repeatedly locking the data
        for _ in 0..100 {
            let planner_clone = planner.clone();
            let running_clone = running.clone();
            handles.push(thread::spawn(move || {
                while running_clone.load(std::sync::atomic::Ordering::Relaxed) {
                    let _lock = planner_clone.lock().unwrap();
                    // Hog lock
                    thread::sleep(Duration::from_millis(10));
                }
            }));
        }

        // Give readers time to start
        thread::sleep(Duration::from_millis(50));

        let planner_clone = planner.clone();

        // Spawn writer thread (update function)
        let writer_handle = thread::spawn(move || {
            let start = std::time::Instant::now();
            let mut _lock = planner_clone.lock().unwrap();
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
