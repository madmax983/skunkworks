use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::Terminal;
use std::io;
use std::time::{Duration, Instant};
use tui_shared::Tui;

mod app;
mod eye;
mod retina;

use app::App;

fn main() -> Result<()> {
    // Setup terminal
    let mut tui = Tui::init()?;

    // Create app with content
    let content = include_str!("main.rs").to_string();

    // Get terminal size for retina size
    let size = tui.terminal.size()?;
    let mut app = App::new(size.width as usize, size.height as usize, content);

    // Run loop
    let res = run_app(&mut tui.terminal, &mut app);

    // Restore terminal
    drop(tui);

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    let tick_rate = Duration::from_millis(32);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| app.draw(f))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => app.should_quit = true,
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
