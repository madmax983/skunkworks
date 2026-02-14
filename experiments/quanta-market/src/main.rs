pub mod model;
pub mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::{Duration, Instant}};

use crate::model::Scheduler;

fn main() -> Result<()> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create App
    let mut scheduler = Scheduler::new();

    // Initial Spawn
    for _ in 0..5 {
        scheduler.spawn_process();
    }

    let res = run_app(&mut terminal, &mut scheduler);

    // Restore Terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, scheduler: &mut Scheduler) -> Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(100);
    let mut paused = false;

    loop {
        terminal.draw(|f| ui::draw_ui(f, scheduler)).map_err(|e| anyhow::anyhow!("{}", e))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char(' ') => paused = !paused,
                    KeyCode::Char('r') => {
                        *scheduler = Scheduler::new();
                        for _ in 0..5 { scheduler.spawn_process(); }
                    }
                    KeyCode::Char('+') => scheduler.spawn_process(),
                    KeyCode::Char('-') => scheduler.kill_process(),
                    KeyCode::Char('b') => {
                        for _ in 0..10 { scheduler.spawn_process(); }
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            if !paused {
                scheduler.tick();
            }
            last_tick = Instant::now();
        }
    }
}
