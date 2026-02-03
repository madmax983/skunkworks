mod game;
mod ui;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::{Duration, Instant};
use tui_shared::Tui;

use crate::game::Game;
use crate::ui::ui;

fn main() -> Result<()> {
    // 1. Setup TUI
    let mut tui = Tui::init()?;

    // 2. Setup Game
    // Use terminal size for game world
    let size = tui.terminal.size()?;
    let width = size.width as f64;
    let height = size.height.saturating_sub(4) as f64; // Reserve 4 lines for UI

    let mut game = Game::new(width, height);

    let tick_rate = Duration::from_millis(30);
    let mut last_tick = Instant::now();

    loop {
        // 3. Draw
        tui.terminal.draw(|f| {
            ui(f, &game);
        })?;

        // 4. Input
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Enter => game.submit_regex(),
                    KeyCode::Backspace => game.input_backspace(),
                        KeyCode::Char(c) => {
                            if game.game_over {
                                if c == 'r' || c == 'R' {
                                    game = Game::new(width, height);
                                }
                            } else {
                                game.input_char(c);
                            }
                        }
                    _ => {}
                },
                Event::Resize(w, h) => {
                    game.width = w as f64;
                    game.height = h.saturating_sub(4) as f64;
                }
                _ => {}
            }
        }

        // 5. Update
        if last_tick.elapsed() >= tick_rate {
            let dt = last_tick.elapsed().as_secs_f64();
            game.update(dt);
            last_tick = Instant::now();
        }
    }

    Ok(())
}
