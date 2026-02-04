pub mod audio;
pub mod physics;
pub mod ui;

use anyhow::Result;
use audio::{run_audio, AudioCommand};
use crossterm::{
    event::{self, Event, KeyCode, MouseButton, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use crossbeam_channel::{bounded, Sender};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

fn main() -> Result<()> {
    // Setup Audio
    let width = 60;
    let height = 30;
    let (cmd_tx, cmd_rx) = bounded(100);
    let (snap_tx, snap_rx) = bounded(1); // Keep it fresh, drop old

    // Start audio stream
    // Note: If audio fails (e.g. no device), we should probably panic or warn.
    // Given the task is about sound, we expect it to work.
    let _stream = run_audio(width, height, cmd_rx, snap_tx)?;

    // Setup TUI
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, width, height, cmd_tx, snap_rx);

    // Restore
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{:?}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    width: usize,
    height: usize,
    cmd_tx: Sender<AudioCommand>,
    snap_rx: crossbeam_channel::Receiver<Vec<f32>>,
) -> io::Result<()> {
    let mut cursor_x = width / 2;
    let mut cursor_y = height / 2;
    let mut listener_x = width / 2;
    let mut listener_y = height / 2;

    // Local copy of walls for rendering
    let mut walls = vec![false; width * height];
    let mut pressure = vec![0.0; width * height];

    loop {
        // Poll for snapshot (non-blocking)
        if let Ok(snap) = snap_rx.try_recv() {
            pressure = snap;
        }

        terminal.draw(|f| {
            let info = format!(
                "Pos: ({}, {}) | Listener: ({}, {}) | SPACE: Pluck | w: Wall | l: Move Listener | 1-3: Presets | q: Quit",
                cursor_x, cursor_y, listener_x, listener_y
            );
            ui::draw(f, width, height, &pressure, &walls, (listener_x, listener_y), (cursor_x, cursor_y), &info);
        })?;

        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key) => {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Up => cursor_y = cursor_y.saturating_sub(1),
                        KeyCode::Down => {
                            if cursor_y < height - 1 {
                                cursor_y += 1;
                            }
                        }
                        KeyCode::Left => cursor_x = cursor_x.saturating_sub(1),
                        KeyCode::Right => {
                            if cursor_x < width - 1 {
                                cursor_x += 1;
                            }
                        }
                        KeyCode::Char(' ') => {
                            let _ = cmd_tx.send(AudioCommand::Pluck(cursor_x, cursor_y, 1.0));
                        }
                        KeyCode::Char('w') => {
                            let idx = cursor_y * width + cursor_x;
                            walls[idx] = !walls[idx];
                            if walls[idx] {
                                let _ = cmd_tx.send(AudioCommand::AddWall(cursor_x, cursor_y));
                            } else {
                                let _ = cmd_tx.send(AudioCommand::RemoveWall(cursor_x, cursor_y));
                            }
                        }
                        KeyCode::Char('l') => {
                            listener_x = cursor_x;
                            listener_y = cursor_y;
                            let _ = cmd_tx.send(AudioCommand::MoveListener(cursor_x, cursor_y));
                        }
                        KeyCode::Char('1') => {
                            // Clear
                            walls.fill(false);
                            // Need to sync with audio.
                            // This is inefficient (many messages), but fine for this scale.
                            // Better: AudioCommand::Clear?
                            // For now, iterate.
                            for y in 0..height {
                                for x in 0..width {
                                     let _ = cmd_tx.send(AudioCommand::RemoveWall(x, y));
                                }
                            }
                        }
                        KeyCode::Char('2') => {
                            // Box
                            walls.fill(false);
                            for y in 0..height {
                                for x in 0..width {
                                     let _ = cmd_tx.send(AudioCommand::RemoveWall(x, y));
                                }
                            }
                            // Add walls at border
                            for x in 0..width {
                                let _ = cmd_tx.send(AudioCommand::AddWall(x, 0));
                                walls[0 * width + x] = true;
                                let _ = cmd_tx.send(AudioCommand::AddWall(x, height - 1));
                                walls[(height - 1) * width + x] = true;
                            }
                            for y in 0..height {
                                let _ = cmd_tx.send(AudioCommand::AddWall(0, y));
                                walls[y * width + 0] = true;
                                let _ = cmd_tx.send(AudioCommand::AddWall(width - 1, y));
                                walls[y * width + width - 1] = true;
                            }
                        }
                        KeyCode::Char('3') => {
                             // "Chamber" - Box with a baffle
                             // Reuse Box logic first?
                             // Just draw baffle
                             let center_x = width / 2;
                             for y in 5..height-5 {
                                 let _ = cmd_tx.send(AudioCommand::AddWall(center_x, y));
                                 walls[y * width + center_x] = true;
                             }
                        }
                        _ => {}
                    }
                }
                Event::Mouse(mouse) => {
                    match mouse.kind {
                        MouseEventKind::Down(MouseButton::Left) => {
                            let x = mouse.column as usize;
                            let y = mouse.row as usize;
                            if x < width && y < height {
                                cursor_x = x;
                                cursor_y = y;
                                // Pluck on click? Or Wall?
                                // Let's toggle wall
                                let idx = y * width + x;
                                walls[idx] = !walls[idx];
                                if walls[idx] {
                                    let _ = cmd_tx.send(AudioCommand::AddWall(x, y));
                                } else {
                                    let _ = cmd_tx.send(AudioCommand::RemoveWall(x, y));
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}
