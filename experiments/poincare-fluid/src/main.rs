//! # 🌊 Poincaré Fluid
//!
//! **Hyperbolic Magnetic Fluid**
//!
//! > "The drop falls into the disk, and as it spreads to the edge, it reaches towards infinity."
//!
//! ## 🧬 Lineage
//! - **Parent A**: `crates/poincare-disk` (Hyperbolic Geometry, Non-Euclidean Distance)
//! - **Parent B**: `experiments/ferrous-fluid` (Magnetic Particle Fluid, Density Accumulation)
//! - **Concept**: A continuous 2D fluid simulation where the magnetic forces between particles are warped by the non-Euclidean geometry of a Poincaré disk.
//!
//! ## 🌿 Overview
//! This experiment drops the standard Euclidean distance calculations of our ferrous fluid simulation and replaces them with hyperbolic distance (`poincare_disk::hyperbolic_dist`).
//!
//! As particles approach the boundaries of the space, the perceived magnetic distance between them approaches infinity, radically altering how the fluid clusters and repels itself.
//!
//! ## 🧪 Novel Trait
//! **Hyperbolic Space Compression:** Particles near the center flow freely as if in normal space, but as they are pushed outward by magnetic repulsion, the space "compresses". This causes incredibly dense clustering near the boundary since the magnetic forces drop off sharply (because the hyperbolic distance is so vast), yet visually they appear stacked on top of each other.
//!
mod physics;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use physics::Universe;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

fn main() -> Result<()> {
    if std::env::args().any(|arg| arg == "--headless") {
        println!("Running in headless mode. Exiting immediately.");
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
    // 200x100 world
    let mut universe = Universe::new(200.0, 100.0);
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(33);

    // Initial magnet
    universe.add_magnet(100.0, 50.0, true);

    loop {
        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.area());

            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title(" Ferrous Fluid "))
                .x_bounds([0.0, universe.width])
                .y_bounds([0.0, universe.height]) // 0 at bottom
                .paint(|ctx| {
                    // Draw Particles
                    for p in &universe.particles {
                        ctx.print(
                            p.pos.x,
                            p.pos.y,
                            ratatui::text::Span::styled("·", Style::default().fg(Color::Cyan)),
                        );
                    }

                    // Draw Magnets
                    for mag in &universe.magnets {
                        let color = if mag.polarity { Color::Red } else { Color::Blue };
                        let label = if mag.polarity { "N" } else { "S" };
                        ctx.print(
                            mag.pos.x,
                            mag.pos.y,
                            ratatui::text::Span::styled(label, Style::default().fg(color).bg(Color::White)),
                        );
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let stats = Paragraph::new(format!(
                "Particles: {} | Magnets: {} | [WASD] Move Magnet | [Space] Toggle | [Enter] Add Magnet | [Q] Quit",
                universe.particles.len(),
                universe.magnets.len()
            ))
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(stats, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                            KeyCode::Char(' ') => {
                                if let Some(last) = universe.magnets.last_mut() {
                                    last.polarity = !last.polarity;
                                }
                            }
                            KeyCode::Char('r') => {
                                universe = Universe::new(200.0, 100.0);
                                universe.add_magnet(100.0, 50.0, true);
                            }
                            KeyCode::Enter => {
                                universe.add_magnet(100.0, 50.0, true);
                            }
                            KeyCode::Char('w') => {
                                if let Some(last) = universe.magnets.last_mut() {
                                    last.pos.y += 5.0;
                                }
                            }
                            KeyCode::Char('s') => {
                                if let Some(last) = universe.magnets.last_mut() {
                                    last.pos.y -= 5.0;
                                }
                            }
                            KeyCode::Char('a') => {
                                if let Some(last) = universe.magnets.last_mut() {
                                    last.pos.x -= 5.0;
                                }
                            }
                            KeyCode::Char('d') => {
                                if let Some(last) = universe.magnets.last_mut() {
                                    last.pos.x += 5.0;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            universe.update(0.05); // Fixed time step, keeping as f64 (literals are f64 by default)
            last_tick = Instant::now();
        }
    }
}
