//! # Market Gray 📈 -> 🦠
//!
//! **Concept:** Morphogenetic Financial Pressure.
//!
//! This hybrid crosses the continuous grid-based physical order book simulation of `market-sim`
//! with the continuous chemical reaction-diffusion substrate of `gray-scott`.
//!
//! ## Lineage
//! - **Parent A (crates/market-sim):** Provides the physical Continuous Double Auction mechanics where Bids and Asks bubble and fall, colliding to form trades.
//! - **Parent B (crates/gray-scott):** Provides the thermodynamic scalar field (U and V morphogens) that generates Turing patterns.
//!
//! ## Novel Trait
//! Physical market liquidity particles (Bids/Asks) act as literal chemical catalysts. Whenever a Bid or Ask exists in a grid cell, it injects the `V` chemical into the morphogenetic substrate at that location.
//!
//! ## Predicted Phenotype
//! An emergent visual laboratory where abstract financial pressure (order book density) explicitly drives and constrains biological morphogenetic growth patterns.
//!
//! ## Usage
//!
//! ```sh
//! cargo run -p market-gray --release
//! ```
//!
//! *(Note: Use `cargo run -p market-gray --release -- --headless` to safely bypass UI panics in CI environments.)*

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use gray_scott::GrayScott;
use market_sim::{Grid, Particle};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct MarketGrayApp {
    market: Grid,
    dish: GrayScott,
    width: usize,
    height: usize,
}

impl MarketGrayApp {
    fn new(width: usize, height: usize) -> Self {
        let market = Grid::new(width, height);
        let mut dish = GrayScott::new(width, height);

        // Seed some initial chemicals
        dish.add_chemical(width / 2, height / 2, 1.0);
        dish.add_chemical(width / 2 + 1, height / 2, 1.0);
        dish.add_chemical(width / 2, height / 2 + 1, 1.0);
        dish.add_chemical(width / 2 + 1, height / 2 + 1, 1.0);

        Self {
            market,
            dish,
            width,
            height,
        }
    }

    fn tick(&mut self) {
        // Randomly inject some market noise (liquidity)
        let mut rng = rand::thread_rng();
        use rand::Rng;

        if rng.gen_bool(0.3) {
            self.market.set(
                rng.gen_range(0..self.width),
                self.height - 1,
                Particle::Bid(rng.gen()),
            );
        }
        if rng.gen_bool(0.3) {
            self.market
                .set(rng.gen_range(0..self.width), 0, Particle::Ask(rng.gen()));
        }

        self.market.update();

        // Inject V chemical where liquidity exists
        for y in 0..self.height {
            for x in 0..self.width {
                match self.market.get(x, y) {
                    Particle::Bid(_) | Particle::Ask(_) | Particle::Trade { .. } => {
                        self.dish.add_chemical(x, y, 0.5);
                    }
                    _ => {}
                }
            }
        }

        // Run Gray-Scott morphogenetic step
        let feed = 0.055;
        let kill = 0.062;
        self.dish.update(feed, kill, 1.0);
    }
}

fn main() -> Result<()> {
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
                        .title(" Market-Gray: Morphogenetic Finance "),
                )
                .x_bounds([0.0, app.width as f64])
                .y_bounds([0.0, app.height as f64])
                .paint(|ctx| {
                    let v_buf = app.dish.v();
                    for y in 0..app.height {
                        for x in 0..app.width {
                            let draw_x = x as f64;
                            let draw_y = (app.height - 1 - y) as f64; // Invert Y for canvas drawing

                            // Draw Gray-Scott V chemical
                            let v = v_buf[y * app.width + x];
                            if v > 0.2 {
                                let color = if v > 0.5 { Color::Cyan } else { Color::Blue };
                                ctx.print(
                                    draw_x,
                                    draw_y,
                                    ratatui::text::Span::styled("•", Style::default().fg(color)),
                                );
                            }

                            // Overlay Market Particles
                            match app.market.get(x, y) {
                                Particle::Bid(_) => {
                                    ctx.print(
                                        draw_x,
                                        draw_y,
                                        ratatui::text::Span::styled(
                                            "▲",
                                            Style::default().fg(Color::Green),
                                        ),
                                    );
                                }
                                Particle::Ask(_) => {
                                    ctx.print(
                                        draw_x,
                                        draw_y,
                                        ratatui::text::Span::styled(
                                            "▼",
                                            Style::default().fg(Color::Red),
                                        ),
                                    );
                                }
                                Particle::Trade { .. } => {
                                    ctx.print(
                                        draw_x,
                                        draw_y,
                                        ratatui::text::Span::styled(
                                            "◆",
                                            Style::default().fg(Color::Yellow),
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
                "Trades: {} | Bids: {} | Asks: {} | [Q] Quit",
                app.market.trade_count, app.market.total_bids, app.market.total_asks
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
