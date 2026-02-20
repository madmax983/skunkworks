mod audio;
mod synth;

use anyhow::Result;
use audio::{AudioBackend, DummyBackend};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use synth::Waveform;

#[cfg(feature = "audio")]
use audio::CpalBackend;

fn main() -> Result<()> {
    println!("⚛️ Genesis: The Percussionist - Lock-Step Experiment ⚛️");
    println!("Starting polyrhythmic threads with resource contention...");

    // 1. Setup Audio
    #[cfg(feature = "audio")]
    let (_stream_guard, audio_handle): (Option<CpalBackend>, Arc<dyn AudioBackend>) =
        match CpalBackend::new() {
            Ok((b, h)) => (Some(b), Arc::new(h)),
            Err(e) => {
                eprintln!("Audio backend failed: {}. Using dummy.", e);
                (None, Arc::new(DummyBackend::new()?))
            }
        };

    #[cfg(not(feature = "audio"))]
    let audio_handle: Arc<dyn AudioBackend> = Arc::new(DummyBackend::new()?);

    // 2. Setup Shared Resource (The "Stage")
    let stage = Arc::new(Mutex::new(()));

    let mut handles = vec![];

    // Musician 1: Kick (The Foundation) - Period 800ms
    // Frequency: 50Hz Sine
    let b1 = audio_handle.clone();
    let s1 = stage.clone();
    handles.push(thread::spawn(move || {
        let period = Duration::from_millis(800);
        loop {
            thread::sleep(period);

            // Try to take the stage
            match s1.try_lock() {
                Ok(_guard) => {
                    // Success! Play loud kick
                    b1.play_note(50.0, 0.3, Waveform::Sine);
                    println!("[KICK ] 🔊 BOM");
                    // Occupy stage for 100ms
                    thread::sleep(Duration::from_millis(100));
                }
                Err(_) => {
                    // Failed! Play quiet click (ghost note)
                    b1.play_note(1000.0, 0.05, Waveform::Noise);
                    println!("[KICK ] ✖️ (clash)");
                }
            }
        }
    }));

    // Musician 2: Snare (The Backbeat) - Period 1200ms
    // Frequency: 200Hz Square
    let b2 = audio_handle.clone();
    let s2 = stage.clone();
    handles.push(thread::spawn(move || {
        let period = Duration::from_millis(1200);
        thread::sleep(Duration::from_millis(400)); // Offset
        loop {
            thread::sleep(period);

            match s2.try_lock() {
                Ok(_guard) => {
                    b2.play_note(200.0, 0.15, Waveform::Square);
                    // Add some noise for snare rattle
                    b2.play_note(800.0, 0.1, Waveform::Noise);
                    println!("[SNARE] 🥁 TACK");
                    thread::sleep(Duration::from_millis(80));
                }
                Err(_) => {
                    println!("[SNARE] ✖️ (clash)");
                }
            }
        }
    }));

    // Musician 3: Hi-Hat (The Clock) - Period 300ms
    // Frequency: Noise
    let b3 = audio_handle.clone();
    let s3 = stage.clone();
    handles.push(thread::spawn(move || {
        let period = Duration::from_millis(300);
        loop {
            thread::sleep(period);

            match s3.try_lock() {
                Ok(_guard) => {
                    b3.play_note(4000.0, 0.05, Waveform::Noise);
                    println!("[HAT  ] 🥢 tsst");
                    thread::sleep(Duration::from_millis(20)); // Short occupancy
                }
                Err(_) => {
                    println!("[HAT  ] ✖️");
                }
            }
        }
    }));

    // Musician 4: The Chaos (Random)
    let b4 = audio_handle.clone();
    let s4 = stage.clone();
    handles.push(thread::spawn(move || {
        loop {
            let delay = rand::random::<u64>() % 2000 + 500;
            thread::sleep(Duration::from_millis(delay));

            if let Ok(_guard) = s4.lock() {
                // This one WAITS (Swing/Drag)
                b4.play_note(110.0, 0.5, Waveform::Sine);
                println!("[CHAOS] 👻 WOOO (sync drag)");
                thread::sleep(Duration::from_millis(200));
            }
        }
    }));

    // Keep main thread alive
    for h in handles {
        h.join().unwrap();
    }

    Ok(())
}
