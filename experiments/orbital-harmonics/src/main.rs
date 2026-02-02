use anyhow::Result;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{
    io,
    time::{Duration, Instant},
};
use tui_shared::Tui;

pub mod app;
pub mod audio;
pub mod physics;

use app::App;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    // Create App
    let mut app = App::new();

    // Run Loop
    let res = run_app(&mut tui.terminal, &mut app);

    // Cleanup handled by Tui drop
    drop(tui);

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(16); // ~60 FPS for smooth animation

    loop {
        terminal.draw(|f| app.draw(f))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if app.handle_input(key) {
                        return Ok(());
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}
