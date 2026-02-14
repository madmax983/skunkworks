mod audio;
mod musician;
mod tui;

use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::{io, time::Duration};
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use audio::{AudioEngine, Voice};
use musician::{Musician, Beacon};
use tui::{TuiState, draw_ui};

fn main() -> Result<()> {
    // Audio Setup
    // AudioEngine::new might fail in CI/Sandbox if no audio device.
    // We should handle it gracefully to allow TUI to run even if silent?
    // But AudioEngine::new returns current_time, which drives everything.
    // If AudioEngine fails, we can fallback to a dummy time source?
    // For now, let's propagate error and see.
    let (audio_engine, tx, current_time) = match AudioEngine::new() {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to initialize audio: {}", e);
            // If we are strictly "Audio-only generative music", this is fatal.
            // But we can try to be nice.
            return Err(e);
        }
    };

    let sample_rate = audio_engine.sample_rate();

    // Prevent audio engine from being dropped (it keeps the stream alive)
    #[allow(unused_variables)]
    let _audio_engine = audio_engine;

    // Shared Resource for Synchronization (The "Drum Circle Center")
    let shared_resource = Arc::new(Mutex::new(()));
    let running = Arc::new(AtomicBool::new(true));

    // Musicians configuration
    // Using prime-ish numbers for interesting phasing
    let configs = vec![
        ("Kick", 500, Voice::Kick),       // 120 BPM base
        ("Snare", 666, Voice::Snare),     // ~90 BPM
        ("HiHat", 250, Voice::Hihat),     // 240 BPM
        ("Clave", 400, Voice::Clave),     // 150 BPM
        ("Bass", 1500, Voice::Synth(0)),  // Slow bass
        ("Pad", 1103, Voice::Synth(7)),   // Prime 1103ms
        ("Glitch", 293, Voice::Synth(12)), // Prime 293ms
    ];

    let mut names = Vec::new();
    let mut periods = Vec::new();
    let mut voices = Vec::new();
    let mut beacons = Vec::new();
    let mut handles = Vec::new();

    for (id, (name, period, voice)) in configs.into_iter().enumerate() {
        let beacon = Arc::new(Beacon::new());
        beacons.push(beacon.clone());
        names.push(name.to_string());
        periods.push(period);
        voices.push(voice);

        let musician = Musician::new(
            id,
            name.to_string(),
            period,
            voice,
            tx.clone(),
            sample_rate,
            current_time.clone(),
            shared_resource.clone(),
            running.clone(),
            beacon,
        );

        handles.push(musician.spawn());
    }

    // TUI Setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tui_state = TuiState {
        musician_names: names,
        musician_periods: periods,
        musician_voices: voices,
        beacons,
        current_time,
        sample_rate,
    };

    let tick_rate = Duration::from_millis(32); // ~30 FPS
    let mut last_tick = std::time::Instant::now();

    loop {
        terminal.draw(|f| draw_ui(f, &tui_state))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = std::time::Instant::now();
        }
    }

    // Cleanup
    running.store(false, Ordering::Relaxed);
    // Wait for threads? No need, we exit process.

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
