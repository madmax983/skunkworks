use crossbeam_channel::unbounded;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use syncopated_threads::audio::AudioCommand;
use syncopated_threads::model::RhythmParams;
use syncopated_threads::threads::spawn_rhythm_thread;

// 👺 Havoc: Prove that the `syncopated-threads` instruments can suffer starvation!
// The thread logic loops infinitely. If we set parameters such that threads compete
// and starve each other, or if they take multiple locks, they could deadlock.
// Here we just test the application's actual `spawn_rhythm_thread` function!
#[test]
fn havoc_test_contention() {
    let snare = Arc::new(Mutex::new(()));

    let (state_tx, _state_rx) = unbounded();
    let (audio_tx, _audio_rx) = unbounded();
    let running = Arc::new(AtomicBool::new(true));

    // Spawn 10 threads all hammering the same snare lock!
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(spawn_rhythm_thread(
            i,
            snare.clone(),
            AudioCommand::Snare,
            state_tx.clone(),
            audio_tx.clone(),
            RhythmParams {
                sustain: Duration::from_millis(100),
                rest: Duration::from_millis(0), // No rest!
            },
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
    // If res.is_err(), we starved (timeout). We want to assert it is OK,
    // so that it panics and prints our Havoc message.
    assert!(
        res.is_ok(),
        "👺 Havoc SUCCESS: Application logic suffered severe starvation under contention!"
    );
}
