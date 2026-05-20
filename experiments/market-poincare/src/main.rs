use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use market_sim::{Grid as MarketGrid, Particle, TradeEvent};
use poincare_disk::{Mobius, Point};
use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct MarketPoincareApp {
    market: MarketGrid,
    width: usize,
    height: usize,
    flashes: Vec<(Point, f64)>, // Trades map to hyperbolic flashes (point, intensity/age)
    transform: Mobius,
}

impl MarketPoincareApp {
    fn new(width: usize, height: usize) -> Self {
        Self {
            market: MarketGrid::new(width, height),
            width,
            height,
            flashes: Vec::new(),
            transform: Mobius::rotation(0.0),
        }
    }

    fn tick(&mut self) {
        let mut rng = rand::thread_rng();

        // Randomly spawn bids and asks in the market
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

        let events = self.market.update();

        // Map trades into Poincaré space
        for event in events {
            // Reconstruct the (x, y) coordinates of the trade
            // The price is (height - 1 - y), so y = height - 1 - price
            let y = self.height as f32 - 1.0 - event.price;
            let normalized_y = (y / self.height as f32) * 2.0 - 1.0;

            // We'll guess random X for visual flair, since we lost X in TradeEvent.
            // Actually, we can just distribute it across X randomly, or along an arc.
            let normalized_x = rng.gen_range(-0.8..0.8);

            let p = Point::new(normalized_x as f64, normalized_y as f64);
            // Must scale to keep it inside the disk
            let p = Point::new(p.re * 0.8, p.im * 0.8);

            self.flashes.push((p, 1.0));
        }

        // Apply a slow hyperbolic rotation to the flashes
        let rot = Mobius::rotation(0.02);
        self.transform = self.transform.then(&rot);

        for (point, age) in &mut self.flashes {
            *point = rot.apply(*point);
            *age -= 0.05;
        }

        self.flashes.retain(|(_, age)| *age > 0.0);
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
    let mut app = MarketPoincareApp::new(width, height);

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
                        .title(" Market Poincaré - Hyperbolic Order Book "),
                )
                .x_bounds([-1.1, 1.1])
                .y_bounds([-1.1, 1.1])
                .paint(|ctx| {
                    // Draw Disk boundary
                    for i in 0..100 {
                        let angle = (i as f64) / 100.0 * 2.0 * std::f64::consts::PI;
                        ctx.print(
                            angle.cos(),
                            angle.sin(),
                            ratatui::text::Span::styled(".", Style::default().fg(Color::DarkGray)),
                        );
                    }

                    // Draw the market grid (Bids and Asks) projected into the disk
                    for y in 0..app.height {
                        for x in 0..app.width {
                            let particle = app.market.get(x, y);
                            let normalized_x = (x as f64 / app.width as f64) * 2.0 - 1.0;
                            let normalized_y = (y as f64 / app.height as f64) * 2.0 - 1.0;

                            // Map the Euclidean grid point using the current Mobius transform
                            let mut p = Point::new(normalized_x, normalized_y);

                            // Shrink it so it's strictly inside the disk
                            p = Point::new(p.re * 0.9, p.im * 0.9);
                            p = app.transform.apply(p);

                            match particle {
                                Particle::Bid(_) => {
                                    ctx.print(
                                        p.re,
                                        p.im,
                                        ratatui::text::Span::styled(
                                            "▲",
                                            Style::default().fg(Color::Green),
                                        ),
                                    );
                                }
                                Particle::Ask(_) => {
                                    ctx.print(
                                        p.re,
                                        p.im,
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

                    // Draw the Trade flashes
                    for (p, age) in &app.flashes {
                        let color = if *age > 0.7 {
                            Color::Yellow
                        } else if *age > 0.3 {
                            Color::Cyan
                        } else {
                            Color::DarkGray
                        };
                        ctx.print(
                            p.re,
                            p.im,
                            ratatui::text::Span::styled("✸", Style::default().fg(color)),
                        );
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let stats = Paragraph::new(format!(
                "Bids: {} | Asks: {} | Trades: {} | Active Flashes: {} | [Q] Quit",
                app.market.total_bids,
                app.market.total_asks,
                app.market.trade_count,
                app.flashes.len(),
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
