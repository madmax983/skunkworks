mod audio;
mod sim;
mod ui;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use sim::Musician;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() -> Result<()> {
    // 1. Setup Audio
    let (audio_tx, audio_rx) = crossbeam::channel::unbounded();
    let audio_engine = audio::AudioEngine::new(audio_rx)?;

    // 2. Setup Simulation
    let instrument = Arc::new(Mutex::new(()));
    let running = Arc::new(AtomicBool::new(true));

    // Tuning: A Minor Pentatonic Scale
    // Intervals: Prime numbers for maximum phasing (in ms)
    let configs = vec![
        (307u64, 220.0f32, "Bass"),     // A3
        (401u64, 261.63f32, "Tenor"),   // C4
        (503u64, 293.66f32, "Alto"),    // D4
        (601u64, 329.63f32, "Soprano"), // E4
        (701u64, 392.00f32, "Lead"),    // G4
        (809u64, 440.00f32, "Air"),     // A4
    ];

    let mut musicians = Vec::new();
    let mut handles = Vec::new();

    for (id, (interval, freq, name)) in configs.into_iter().enumerate() {
        let musician = Musician::new(id, name.to_string());

        let handle = sim::spawn_musician(
            &musician,
            interval,
            freq,
            audio_tx.clone(),
            instrument.clone(),
            running.clone(),
        );

        musicians.push(musician);
        handles.push(handle);
    }

    // 3. Setup UI
    let mut tui = tui_shared::Tui::init()?;

    // 4. Main Loop
    loop {
        tui.terminal.draw(|f| {
            ui::draw(f, &musicians);
        })?;

        // Poll for events
        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                        break;
                    }
                }
                _ => {}
            }
        }
    }

    // 5. Cleanup
    // Signal threads to stop
    running.store(false, Ordering::Relaxed);

    // Drop TUI to restore terminal before printing logs (optional, but good)
    drop(tui);

    println!("Shutting down simulation...");

    // Join musician threads
    for handle in handles {
        let _ = handle.join();
    }
    println!("Musicians stopped.");

    // Drop audio_tx to signal audio engine to finish draining
    drop(audio_tx);

    // Join audio engine
    println!("Finalizing audio...");
    audio_engine.join()?;
    println!("Audio saved to 'thread_frequency_output.wav'.");

    Ok(())
}
