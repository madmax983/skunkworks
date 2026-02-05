mod audio;
mod drummer;
mod tui;

use crate::audio::{AudioEngine, SoundEvent};
use crate::drummer::{Drummer, DrummerState};
use crate::tui::{run_app, App};
use crossbeam::channel::unbounded;
use std::sync::{Arc, Mutex};

fn main() -> anyhow::Result<()> {
    let (audio_tx, audio_rx) = unbounded::<SoundEvent>();
    let (state_tx, state_rx) = unbounded::<(usize, DrummerState)>();

    // Initialize Audio
    // Note: AudioEngine holds the stream, so it must stay alive.
    let _audio = AudioEngine::new(audio_rx)?;

    // Shared Resources (The "Drums")
    let kick_mutex = Arc::new(Mutex::new(()));
    let snare_mutex = Arc::new(Mutex::new(()));

    // Polyrhythmic Configuration
    // ID | Interval (ms) | Resource
    // 0  | 500 (120 BPM) | Kick
    // 1  | 666 (Triplet) | Kick (Contention with 0)
    // 2  | 375 (Dotted 8)| Snare
    // 3  | 250 (16th)    | Snare (Contention with 2)

    let configs = vec![
        (500, kick_mutex.clone()),
        (666, kick_mutex.clone()),
        (375, snare_mutex.clone()),
        (250, snare_mutex.clone()),
    ];

    for (i, (interval, resource)) in configs.into_iter().enumerate() {
        let d = Drummer {
            id: i,
            interval_ms: interval,
            resource,
            audio_tx: audio_tx.clone(),
            state_tx: state_tx.clone(),
        };
        d.spawn();
    }

    // Run TUI
    let app = App::new(4, state_rx);
    run_app(app)?;

    Ok(())
}
