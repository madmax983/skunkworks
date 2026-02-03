pub mod allocator;
pub mod painter;
pub mod ui;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::{Duration, Instant};
use tui_shared::Tui;
use crate::painter::Painter;

fn main() -> Result<()> {
    // Initialize TUI
    let mut tui = Tui::init()?;

    // Initialize Painter with a reasonable canvas size
    // 80x40 covers most screens and provides good resolution
    let mut painter = Painter::new(80, 40);

    run_app(&mut tui, &mut painter)?;

    Ok(())
}

fn run_app(tui: &mut Tui, painter: &mut Painter) -> Result<()> {
    // 30 FPS for smooth painting
    let tick_rate = Duration::from_millis(33);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui::draw(f, painter))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char(' ') => painter.switch_mode(),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            painter.tick();
            last_tick = Instant::now();
        }
    }
    Ok(())
}
