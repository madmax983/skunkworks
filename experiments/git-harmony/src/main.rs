use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use git_scanner::GitScanner;
use mapping::map_event_to_command;
use tui::{draw_ui, init_tui, restore_tui};
use resonance_audio::audio::{AudioCommand, AudioSnapshot};
use std::time::{Duration, Instant};
use crossbeam_channel::bounded;

pub mod audio;
pub mod git_scanner;
pub mod mapping;
pub mod tui;

fn main() -> Result<()> {
    // 1. Setup Channels
    // Audio command channel
    let (cmd_tx, cmd_rx) = bounded(1024);
    // Snapshot channel (for visualization)
    let (snap_tx, snap_rx) = bounded(2);

    // 2. Init Audio
    let _audio_handle = audio::init_audio(cmd_rx, snap_tx)?;

    // 3. Init Git Scanner
    // Try to open current directory, fallback to parent if fails (e.g. inside experiments/git-harmony)
    let scanner_result = GitScanner::new(".", 100).or_else(|_| GitScanner::new("..", 100));

    let mut scanner = match scanner_result {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to open git repo: {}. Exiting.", e);
            return Ok(());
        }
    };

    // 4. Init TUI
    let mut terminal = init_tui()?;

    // 5. Loop State
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(100); // 10 events per second
    let mut current_snapshot: Option<AudioSnapshot> = None;
    let mut status_msg = String::from("Press 'q' to quit. 'p' to pluck random.");
    let mut paused = false;

    loop {
        // Draw TUI
        terminal.draw(|f| {
            draw_ui(f, &current_snapshot, &status_msg);
        })?;

        // Handle Input
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('p') => {
                            // Manual pluck
                            let _ = cmd_tx.send(AudioCommand::Pluck {
                                x: 50,
                                y: 50,
                                strength: 1.0,
                            });
                            status_msg = "Manual Pluck!".to_string();
                        },
                        KeyCode::Char(' ') => {
                            paused = !paused;
                            status_msg = if paused { "Paused".to_string() } else { "Resumed".to_string() };
                        }
                        _ => {}
                    }
                }
            }
        }

        // Receive Audio Snapshot
        // We only care about the latest one
        while let Ok(snap) = snap_rx.try_recv() {
            current_snapshot = Some(snap);
        }

        // Process Git Events
        if !paused && last_tick.elapsed() >= tick_rate {
            if let Some(event) = scanner.next_event() {
                status_msg = format!("{} | {} | +{}/-{}", event.author, event.file_path, event.insertions, event.deletions);
                let cmd = map_event_to_command(&event, 100, 100);
                let _ = cmd_tx.send(cmd);
            } else {
                //status_msg = "History ended.".to_string();
                // Maybe restart? Or just wait.
            }
            last_tick = Instant::now();
        }
    }

    // Teardown
    restore_tui(&mut terminal)?;

    Ok(())
}
