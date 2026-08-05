//! # 🧬 Market Gray (market-gray)
//!
//! **Lineage:** `crates/market-sim` × `crates/gray-scott`
//!
//! ## Concept: Morphogenetic Financial Liquidity
//!
//! This experiment projects a discrete Continuous Double Auction (CDA) market grid into a continuous Gray-Scott reaction-diffusion substrate. Bids and asks act as active biological sources feeding the grid, while executed trades act as intense "kill" chemical drops.
//!
//! ### Hybrid Vigor
//! - **Market Dynamics (`market-sim`)**: Bids and asks flow across a 2D market grid, attempting to fulfill orders and providing market liquidity.
//! - **Reaction-Diffusion (`gray-scott`)**: The chemical substrate allows us to view the "organic footprint" of the market. Rather than discrete data points, the market generates self-sustaining Turing patterns driven entirely by financial events.
//!
//! ## Execution
//!
//! ```bash
//! cargo run -p market-gray
//! ```
//!
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use gray_scott::GrayScott;
use market_sim::{Grid as MarketGrid, Particle};
use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct MarketGrayApp {
    market: MarketGrid,
    gray_scott: GrayScott,
    width: usize,
    height: usize,
}

impl MarketGrayApp {
    fn new(width: usize, height: usize) -> Self {
        // Initialize Gray-Scott with a high kill rate to ensure it quickly resets
        // to empty state without continuous stimulation
        let mut gs = GrayScott::new(width, height);
        gs.diff_u = 1.0;
        gs.diff_v = 0.5;

        Self {
            market: MarketGrid::new(width, height),
            gray_scott: gs,
            width,
            height,
        }
    }

    fn tick(&mut self) {
        let mut rng = rand::thread_rng();

        // 1. Spawn market orders
        for _ in 0..10 {
            let x = rng.gen_range(0..self.width);
            if rng.gen_bool(0.5) {
                // Bid spawns at the bottom
                self.market
                    .set(x, self.height - 1, Particle::Bid(rng.gen_range(0..100)));
            } else {
                // Ask spawns at the top
                self.market.set(x, 0, Particle::Ask(rng.gen_range(0..100)));
            }
        }

        // 2. Update the market
        let events = self.market.update();

        // 3. Map the market directly to Gray-Scott
        // Bids and Asks act as feed (U) chemical, "keeping the grid alive"
        let mut drops = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if let Particle::Bid(_) | Particle::Ask(_) = self.market.get(x, y) {
                    drops.push((x, y));
                }
            }
        }

        // We simulate the global market providing chemical feed
        self.gray_scott.update(0.055, 0.062, 0.5);

        // 4. Trades drop the Kill (V) chemical
        for event in events {
            // Drop heavy V chemical at the trade site
            // Since we don't have X directly on TradeEvent, we pick a random X
            // across the width for the liquidity splash
            let x = rng.gen_range(0..self.width);
            let y = self.height - 1 - event.price as usize;
            self.gray_scott.add_chemical(x, y, 1.0);
        }

        // Add additional feed based on market particles
        for (x, y) in drops {
            // We slightly feed the market participants
            self.gray_scott.add_chemical(x, y, 0.01);
        }

        self.gray_scott.update(0.0367, 0.0649, 1.0);
    }
}

fn main() -> Result<()> {
    // Implement standard headless guard for workspace tests
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Headless mode: exiting early to prevent X11 panics or execution timeouts.");
        return Ok(());
    }

    let mut tui = Tui::init()?;
    let res = run_app(&mut tui);
    tui.exit()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app(tui: &mut Tui) -> Result<()> {
    let width = 50;
    let height = 50;
    let mut app = MarketGrayApp::new(width, height);

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(50);

    loop {
        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.area());

            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Morphogenetic Financial Liquidity "),
                )
                .x_bounds([0.0, width as f64])
                .y_bounds([0.0, height as f64])
                .paint(|ctx| {
                    for y in 0..app.height {
                        for x in 0..app.width {
                            let idx = app.gray_scott.get_index(x, y);
                            let v = app.gray_scott.v()[idx];

                            // Map Turing patterns to color
                            let intensity = (v * 255.0).clamp(0.0, 255.0) as u8;

                            if intensity > 20 {
                                let color = Color::Rgb(intensity, 50, 255 - intensity);
                                ctx.print(
                                    x as f64,
                                    (app.height - 1 - y) as f64, // Flip Y for terminal rendering
                                    ratatui::text::Span::styled("█", Style::default().fg(color)),
                                );
                            }

                            // Overlay actual market particles
                            match app.market.get(x, y) {
                                Particle::Bid(_) => {
                                    ctx.print(
                                        x as f64,
                                        (app.height - 1 - y) as f64,
                                        ratatui::text::Span::styled(
                                            "▲",
                                            Style::default().fg(Color::Green),
                                        ),
                                    );
                                }
                                Particle::Ask(_) => {
                                    ctx.print(
                                        x as f64,
                                        (app.height - 1 - y) as f64,
                                        ratatui::text::Span::styled(
                                            "▼",
                                            Style::default().fg(Color::Red),
                                        ),
                                    );
                                }
                                _ => {}
                            }
                        }
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let stats = Paragraph::new(format!(
                "Bids: {} | Asks: {} | Trades: {} | [Q] Quit",
                app.market.total_bids, app.market.total_asks, app.market.trade_count,
            ))
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(stats, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }
    }
}
