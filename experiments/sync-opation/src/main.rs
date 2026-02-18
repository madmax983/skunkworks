mod audio;
mod conductor;
mod ui;

use anyhow::Result;
use crossbeam::channel;
use crate::audio::AudioEngine;
use crate::conductor::Conductor;
use crate::ui::App;

fn main() -> Result<()> {
    // 1. Audio Channel
    let (audio_tx, audio_rx) = channel::unbounded();

    // 2. UI Channel
    let (ui_tx, ui_rx) = channel::unbounded();

    // 3. Initialize Audio Engine
    // Note: AudioEngine holds the stream alive as long as it exists.
    let _audio_engine = AudioEngine::new(audio_rx)?;

    // 4. Initialize Conductor
    let mut conductor = Conductor::new(audio_tx, ui_tx);

    // 5. Add Musicians (Polyrhythms)
    // Using prime-ish numbers for loop durations to create phasing patterns.
    // Frequencies form an A Major Pentatonic chord spread out.

    // Musician 1: Base pulse (Anchor)
    conductor.add_musician(1, 400, 80, 220.0); // A3

    // Musician 2: 5/4 against M1 roughly
    conductor.add_musician(2, 500, 80, 277.18); // C#4

    // Musician 3: 6/4 against M1
    conductor.add_musician(3, 600, 80, 329.63); // E4

    // Musician 4: 7/4 against M1
    conductor.add_musician(4, 700, 80, 440.0); // A4

    // Musician 5: Slower, deep cycle
    conductor.add_musician(5, 1100, 150, 110.0); // A2 (Bass)

    // 6. Run UI
    let mut app = App::new(ui_rx);
    app.run()?;

    // 7. Cleanup
    conductor.stop();

    Ok(())
}
