//! # market-locus
//! **Parents**: `market-sim` + `locus`
//!
//! Topological Market Liquidity. An order book where trades wrap around topological bounds.
//!
//! **Lineage Plan**:
//! - From market-sim: Discrete financial order book grid, Bid/Ask particle physics, and execution logic.
//! - From locus: Topological spaces (Torus, Klein Bottle, Sphere, Projective Plane) and Vec2 coordinates.
//! - Novel trait: Continuous loop visualization where localized market price spikes and volatility wrap around to instantly affect opposite financial boundaries.
//!
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use locus::Topology;
use market_sim::{Grid, Particle};
use rand::Rng;
use ratatui::{
    style::{Color, Style},
    text::Span,
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct MarketLocusApp {
    grid: Grid,
    topology: Topology,
}

impl MarketLocusApp {
    fn new(width: usize, height: usize, topology: Topology) -> Self {
        Self {
            grid: Grid::new(width, height),
            topology,
        }
    }

    fn tick(&mut self) {
        let mut rng = rand::thread_rng();
        let w = self.grid.width;
        let h = self.grid.height;

        for _ in 0..5 {
            let x = rng.gen_range(0..w);
            if matches!(self.grid.get(x, h - 1), Particle::Empty) {
                self.grid.set(x, h - 1, Particle::Bid(rng.gen()));
            }
        }
        for _ in 0..5 {
            let x = rng.gen_range(0..w);
            if matches!(self.grid.get(x, 0), Particle::Empty) {
                self.grid.set(x, 0, Particle::Ask(rng.gen()));
            }
        }

        let _ = self.grid.update();

        let mut changes = Vec::new();
        for x in 0..w {
            if let Particle::Bid(id) = self.grid.get(x, 0) {
                if let Some((ny, nx)) = self.topology.normalize(-1, x as i64, w, h) {
                    changes.push((x, 0, nx, ny, Particle::Bid(id)));
                }
            }
            if let Particle::Ask(id) = self.grid.get(x, h - 1) {
                if let Some((ny, nx)) = self.topology.normalize(h as i64, x as i64, w, h) {
                    changes.push((x, h - 1, nx, ny, Particle::Ask(id)));
                }
            }
        }

        for (ox, oy, nx, ny, particle) in changes {
            if matches!(self.grid.get(nx, ny), Particle::Empty) {
                self.grid.set(nx, ny, particle);
                self.grid.set(ox, oy, Particle::Empty);
            }
        }
    }

    fn draw(&self, frame: &mut ratatui::Frame) {
        use ratatui::layout::{Constraint, Direction, Layout};
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(frame.area());

        let title = format!(
            " 🧬 Splice: market-sim × locus | Topology: {:?} | [Tab] Change Topology | [Q] Quit ",
            self.topology
        );

        let canvas = Canvas::default()
            .block(Block::default().title(title).borders(Borders::ALL))
            .paint(|ctx| {
                for y in 0..self.grid.height {
                    for x in 0..self.grid.width {
                        let particle = self.grid.get(x, y);
                        let cx = (x as f64 / self.grid.width as f64) * 100.0;
                        let cy = (1.0 - (y as f64 / self.grid.height as f64)) * 100.0;

                        match particle {
                            Particle::Bid(_) => {
                                ctx.print(
                                    cx,
                                    cy,
                                    Span::styled("▲", Style::default().fg(Color::Green)),
                                );
                            }
                            Particle::Ask(_) => {
                                ctx.print(
                                    cx,
                                    cy,
                                    Span::styled("▼", Style::default().fg(Color::Red)),
                                );
                            }
                            Particle::Trade { .. } => {
                                ctx.print(
                                    cx,
                                    cy,
                                    Span::styled(
                                        "X",
                                        Style::default().fg(Color::Yellow).bg(Color::White),
                                    ),
                                );
                            }
                            _ => {}
                        }
                    }
                }
            })
            .x_bounds([0.0, 100.0])
            .y_bounds([0.0, 100.0]);

        frame.render_widget(canvas, chunks[0]);

        let stats = Paragraph::new(format!(
            "Trades: {} | Bids: {} | Asks: {}",
            self.grid.trade_count, self.grid.total_bids, self.grid.total_asks
        ))
        .block(Block::default().borders(Borders::ALL));
        frame.render_widget(stats, chunks[1]);
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let headless = args.iter().any(|arg| arg == "--headless");

    let mut tui = if headless { None } else { Some(Tui::init()?) };
    let mut app = MarketLocusApp::new(100, 40, Topology::Torus);

    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();
    let mut frame_count = 0;

    loop {
        if headless {
            app.tick();
            frame_count += 1;
            if frame_count > 50 {
                println!("🧬 market-locus running in headless mode for 50 ticks.");
                break;
            }
            continue;
        }

        if let Some(tui) = &mut tui {
            tui.terminal.draw(|f| app.draw(f))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press
                        && (key.code == KeyCode::Char('q') || key.code == KeyCode::Esc)
                    {
                        break;
                    }
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Tab {
                        app.topology = match app.topology {
                            Topology::Plane => Topology::Torus,
                            Topology::Torus => Topology::Klein,
                            Topology::Klein => Topology::CylinderH,
                            Topology::CylinderH => Topology::CylinderV,
                            Topology::CylinderV => Topology::Mobius,
                            Topology::Mobius => Topology::Sphere,
                            Topology::Sphere => Topology::Projective,
                            Topology::Projective => Topology::Plane,
                            _ => Topology::Plane,
                        };
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                app.tick();
                last_tick = Instant::now();
            }
        }
    }

    Ok(())
}
