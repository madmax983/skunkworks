use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::{Duration, Instant};
use tui_shared::Tui;

mod diff;
mod sim;
mod ui;

use sim::World;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let size = tui.terminal.size()?;
    // Reserve space for status bar (3 lines)
    let initial_height = size.height.saturating_sub(3);
    let mut world = World::new(size.width as f64, initial_height as f64);

    // Load initial diff
    let chars = diff::get_deleted_chars()?;
    if !chars.is_empty() {
        world.spawn_particles(&chars);
    } else {
        // Fallback for demo if no diff
        let demo_text: Vec<char> = "NO DELETED CODE FOUND - TRY DELETING LINES"
            .chars()
            .collect();
        world.spawn_particles(&demo_text);
    }

    let tick_rate = Duration::from_millis(30);
    let mut last_tick = Instant::now();

    loop {
        // Draw
        tui.terminal.draw(|f| {
            ui::draw(f, &world);
        })?;

        // Input
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char(' ') => world.shake(),
                    KeyCode::Char('r') => {
                        let chars = diff::get_deleted_chars()?;
                        if !chars.is_empty() {
                            world.spawn_particles(&chars);
                        }
                    }
                    _ => {}
                },
                Event::Resize(w, h) => {
                    // Update world dimensions
                    world.width = w as f64;
                    world.height = h.saturating_sub(3) as f64;

                    // Resize pile array
                    let new_len = world.width.ceil() as usize + 1;
                    // Reset pile to avoid index errors, physics will rebuild it as particles fall
                    // or we could try to preserve it, but simpler to clear.
                    // Actually, if we resize, the x-indices change meaning relative to screen?
                    // No, x is absolute char position.
                    // We just resize the buffer.
                    world.pile_heights.resize(new_len, 0.0);
                }
                _ => {}
            }
        }

        // Update Physics
        if last_tick.elapsed() >= tick_rate {
            // Use fixed DT for consistent physics, or real DT
            let _dt = last_tick.elapsed().as_secs_f64();
            last_tick = Instant::now();

            // Physics steps.
            // 0.1s step size seems okay for visual feel
            world.update(0.1);
        }
    }

    Ok(())
}
