use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use locus::Topology;
use market_sim::{Grid as MarketGrid, Particle};
use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct LocusMarketApp {
    market: MarketGrid,
    width: usize,
    height: usize,
    topology: Topology,
    flashes: Vec<(usize, usize, f64)>, // x, y, age
}

impl LocusMarketApp {
    fn new(width: usize, height: usize) -> Self {
        Self {
            market: MarketGrid::new(width, height),
            width,
            height,
            topology: Topology::Torus,
            flashes: Vec::new(),
        }
    }

    fn tick(&mut self) {
        let mut rng = rand::thread_rng();

        // Spawn bids and asks randomly
        for _ in 0..5 {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);

            if rng.gen_bool(0.5) {
                if let Particle::Empty = self.market.get(x, y) {
                    self.market.set(x, y, Particle::Bid(rng.gen_range(0..100)));
                }
            } else {
                if let Particle::Empty = self.market.get(x, y) {
                    self.market.set(x, y, Particle::Ask(rng.gen_range(0..100)));
                }
            }
        }

        // We use the market simulation update, but we wrap the grid bounds using Topology
        let events = self.market.update();

        // For topological wrapping, market-sim natively moves them up/down. We will manually apply wrapping here.
        // Bids move "up" (decrease y), Asks move "down" (increase y). If they hit boundary, we wrap them.
        for x in 0..self.width {
            for y in 0..self.height {
                let particle = self.market.get(x, y);
                match particle {
                    Particle::Bid(id) if y == 0 => {
                        // Reached top, wrap to bottom
                        if let Some((ny, nx)) = self.topology.normalize(-1, x as i64, self.width, self.height) {
                            if let Particle::Empty = self.market.get(nx, ny) {
                                self.market.set(nx, ny, Particle::Bid(id));
                                self.market.set(x, y, Particle::Empty);
                            }
                        }
                    }
                    Particle::Ask(id) if y == self.height - 1 => {
                        // Reached bottom, wrap to top
                        if let Some((ny, nx)) = self.topology.normalize(self.height as i64, x as i64, self.width, self.height) {
                            if let Particle::Empty = self.market.get(nx, ny) {
                                self.market.set(nx, ny, Particle::Ask(id));
                                self.market.set(x, y, Particle::Empty);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        // Register trades as flashes
        for event in events {
            // event.price translates to the Y coordinate (height - 1 - price)
            let y = self.height.saturating_sub(1).saturating_sub(event.price as usize);
            let x = rng.gen_range(0..self.width); // We just flash randomly on X for visual effect since X is lost in TradeEvent
            self.flashes.push((x, y, 1.0));
        }

        for (_, _, age) in &mut self.flashes {
            *age -= 0.1;
        }
        self.flashes.retain(|(_, _, age)| *age > 0.0);
    }
}

fn main() -> std::io::Result<()> {
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

fn run_app(tui: &mut Tui) -> std::io::Result<()> {
    let width = 60;
    let height = 30;
    let mut app = LocusMarketApp::new(width, height);

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(100);

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
                        .title(format!(" Locus Market - Topology: {:?} ", app.topology)),
                )
                .x_bounds([0.0, width as f64])
                .y_bounds([0.0, height as f64])
                .paint(|ctx| {
                    for y in 0..app.height {
                        for x in 0..app.width {
                            let particle = app.market.get(x, y);
                            let rx = x as f64;
                            let ry = (app.height - 1 - y) as f64; // Invert Y for canvas

                            match particle {
                                Particle::Bid(_) => {
                                    ctx.print(rx, ry, ratatui::text::Span::styled("▲", Style::default().fg(Color::Green)));
                                }
                                Particle::Ask(_) => {
                                    ctx.print(rx, ry, ratatui::text::Span::styled("▼", Style::default().fg(Color::Red)));
                                }
                                _ => {}
                            }
                        }
                    }

                    for (x, y, age) in &app.flashes {
                        let rx = *x as f64;
                        let ry = (app.height - 1 - *y) as f64;
                        let color = if *age > 0.5 { Color::Yellow } else { Color::DarkGray };
                        ctx.print(rx, ry, ratatui::text::Span::styled("✸", Style::default().fg(color)));
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let stats = Paragraph::new(format!(
                "Bids: {} | Asks: {} | Trades: {} | [T]oggle Topology | [Q]uit",
                app.market.total_bids, app.market.total_asks, app.market.trade_count
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
                        KeyCode::Char('t') | KeyCode::Char('T') => {
                            app.topology = match app.topology {
                                Topology::Plane => Topology::Torus,
                                Topology::Torus => Topology::Klein,
                                Topology::Klein => Topology::CylinderH,
                                Topology::CylinderH => Topology::Plane,
                                _ => Topology::Plane,
                            };
                        }
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
