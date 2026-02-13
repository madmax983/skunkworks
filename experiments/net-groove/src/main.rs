pub mod audio;
pub mod network;
pub mod shared;
pub mod tui;

use std::sync::Arc;
use anyhow::Result;

use crate::audio::AudioEngine;
use crate::network::start_monitoring;
use crate::shared::GrooveState;
use crate::tui::AppTui;

fn main() -> Result<()> {
    // 1. Initialize Shared State
    let state = Arc::new(GrooveState::new());

    // 2. Start Network Monitoring (Spawns threads)
    start_monitoring(state.clone());

    // 3. Start Audio Engine (Spawns thread or stream)
    let _audio_engine = AudioEngine::new(state.clone())?;

    // 4. Run TUI (Main thread blocks here)
    let mut tui = AppTui::new(state);
    tui.run()?;

    Ok(())
}
