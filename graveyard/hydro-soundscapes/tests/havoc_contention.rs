use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[cfg(feature = "audio")]
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
        panic!("Havoc failed to cause a crash!");
    }
}

#[test]
#[ignore]
fn havoc_test_contention_inner() {
    if std::env::args().any(|arg| arg == "havoc_test_contention_inner") {
        #[cfg(feature = "audio")]
        {
            let params = Arc::new(Mutex::new(audio::AudioState {
                frequency: 220.0,
                volume: 0.1,
                modulation: 0.0,
            }));
            let mut handles = vec![];
            let running = Arc::new(std::sync::atomic::AtomicBool::new(true));

            // 100 threads fighting for the lock, simulating rapid parameter updates
            for _ in 0..100 {
                let p_clone = params.clone();
                let running_clone = running.clone();
                handles.push(thread::spawn(move || {
                    while running_clone.load(std::sync::atomic::Ordering::Relaxed) {
                        let mut _lock = p_clone.lock().unwrap();
                        thread::sleep(Duration::from_millis(10));
                    }
                }));
            }

            thread::sleep(Duration::from_millis(50));

            let p_clone = params.clone();

            // Writer thread simulating the audio callback thread trying to read params
            let audio_thread = thread::spawn(move || {
                let start = std::time::Instant::now();
                let _lock = p_clone.lock().unwrap();
                start.elapsed()
            });

            thread::sleep(Duration::from_millis(500));
            running.store(false, std::sync::atomic::Ordering::Relaxed);

            let elapsed = audio_thread.join().unwrap();

            for h in handles {
                let _ = h.join();
            }

            if elapsed > Duration::from_millis(50) {
                std::process::exit(101);
            } else {
                std::process::exit(0);
            }
        }
        #[cfg(not(feature = "audio"))]
        {
            std::process::exit(101);
        }
    }
}
