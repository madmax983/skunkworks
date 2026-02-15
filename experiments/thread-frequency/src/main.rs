mod audio;
mod musician;
mod tui;

use anyhow::Result;
use audio::{AudioEngine, Voice};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use musician::{Beacon, Musician};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::{io, time::Duration};
use tui::{draw_ui, TuiState};

fn main() -> Result<()> {
    // Audio Setup
    // AudioEngine::new might fail in CI/Sandbox if no audio device.
    // We try to handle it gracefully by allowing simulation mode (via features) or just failing if strict.
    // Since we refactored audio.rs to have a fallback simulation mode via cfg, this should be fine
    // provided the feature flags are set correctly.
    // If 'audio' feature is enabled but fails (e.g. no device), it returns Err.
    // We can't easily fallback to simulation at runtime if it's compile-time cfg.
    // But we can just let it fail and print error.
    let (audio_engine, tx, current_time) = match AudioEngine::new() {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to initialize audio: {}", e);
            return Err(e);
        }
    };

    let sample_rate = audio_engine.sample_rate();

    // Prevent audio engine from being dropped
    #[allow(unused_variables)]
    let _audio_engine = audio_engine;

    // Shared Resource for Synchronization (The "Drum Circle Center")
    // Threads contend for this lock.
    let shared_resource = Arc::new(Mutex::new(()));
    let running = Arc::new(AtomicBool::new(true));

    // Musicians configuration
    // (Name, Period(ms), Voice, WorkLoad(iters), Drift(ms))
    let configs = vec![
        ("Kick", 500, Voice::Kick, 1000, 0),          // Anchor: 120 BPM, stable
        ("Snare", 666, Voice::Snare, 2000, 5),        // Polyrhythm 3:4ish, slight drift
        ("HiHat", 250, Voice::Hihat, 500, 15),        // Fast, jittery (human feel)
        ("Perc", 400, Voice::Clave, 3000, 2),         // 150 BPM, contends moderately
        ("Bass", 1500, Voice::Synth(0), 10000, 0),    // Slow, Heavy work (blocks others)
        ("Pad", 1103, Voice::Synth(7), 5000, 10),     // Prime period, moderate work
        ("Glitch", 293, Voice::Synth(12), 100, 50),   // Fast prime, very jittery
    ];

    let mut names = Vec::new();
    let mut periods = Vec::new();
    let mut voices = Vec::new();
    let mut beacons = Vec::new();
    let mut handles = Vec::new();

    for (id, (name, period, voice, work, drift)) in configs.into_iter().enumerate() {
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
            work,
            drift,
        );

        handles.push(musician.spawn());
    }

    // TUI Setup
    // Use a guard to ensure cleanup even on panic?
    // Rust doesn't have try-finally, but Drop trait handles it.
    // For now, standard pattern.
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
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = std::time::Instant::now();
        }

        // Check if we should stop (external signal?)
        if !running.load(Ordering::Relaxed) {
            break;
        }
    }

    // Cleanup
    running.store(false, Ordering::Relaxed);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    println!("Session ended. Threads are winding down...");

    // We don't join threads because they might be sleeping or stuck in loops.
    // The OS will clean them up on exit.
    // If we wanted to be clean, we'd join, but `running` flag should stop them eventually.

    Ok(())
}
