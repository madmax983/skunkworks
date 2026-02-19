mod audio;
mod simulation;
mod ui;

use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;

use crate::audio::{AudioWriter, Waveform};
use crate::simulation::{GrooveThread, run_groove_thread};
use crate::ui::App;

fn main() -> anyhow::Result<()> {
    // 1. Setup Channels
    let (audio_tx, audio_rx) = std::sync::mpsc::channel();
    let (ui_tx, ui_rx) = std::sync::mpsc::channel();

    // 2. Setup Shared Resource (The "Atom")
    let lock = Arc::new(Mutex::new(()));

    // 3. Spawn Audio Thread
    let running = Arc::new(AtomicBool::new(true));
    let audio_running = running.clone();

    let audio_handle = thread::spawn(move || {
        let filename = "atomic_groove_session.wav";
        let mut writer = AudioWriter::new(filename, 44100, audio_rx).expect("Failed to create audio writer");

        let chunk_duration = 0.05; // 50ms

        while audio_running.load(Ordering::Relaxed) {
            if let Err(e) = writer.process_events() {
                eprintln!("Audio process error: {}", e);
                break;
            }
            if let Err(e) = writer.generate_chunk(chunk_duration) {
                eprintln!("Audio generate error: {}", e);
                break;
            }
            thread::sleep(Duration::from_secs_f32(chunk_duration));
        }

        println!("Finalizing Audio...");
        drop(writer);
        println!("Audio Saved to {}", filename);
    });

    // 4. Spawn Groove Threads
    let threads_config = vec![
        GrooveThread {
            id: 0,
            interval: Duration::from_millis(500),
            jitter: true,
            waveform: Waveform::Sine,
            frequency: 60.0,
            pan: 0.0,
        },
        GrooveThread {
            id: 1,
            interval: Duration::from_millis(250),
            jitter: true,
            waveform: Waveform::Noise,
            frequency: 1000.0,
            pan: -0.5,
        },
        GrooveThread {
            id: 2,
            interval: Duration::from_millis(750),
            jitter: true,
            waveform: Waveform::Square,
            frequency: 150.0,
            pan: 0.5,
        },
        GrooveThread {
            id: 3,
            interval: Duration::from_millis(600),
            jitter: true,
            waveform: Waveform::Sawtooth,
            frequency: 220.0,
            pan: 0.0,
        },
    ];

    for config in threads_config.clone() {
        let lock_clone = lock.clone();
        let audio_tx_clone = audio_tx.clone();
        let ui_tx_clone = ui_tx.clone();
        thread::spawn(move || {
            run_groove_thread(config, lock_clone, audio_tx_clone, ui_tx_clone);
        });
    }

    // 5. Run UI
    let app = App::new(threads_config.len(), ui_rx);

    match app.run() {
        Ok(_) => {},
        Err(e) => eprintln!("UI Error: {}", e),
    }

    // 6. Cleanup
    running.store(false, Ordering::Relaxed);
    let _ = audio_handle.join();

    Ok(())
}
