use anyhow::Result;
use clap::Parser;
use crossterm::event::{self, Event, KeyCode};
use std::time::{Duration, Instant};
use tui_shared::Tui;

mod market;
mod tui;

use market::Market;
use tui as ui;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of ticks to run (optional)
    #[arg(short, long)]
    ticks: Option<usize>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Setup Terminal
    let mut tui = Tui::init()?;

    // App State
    let mut market = Market::new(1024, 20); // 1KB heap, 20 agents
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(50); // Fast simulation

    loop {
        tui.terminal.draw(|f| ui::draw_ui(f, &market))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            market.tick();
            last_tick = Instant::now();

            if let Some(limit) = args.ticks {
                if market.tick_count >= limit {
                    break;
                }
            }
        }
    }

    Ok(())
}
