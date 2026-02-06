mod rhythm;
mod monitor;
mod audio;
mod tui;

use std::sync::{Arc, RwLock};
use monitor::{Monitor, SystemStats};
use audio::AudioEngine;
use tui::App;

fn main() -> anyhow::Result<()> {
    // Shared State
    let stats = Arc::new(RwLock::new(SystemStats::default()));

    // Monitor
    let monitor = Monitor::new(stats.clone());
    monitor.spawn();

    // Audio
    let _audio_engine = AudioEngine::new(stats.clone());

    // Log audio status if needed (TUI will overwrite stdout, so maybe not)
    if let Err(ref e) = _audio_engine {
        // If we want to debug, we can print to stderr before TUI starts
        eprintln!("Audio Warning: {}", e);
        // We continue anyway, so the TUI works.
    }

    // TUI
    let mut app = App::new(stats.clone());
    app.run()?;

    Ok(())
}
