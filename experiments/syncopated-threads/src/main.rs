mod audio;
mod model;
mod threads;
mod tui;

use crate::audio::{start_audio_thread, AudioCommand};
use crate::model::{Instrument, RhythmParams};
use crate::threads::spawn_rhythm_thread;
use crate::tui::run_tui;
use crossbeam_channel::unbounded;
use std::sync::{atomic::AtomicBool, Arc, Mutex};
use std::time::Duration;

#[allow(clippy::vec_init_then_push)]
fn main() -> anyhow::Result<()> {
    // 1. Audio
    let (audio_sender, audio_receiver) = unbounded();
    let audio_handle = start_audio_thread(audio_receiver);

    // 2. State Channel for TUI
    let (state_sender, state_receiver) = unbounded();

    // 3. Instruments (Mutexes)
    let kick: Instrument = Arc::new(Mutex::new(()));
    let snare: Instrument = Arc::new(Mutex::new(()));
    let hat: Instrument = Arc::new(Mutex::new(()));

    let running = Arc::new(AtomicBool::new(true));

    // 4. Spawn Threads
    let mut handles = vec![];

    // Thread 0: Kick (4/4 approx - 500ms)
    handles.push(spawn_rhythm_thread(
        0,
        kick.clone(),
        AudioCommand::Kick,
        state_sender.clone(),
        audio_sender.clone(),
        RhythmParams {
            sustain: Duration::from_millis(50), // Hold lock for 50ms
            rest: Duration::from_millis(450),   // Total 500ms
        },
        running.clone(),
    ));

    // Thread 1: Snare (Polyrhythm - 333ms)
    handles.push(spawn_rhythm_thread(
        1,
        snare.clone(),
        AudioCommand::Snare,
        state_sender.clone(),
        audio_sender.clone(),
        RhythmParams {
            sustain: Duration::from_millis(50),
            rest: Duration::from_millis(283), // Total 333ms
        },
        running.clone(),
    ));

    // Thread 2: Hat (Fast - 200ms)
    handles.push(spawn_rhythm_thread(
        2,
        hat.clone(),
        AudioCommand::Hat,
        state_sender.clone(),
        audio_sender.clone(),
        RhythmParams {
            sustain: Duration::from_millis(20),
            rest: Duration::from_millis(180), // Total 200ms
        },
        running.clone(),
    ));

    // Thread 3: "The Jazz Player" - Contends for Snare periodically
    handles.push(spawn_rhythm_thread(
        3,
        snare.clone(), // Contends with Thread 1!
        AudioCommand::Snare,
        state_sender.clone(),
        audio_sender.clone(),
        RhythmParams {
            sustain: Duration::from_millis(100), // Hog the snare!
            rest: Duration::from_millis(700),
        },
        running.clone(),
    ));

    // 5. Run TUI (Blocks until 'q' pressed)
    run_tui(state_receiver, 4)?;

    // 6. Cleanup
    running.store(false, std::sync::atomic::Ordering::Relaxed);

    // Send stop signal to audio
    let _ = audio_sender.send(AudioCommand::Stop);

    // Join threads (Might take up to max rest duration)
    // We should probably just detach them or wait.
    // Waiting is polite.
    for handle in handles {
        let _ = handle.join();
    }
    let _ = audio_handle.join();

    Ok(())
}
