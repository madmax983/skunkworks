mod app;
mod fs;
mod math;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{
    io,
    time::{Duration, Instant},
};

use crate::app::App;

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new()?;

    // Run loop
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    let tick_rate = Duration::from_millis(32); // ~30 FPS
    let mut last_tick = Instant::now();

    while app.running {
        terminal.draw(|f| ui::ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            #[allow(clippy::collapsible_if)]
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => app.running = false,
                    KeyCode::Char('w') => app.rotate(0.05, 0.0),
                    KeyCode::Char('s') => app.rotate(-0.05, 0.0),
                    KeyCode::Char('a') => app.rotate(0.0, 0.05),
                    KeyCode::Char('d') => app.rotate(0.0, -0.05),
                    KeyCode::Char('+') | KeyCode::Char('=') => app.zoom *= 1.1,
                    KeyCode::Char('-') | KeyCode::Char('_') => app.zoom *= 0.9,
                    KeyCode::Char(' ') => app.auto_rotate = !app.auto_rotate,
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }

    Ok(())
}
