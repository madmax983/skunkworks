mod audio;
mod drummer;
mod euclidean;
mod system;
mod tui;

use crate::audio::{AudioEngine, SoundEvent};
use crate::drummer::{Drummer, DrummerUpdate};
use crate::system::spawn_monitor;
use crate::tui::{run_app, App};
use crossbeam::channel::unbounded;
use std::sync::{Arc, Mutex};

fn main() -> anyhow::Result<()> {
    let (audio_tx, audio_rx) = unbounded::<SoundEvent>();
    let (state_tx, state_rx) = unbounded::<DrummerUpdate>();

    // Initialize Audio
    let _audio = AudioEngine::new(audio_rx)?;

    // Initialize System Monitor
    let cpu_load = spawn_monitor();

    // Shared Resources (The "Drums")
    let kick_mutex = Arc::new(Mutex::new(()));
    let snare_mutex = Arc::new(Mutex::new(()));

    // Euclidean Configuration
    // ID | k | n | Resource
    // All threads run at same pulse (120 BPM 16th note = 125ms)
    // This creates Polymeters (different cycle lengths)
    let pulse_ms = 125;

    let configs = vec![
        (3, 8, kick_mutex.clone()),   // Tresillo (Length 8)
        (5, 12, kick_mutex.clone()),  // African Bell (Length 12)
        (4, 16, snare_mutex.clone()), // 4-on-floor (Length 16)
        (7, 16, snare_mutex.clone()), // Samba-ish (Length 16)
    ];

    for (i, (k, n, resource)) in configs.into_iter().enumerate() {
        let d = Drummer {
            id: i,
            interval_ms: pulse_ms,
            resource,
            audio_tx: audio_tx.clone(),
            state_tx: state_tx.clone(),
            cpu_load: cpu_load.clone(),
            euclidean_k: k,
            euclidean_n: n,
        };
        d.spawn();
    }

    // Run TUI
    let app = App::new(4, state_rx, cpu_load);
    run_app(app)?;

    Ok(())
}
