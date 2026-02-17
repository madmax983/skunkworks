use std::sync::Arc;
use std::time::Duration;
use thread_cycle::audio::AudioEvent;
use thread_cycle::simulation::{SharedResource, ThreadCycler};
use crossbeam_channel::unbounded;

#[test]
fn test_thread_cycler_lifecycle() {
    // Setup
    let resource = Arc::new(SharedResource::new());
    let cycler = ThreadCycler::new(
        1,
        Duration::from_millis(100), // Cycle time
        Duration::from_millis(50),  // Work time
        resource.clone(),
    );

    let (tx, rx) = unbounded();

    // Action: Run one cycle
    // This should attempt lock, acquire it (since resource is free), work, then release.
    cycler.run_one_cycle(|e| { tx.send(e).unwrap(); });

    // Verification
    // We expect:
    // 1. LockAttempt(1)
    // 2. LockAcquired(1)
    // 3. LockReleased(1)

    let events: Vec<AudioEvent> = rx.try_iter().collect();

    assert!(events.contains(&AudioEvent::LockAttempt(1)), "Missing LockAttempt");
    assert!(events.contains(&AudioEvent::LockAcquired(1)), "Missing LockAcquired");
    assert!(events.contains(&AudioEvent::LockReleased(1)), "Missing LockReleased");

    // Check order roughly
    let attempt_pos = events.iter().position(|e| *e == AudioEvent::LockAttempt(1)).unwrap();
    let acquire_pos = events.iter().position(|e| *e == AudioEvent::LockAcquired(1)).unwrap();
    let release_pos = events.iter().position(|e| *e == AudioEvent::LockReleased(1)).unwrap();

    assert!(attempt_pos < acquire_pos, "Attempt should be before Acquire");
    assert!(acquire_pos < release_pos, "Acquire should be before Release");
}
