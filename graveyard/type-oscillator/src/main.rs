use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode};
use std::time::{Duration, Instant};
use tui_shared::Tui;
use type_oscillator::{font_loader, glyph, modulator, ui};

fn main() -> Result<()> {
    // 1. Load Font
    let font = font_loader::load_font().context("Failed to load font")?;

    // 2. Initial Glyph
    let mut current_char = 'G'; // Genesis
    let mut base_points = glyph::extract_outline(&font, current_char);

    // 3. Setup Modulator
    let mut oscillator = modulator::Oscillator::new();

    // 4. Setup TUI
    let mut tui = Tui::init()?;

    // 5. Game Loop
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(16); // ~60 FPS

    loop {
        // Handle Input
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char(c) => {
                        // Change glyph if printable
                        if !c.is_control() {
                            current_char = c;
                            base_points = glyph::extract_outline(&font, current_char);
                        }
                    }
                    KeyCode::Up => oscillator.amplitude += 1.0,
                    KeyCode::Down => oscillator.amplitude -= 1.0,
                    KeyCode::Right => oscillator.frequency += 0.01,
                    KeyCode::Left => oscillator.frequency -= 0.01,
                    KeyCode::PageUp => oscillator.speed += 0.5,
                    KeyCode::PageDown => oscillator.speed -= 0.5,
                    _ => {}
                }
            }
        }

        // Update Physics
        let now = Instant::now();
        let dt = now.duration_since(last_tick).as_secs_f64();
        if dt >= tick_rate.as_secs_f64() {
            oscillator.update(dt);
            last_tick = now;
        }

        // Modulate Points
        let modulated_points = oscillator.modulate(&base_points);

        // Draw
        tui.terminal.draw(|f| {
            ui::draw(f, &modulated_points);
        })?;

        // Sleep a bit to prevent 100% CPU usage if polling was fast
        std::thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}
