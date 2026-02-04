mod diff;
mod game;
mod ui;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use game::Game;
use std::time::{Duration, Instant};
use tui_shared::Tui;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    let size = tui.terminal.size()?;
    // Reserve 2 lines for borders/footer in calculation?
    // Canvas takes full area provided. UI uses Layout.
    // Let's rely on UI passing the area?
    // But Game logic needs world bounds.
    // Let's set world bounds to terminal size for now.
    // Note: Canvas rendering scales coordinates to the area it's drawn in?
    // No, ratatui Canvas uses specific x_bounds/y_bounds.
    // If I draw at x=10, and bounds are 0..100, it draws at 10% of width.
    // So "Screen Coordinates" approach works if bounds match pixel/cell count.

    let width = size.width as f64;
    let height = size.height.saturating_sub(1) as f64; // Reserve footer

    let mut game = Game::new(width, height)?;

    let tick_rate = Duration::from_millis(16); // ~60 FPS
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui::draw(f, &game))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Left | KeyCode::Char('a') => game.move_player(-2.0),
                        KeyCode::Right | KeyCode::Char('d') => game.move_player(2.0),
                        KeyCode::Char(' ') => game.shoot(),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            // Check resize
            if let Ok(size) = tui.terminal.size() {
                let w = size.width as f64;
                let h = size.height.saturating_sub(1) as f64;
                // Only update if significantly changed to avoid jitter?
                if (w - game.width).abs() > 0.1 || (h - game.height).abs() > 0.1 {
                    game.width = w;
                    game.height = h;
                    game.player_x = game.player_x.clamp(0.0, w - 3.0);
                }
            }

            game.tick();
            last_tick = Instant::now();
        }
    }

    Ok(())
}
