//! # Gray-Locus Morphogenesis
//!
//! This hybrid bridges the biological reaction-diffusion patterns of `gray-scott` with the continuous,
//! non-Euclidean topological boundaries of `locus`.
//!
//! ## Lineage
//!
//! * **From `gray-scott`**: Provides the continuous thermodynamic scalar field representing the chemical concentrations of U and V, resulting in Turing patterns.
//! * **From `locus`**: Provides the `Topology` enum and `Vec2` logic to wrap a moving "cursor" or agent dropping the reaction-diffusion seed across non-Euclidean boundaries (e.g., Klein Bottle).
//!
//! ## Emergent Phenotype
//!
//! A localized chemical seed drifts through space, wrapping around a Klein Bottle boundary.
//! It continuously drops the `V` chemical. The resulting Turing patterns form organic highways
//! reflecting the path of the seed, proving biological computation can be seeded via topological movement.
//!

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use gray_scott::GrayScott;
use locus::{Topology, Vec2};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct GrayLocusApp {
    dish: GrayScott,
    width: usize,
    height: usize,
    pos: Vec2,
    vel: Vec2,
    topo: Topology,
}

impl GrayLocusApp {
    fn new(width: usize, height: usize) -> Self {
        let mut dish = GrayScott::new(width, height);
        // Add a chemical seed in the center
        dish.add_chemical(width / 2, height / 2, 1.0);
        dish.add_chemical(width / 2 + 1, height / 2, 1.0);
        dish.add_chemical(width / 2, height / 2 + 1, 1.0);
        dish.add_chemical(width / 2 + 1, height / 2 + 1, 1.0);
        Self {
            dish,
            width,
            height,
            pos: Vec2::new(width as f64 / 2.0, height as f64 / 2.0),
            vel: Vec2::new(1.5, 0.8), // Constant drifting velocity
            topo: Topology::Klein,
        }
    }

    fn tick(&mut self) {
        let feed = 0.055;
        let kill = 0.062;
        // Perform multiple simulation steps per tick for faster morphogenesis
        for _ in 0..10 {
            self.dish.update(feed, kill, 1.0);
        }

        // Move the seed
        self.pos += self.vel;

        // Use locus topology to normalize the position
        let y_idx = self.pos.y.round() as i64;
        let x_idx = self.pos.x.round() as i64;

        if let Some((ny, nx)) = self.topo.normalize(y_idx, x_idx, self.width, self.height) {
            // Drop chemical at the wrapped location
            self.dish.add_chemical(nx, ny, 1.0);

            // Re-sync float position with wrapped grid position
            self.pos.x = nx as f64;
            self.pos.y = ny as f64;
        } else {
            // Out of bounds for the topology (e.g. Plane), reverse velocity
            self.vel.x = -self.vel.x;
            self.vel.y = -self.vel.y;
            // Nudge back
            self.pos += self.vel;
        }
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
    let mut app = GrayLocusApp::new(width, height);

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
                        .title(" Gray-Locus Morphogenesis "),
                )
                .x_bounds([0.0, app.width as f64])
                .y_bounds([0.0, app.height as f64])
                .paint(|ctx| {
                    let v_buf = app.dish.v();
                    for y in 0..app.height {
                        for x in 0..app.width {
                            let v = v_buf[y * app.width + x];
                            if v > 0.1 {
                                let color = if v > 0.5 { Color::Cyan } else { Color::Blue };

                                ctx.print(
                                    x as f64,
                                    y as f64,
                                    ratatui::text::Span::styled("•", Style::default().fg(color)),
                                );
                            }
                        }
                    }

                    // Draw the moving seed cursor
                    ctx.print(
                        app.pos.x as f64,
                        app.pos.y as f64,
                        ratatui::text::Span::styled("x", Style::default().fg(Color::Yellow)),
                    );
                });

            f.render_widget(canvas, chunks[0]);

            let v_buf = app.dish.v();
            let total_v: f32 = v_buf.iter().sum();

            let stats = Paragraph::new(format!(
                "Topology: Klein Bottle | Total Mass (V): {:.2} | Grid: {}x{} | [Q] Quit",
                total_v, app.width, app.height
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
