//! # quipu-market
//!
//! **Parents**: `crates/market-sim` + `crates/quipu`
//! **Concept**: Physical Knotted Market Ledger
//!
//! ## Traits
//! * **Lineage from `market-sim`**: Discrete Euclidean order book grid where Bids and Asks collide to generate trades.
//! * **Lineage from `quipu`**: Discrete structural memory architecture that encodes data dynamically as permanent tied knots along a cord.
//! * **Phenotype**: This hybrid turns abstract, ephemeral market liquidity into a permanent, physical artifact. Every time a trade executes in the order book, a new knotted record of that trade's price is structurally tied onto a Quipu cord. The Quipu cord visualizes the permanent sedimentation of financial activity.
//!
//! ## Quick Start
//!
//! You can run the interactive simulation using Cargo:
//!
//! ```bash
//! cargo run -p quipu-market
//! ```
//!
//! *Note: For automated environments, append the `--headless` flag to bypass the interactive terminal UI.*
//!
//! ### Visual Example
//!
//! When running the TUI, you will observe real-time trades translating into knots tied into the history cord:
//!
//! ```text
//! ▲    .   ▼ .    ▲▲ ▼   ▲    .
//!   .  . ▲ ▲      .  ▲ .    .
//! . .  .   .    ▼  ✸  ▲  .  ▲
//!  .    ▲ .   ▲ .  ▲ ▲ ✸
//!
//! [ Cord 0 ] => 15 knots:  . ▲ ▲ ... . . . ▼ ▼ ▲
//! ```
//!
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use market_sim::{Grid as MarketGrid, Particle};
use quipu::{Cord, Quipu};
use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct QuipuMarketApp {
    market: MarketGrid,
    quipu: Quipu,
    width: usize,
    height: usize,
}

impl QuipuMarketApp {
    fn new(width: usize, height: usize) -> Self {
        let mut quipu = Quipu::new();
        // Start with an empty cord for our ledger
        quipu.add_cord(Cord::from(0u64));
        Self {
            market: MarketGrid::new(width, height),
            quipu,
            width,
            height,
        }
    }

    fn tick(&mut self) {
        let mut rng = rand::thread_rng();

        // Spawn bids and asks
        for _ in 0..8 {
            let x = rng.gen_range(0..self.width);
            if rng.gen_bool(0.5) {
                self.market
                    .set(x, self.height - 1, Particle::Bid(rng.gen_range(1..100)));
            } else {
                self.market.set(x, 0, Particle::Ask(rng.gen_range(1..100)));
            }
        }

        let events = self.market.update();

        // Each trade gets tied as a knot on the Quipu cord
        for event in events {
            let price_value = event.price as usize;

            // To simulate building a ledger, we append a new cord for every N trades,
            // or just grow the value of the main cord. Let's grow the main cord value
            // and occasionally add new cords if the current one gets too big.

            // In quipu, Cord::from(val) creates base-10 knot clusters.
            // Let's add a new cord for each trade to act as an append-only ledger list.
            if self.quipu.cords.len() > 10 {
                // Keep the ledger bounded to the last 10 trades visually
                self.quipu.cords.remove(0);
            }
            self.quipu.add_cord(Cord::from(price_value as u64));
        }
    }
}

fn main() -> Result<()> {
    if std::env::args().any(|arg| arg == "--headless") {
        println!("Headless mode: exiting early to prevent CI timeouts.");
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
    let width = 40;
    let height = 20;
    let mut app = QuipuMarketApp::new(width, height);

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(100);

    loop {
        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(f.area());

            // Left panel: The Market Simulation
            let market_block = Block::default()
                .borders(Borders::ALL)
                .title(" Market Sim (Bids & Asks) ");
            f.render_widget(market_block, chunks[0]);

            // Render market cells manually within the block
            let inner_market = Rect::new(
                chunks[0].x + 1,
                chunks[0].y + 1,
                chunks[0].width - 2,
                chunks[0].height - 2,
            );
            for y in 0..app.height {
                for x in 0..app.width {
                    let screen_x = inner_market.x + x as u16;
                    let screen_y = inner_market.y + y as u16;

                    if screen_x < inner_market.right() && screen_y < inner_market.bottom() {
                        let particle = app.market.get(x, y);
                        match particle {
                            Particle::Bid(_) => {
                                let widget = Paragraph::new(Span::styled(
                                    "▲",
                                    Style::default().fg(Color::Green),
                                ));
                                f.render_widget(widget, Rect::new(screen_x, screen_y, 1, 1));
                            }
                            Particle::Ask(_) => {
                                let widget = Paragraph::new(Span::styled(
                                    "▼",
                                    Style::default().fg(Color::Red),
                                ));
                                f.render_widget(widget, Rect::new(screen_x, screen_y, 1, 1));
                            }
                            Particle::Trade { age } => {
                                let color = if age > 5 {
                                    Color::Yellow
                                } else {
                                    Color::DarkGray
                                };
                                let widget =
                                    Paragraph::new(Span::styled("✸", Style::default().fg(color)));
                                f.render_widget(widget, Rect::new(screen_x, screen_y, 1, 1));
                            }
                            Particle::Empty => {
                                let widget = Paragraph::new(Span::styled(
                                    ".",
                                    Style::default().fg(Color::Rgb(30, 30, 30)),
                                ));
                                f.render_widget(widget, Rect::new(screen_x, screen_y, 1, 1));
                            }
                            Particle::Wall => {}
                        }
                    }
                }
            }

            // Right panel: The Quipu Ledger
            let quipu_block = Block::default()
                .borders(Borders::ALL)
                .title(" Quipu Ledger (Knotted Trades) ");
            f.render_widget(quipu_block.clone(), chunks[1]);

            let inner_quipu = Rect::new(
                chunks[1].x + 1,
                chunks[1].y + 1,
                chunks[1].width - 2,
                chunks[1].height - 2,
            );

            let mut cursor_y = inner_quipu.y;
            for (i, cord) in app.quipu.cords.iter().enumerate() {
                if i == 0 && app.quipu.cords.len() > 1 {
                    continue; // Skip the initial dummy cord if we have real trades
                }

                let mut cord_str = String::new();
                for cluster in &cord.clusters {
                    for knot in cluster {
                        cord_str.push_str(&knot.symbol());
                    }
                    cord_str.push('-');
                }

                let price_val = cord.value();
                let display = format!("Trade {}: [{}] => {} knots", i, cord_str, price_val);

                if cursor_y < inner_quipu.bottom() {
                    let widget =
                        Paragraph::new(Span::styled(display, Style::default().fg(Color::Yellow)));
                    f.render_widget(
                        widget,
                        Rect::new(inner_quipu.x, cursor_y, inner_quipu.width, 1),
                    );
                    cursor_y += 2;
                }
            }
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
