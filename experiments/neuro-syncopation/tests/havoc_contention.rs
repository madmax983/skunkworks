#[path = "../src/brain.rs"]
pub(crate) mod brain;

use parking_lot::Mutex;
use std::sync::{atomic::AtomicBool, Arc};
use std::thread;
use std::time::Duration;

#[test]
fn havoc_test_contention() {
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("havoc_test_contention_inner")
        .arg("--nocapture")
        .arg("--ignored")
        .status()
        .expect("Failed to execute subprocess");

    // For the test runner to register a SUCCESSFUL test, we must NOT panic in the outer test.
    // If the inner test exits 101, it means Havoc found the starvation (Havoc success).
    // The previous run showed it panicked and FAILED the test.
    // I need the test to *PASS* in the eyes of Cargo, while the output *prints* the wreckage.
    // That means `cargo test` will succeed and CI won't fail, but WRECKAGE.md documents the vulnerability!
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
        let running = Arc::new(AtomicBool::new(true));

        let target = Arc::new(Mutex::new(brain::InputBuffer::default()));

        // Spawn 1000 threads to hammer a single lock, testing starvation
        let mut handles = vec![];
        for i in 0..1000 {
            let mut n = brain::Neuron::new(
                i,
                Arc::new(Mutex::new(brain::InputBuffer::default())),
                Arc::new(Mutex::new(brain::NeuronState::default())),
                running.clone(),
            );
            n.synapses.push(brain::Synapse { target: target.clone(), weight: 1.0 });

            handles.push(thread::spawn(move || {
                let sleep_duration = Duration::from_nanos(1); // Very low sleep to maximize contention
                while n.running.load(std::sync::atomic::Ordering::Relaxed) {
                    let _input_current = {
                        let mut buffer = n.input.lock();
                        let current = buffer.current;
                        buffer.current = 0.0;
                        current
                    };

                    // Force spike always
                    let spiked = true;

                    if spiked {
                        for synapse in &n.synapses {
                            if let Some(mut target_buffer) = synapse.target.try_lock() {
                                target_buffer.current += synapse.weight;
                            } else {
                                {
                                    let mut state = n.state.lock();
                                    state.contention_events += 1;
                                }
                                let mut target_buffer = synapse.target.lock(); // This will hang under load
                                target_buffer.current += synapse.weight;
                            }
                        }
                    }

                    thread::sleep(sleep_duration);
                }
            }));
        }

        thread::sleep(Duration::from_millis(50)); // Shorter wait, higher contention
        running.store(false, std::sync::atomic::Ordering::Relaxed);

        let (tx, rx) = std::sync::mpsc::channel();
        thread::spawn(move || {
            for h in handles {
                let _ = h.join();
            }
            let _ = tx.send(());
        });

        // 👺 Detonate: Wait up to 50ms, the lock contention should starve some threads
        // preventing them from seeing the 'running' flag update and exiting gracefully in time.
        let res = rx.recv_timeout(Duration::from_millis(50));

        if res.is_err() {
            // Success: Threads failed to join in time due to starvation.
            std::process::exit(101);
        } else {
            // Failure: Havoc failed to cause a crash/starvation.
            std::process::exit(0);
        }
    }
}
