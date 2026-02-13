mod suanpan;
mod market;
mod tui;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::{Duration, Instant}};

use market::{Market, Scalper};

fn main() -> Result<()> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup Simulation
    let mut market = Market::new(100);
    let mut scalper = Scalper::new(10000);

    let tick_rate = Duration::from_millis(250); // Speed of market
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            tui::draw(f, &market, &scalper);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('r') => {
                         market = Market::new(100);
                         scalper = Scalper::new(10000);
                    },
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            market.tick();
            scalper.update(&market.price);
            let _action = scalper.trade(&market.price);
            // Log action? TUI doesn't show logs yet properly (just stats)
            // But stats show position changing.
            last_tick = Instant::now();
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
