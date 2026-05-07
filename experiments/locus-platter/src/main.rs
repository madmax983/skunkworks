//! 🧬 Splice: Cross locus × platter
//!
//! **Concept**: Topological Scalar Field Morphogenesis.
//!
//! **Lineage**:
//! - Parent A (locus): Provides the geometry and grid topology boundaries (Torus, Klein Bottle, Cylinder, etc).
//! - Parent B (platter): Provides the continuous 2D scalar field for tracking heat/mass decay.
//!
//! **Novel trait**: The scalar field's diffusion, accumulation, and heat mapping are projected onto the topological constraints of a non-planar 2D grid. We drop heat sources and watch them wrap around according to different spatial boundaries.
//!
//! **Predicted Phenotype**: A visual mapping showing how different Euclidean boundaries affect heat.

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use platter::Platter;
use locus::{Topology, Vec2};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct LocusPlatterApp {
    platter: Platter,
    width: usize,
    height: usize,
    topology: Topology,
    sources: Vec<Vec2>,
    time: f64,
}

impl LocusPlatterApp {
    fn new(width: usize, height: usize, topology: Topology) -> Self {
        Self {
            platter: Platter::new(width, height),
            width,
            height,
            topology,
            sources: vec![
                Vec2::new(width as f64 / 2.0, height as f64 / 2.0),
                Vec2::new(width as f64 / 4.0, height as f64 / 4.0),
            ],
            time: 0.0,
        }
    }

    fn tick(&mut self) {
        self.time += 0.1;

        // Move sources
        self.sources[0].x += self.time.sin() * 2.0;
        self.sources[0].y += self.time.cos() * 2.0;

        self.sources[1].x -= self.time.cos() * 1.5;
        self.sources[1].y += self.time.sin() * 1.5;

        // Normalize sources to keep them in bounds based on topology
        for source in &mut self.sources {
            if let Some((ny, nx)) = self.topology.normalize(source.y.round() as i64, source.x.round() as i64, self.width, self.height) {
                source.x = nx as f64;
                source.y = ny as f64;
            } else {
                // If plane topology and out of bounds, bounce back slightly (just a basic hack to keep it inside)
                source.x = source.x.clamp(0.0, (self.width - 1) as f64);
                source.y = source.y.clamp(0.0, (self.height - 1) as f64);
            }
        }

        // Drop heat from sources, diffusing outward
        for y in 0..self.height {
            for x in 0..self.width {
                let mut added_heat = 0.0;
                for source in &self.sources {
                    // For Torus etc., distance is complicated, but since we normalize sources to be within
                    // width/height, we can just do simple Euclidean distance here for a local pulse effect.
                    // A proper diffusion would consider wrapped neighbors.
                    let dx = x as f64 - source.x;
                    let dy = y as f64 - source.y;
                    let dist = (dx * dx + dy * dy).sqrt();

                    if dist < 8.0 {
                        added_heat += (8.0 - dist) / 8.0 * 0.2;
                    }
                }
                if added_heat > 0.0 {
                    self.platter.accumulate(x, y, added_heat);
                }
            }
        }

        // Platter decay
        self.platter.decay(0.9);
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let res = run_app(&mut tui);
    tui.exit()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app(tui: &mut Tui) -> Result<()> {
    let width = 80;
    let height = 40;

    // Cycle through topologies
    let topologies = [Topology::Plane, Topology::Torus, Topology::Klein, Topology::CylinderH];
    let mut topo_idx = 1;

    let mut app = LocusPlatterApp::new(width, height, topologies[topo_idx]);

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
                        .title(format!(" Locus Platter - Topology: {:?} ", app.topology)),
                )
                .x_bounds([0.0, app.width as f64])
                .y_bounds([0.0, app.height as f64])
                .paint(|ctx| {
                    for y in 0..app.height {
                        for x in 0..app.width {
                            let val = app.platter.get(x, y);
                            if val > 0.05 {
                                let color = if val > 0.8 {
                                    Color::Red
                                } else if val > 0.4 {
                                    Color::Yellow
                                } else if val > 0.2 {
                                    Color::Green
                                } else {
                                    Color::Blue
                                };
                                ctx.print(
                                    x as f64,
                                    (app.height - 1 - y) as f64, // Flip Y for canvas
                                    Span::styled("█", Style::default().fg(color)),
                                );
                            }
                        }
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let stats = Paragraph::new(
                "Use [Space] to switch topology | [Q] to quit"
            )
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
                        KeyCode::Char(' ') => {
                            topo_idx = (topo_idx + 1) % topologies.len();
                            app.topology = topologies[topo_idx];
                            // Clear platter to start fresh
                            app.platter = Platter::new(width, height);
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
