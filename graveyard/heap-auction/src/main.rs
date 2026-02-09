use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use heap_auction::model::System;
use std::time::{Duration, Instant};
use tui_shared::Tui;

mod tui;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    // System size: 1000 blocks, 50 agents
    let mut system = System::new(1000, 50);
    let mut paused = false;
    let tick_rate = Duration::from_millis(50); // 20 TPS
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| {
            tui::draw(f, &system);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char(' ') => paused = !paused,
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            if !paused {
                system.tick();
            }
            last_tick = Instant::now();
        }
    }

    Ok(())
}
