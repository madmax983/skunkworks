use anyhow::Result;
use crossbeam_channel::unbounded;
use crossterm::event::{self, Event, KeyCode};
use std::time::{Duration, Instant};
use tui_shared::Tui;

mod audio;
mod sim;
mod tui;

use crate::audio::{AudioEngine, SpikeEvent};
use crate::sim::Cortex;

fn main() -> Result<()> {
    // TUI Init
    let mut terminal_interface = Tui::init()?;

    // Channels
    let (spike_tx, spike_rx) = unbounded::<SpikeEvent>();

    // Audio Engine
    // Note: AudioEngine holds the stream/thread, so it must be kept alive.
    let _audio_engine = AudioEngine::new(spike_rx)?;

    // Simulation
    let width = 100.0;
    let height = 100.0;
    let neuron_count = 200;
    let mut cortex = Cortex::new(width, height, neuron_count, spike_tx);

    let tick_rate = Duration::from_millis(16); // ~60 FPS
    let mut last_tick = Instant::now();

    loop {
        // Draw
        terminal_interface.terminal.draw(|f| {
            tui::draw(f, &cortex);
        })?;

        // Input
        // Poll with 0 timeout to check input immediately
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => break,
                    KeyCode::Char(' ') => {
                        cortex.stimulate_random(20.0);
                    }
                    _ => {}
                }
            }
        }

        // Update
        if last_tick.elapsed() >= tick_rate {
            // Run 10 simulation steps per frame to approximate real-time behavior
            // (10 * 1.0ms = 10ms simulated per ~16ms real time)
            for _ in 0..10 {
                cortex.update(1.0);
            }
            last_tick = Instant::now();
        } else {
            // Sleep to yield CPU
            std::thread::sleep(Duration::from_millis(1));
        }
    }

    Ok(())
}
