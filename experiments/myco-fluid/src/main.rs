//! # 🧫 `myco-fluid`
//!
//! A biological-magnetic hybrid TUI experiment.
//!
//! ## Lineage
//!
//! **Parents**:
//! - `experiments/myco-transit` (Slime mold cellular automata and pheromone pathfinding)
//! - `experiments/ferrous-fluid` (Magnetic particle simulation)
//!
//! **Created by**: The Splice Surgeon 🧬
//!
//! ## Phenotype (Emergent Trait)
//!
//! **Pheromone-Guided Magnetic Particles:**
//! The particles from `ferrous-fluid` act as the agents. They are primarily driven by raw magnetic forces (Pull/Repel from user-placed magnets), but as they move, they deposit a "pheromone trail" onto a grid (inherited from `myco-transit`). This trail diffuses and decays over time.
//!
//! Crucially, the particles are not just passive writers to the grid—they *read* it. They calculate the gradient of the local pheromone density and are gently pulled towards areas of higher density.
//!
//! This creates an emergent "Magnetic Memory". The trails formed by past magnetic lines of force become stable physical structures guiding future particles. It’s no longer just a fluid responding to magnets; it's a slime mold that reorganizes its pathing networks based on magnetic stimuli.
//!
//! ## Controls
//! - `[WASD]` Move Magnet
//! - `[Space]` Toggle Polarity (Red = North/Attract, Blue = South/Repel)
//! - `[Enter]` Add Magnet
//! - `[R]` Reset
//! - `[Q]` Quit
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
                .block(Block::default().borders(Borders::ALL).title(" Myco-Fluid "))
                .x_bounds([0.0, universe.width])
                .y_bounds([0.0, universe.height]) // 0 at bottom
                .paint(|ctx| {
                    // Draw Pheromones
                    for y in 0..universe.grid_h {
                        for x in 0..universe.grid_w {
                            let scent = universe.get_trail(x, y);
                            if scent > 0.1 {
                                let intensity = (scent * 255.0).min(255.0) as u8;
                                let color = Color::Rgb(0, intensity, intensity); // Cyan-ish trail
                                ctx.print(x as f64, y as f64, ratatui::text::Span::styled("·", Style::default().fg(color)));
                            }
                        }
                    }

                    // Draw Particles
                    for p in &universe.particles {
                        ctx.print(
                            p.pos.x,
                            p.pos.y,
                            ratatui::text::Span::styled("O", Style::default().fg(Color::Yellow)),
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
            universe.update(0.05); // Fixed time step
            last_tick = Instant::now();
        }
    }
}
