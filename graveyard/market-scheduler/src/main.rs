mod agent;
mod market;
mod simulation;
mod tui;
mod types;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use simulation::Simulation;
use std::time::{Duration, Instant};
use tui_shared::Tui;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut sim = Simulation::new(20);

    // Config
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();
    let mut paused = false;

    loop {
        tui.terminal.draw(|f| tui::draw(f, &sim))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char(' ') => paused = !paused,
                    KeyCode::Char('r') => sim = Simulation::new(20),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            if !paused {
                sim.update();
            }
            last_tick = Instant::now();
        }
    }

    Ok(())
}
