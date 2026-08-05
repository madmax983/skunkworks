//! # physics-poincare
//!
//! A TUI visualization crossing `physics-pbd` with `poincare-disk`. Rigid Euclidean bodies are simulated via Position Based Dynamics, then projected into the non-Euclidean Poincaré disk.
//!
//! ## Lineage
//!
//! *   **crates/physics-pbd**: Provides the rigid-body physics constraints (PBD solver).
//! *   **crates/poincare-disk**: Provides the hyperbolic coordinate mapping.
//! *   **Novel Trait**: Physics works in standard Euclidean space, but visualization is compressed into the non-Euclidean disk, stretching and distorting objects exponentially as they move outward.
//!
//! ## Run
//!
//! ```bash
//! cargo run -p physics-poincare
//! ```
//!
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use glam::Vec3;
use physics_pbd::PbdSystem;
use poincare_disk::{Mobius, Point};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct PhysicsPoincareApp {
    system: PbdSystem,
    transform: Mobius,
}

impl PhysicsPoincareApp {
    fn new() -> Self {
        let mut system = PbdSystem::new();
        // Anchor point (infinite mass)
        let anchor = system.add_particle(Vec3::new(0.0, 0.5, 0.0), 0.0);
        // Bob (1kg)
        let bob = system.add_particle(Vec3::new(0.5, 0.5, 0.0), 1.0);
        // Another Bob
        let bob2 = system.add_particle(Vec3::new(0.8, 0.2, 0.0), 1.0);

        system.add_distance_constraint(anchor, bob, 1.0).unwrap();
        system.add_distance_constraint(bob, bob2, 1.0).unwrap();

        // Initial push
        system.particles[bob].vel = Vec3::new(2.0, 0.0, 0.0);

        Self {
            system,
            transform: Mobius::rotation(0.0),
        }
    }

    fn tick(&mut self, dt: f32) {
        // Apply gravity manually
        for p in &mut self.system.particles {
            if p.inv_mass > 0.0 {
                p.vel.y -= 9.8 * dt;
            }
        }

        // Step the PBD system
        self.system.step(dt, 5);

        // Spin the universe a little bit
        self.transform = self.transform.then(&Mobius::rotation(0.01));
    }

    fn draw(&self, frame: &mut ratatui::Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(frame.area());

        let canvas = Canvas::default()
            .block(
                Block::default()
                    .title(" 🧬 Splice: physics-pbd × poincare-disk ")
                    .borders(Borders::ALL),
            )
            .x_bounds([-1.1, 1.1])
            .y_bounds([-1.1, 1.1])
            .paint(|ctx| {
                // Draw Boundary
                for i in 0..100 {
                    let angle = (i as f64) / 100.0 * 2.0 * std::f64::consts::PI;
                    ctx.print(
                        angle.cos(),
                        angle.sin(),
                        ratatui::text::Span::styled(".", Style::default().fg(Color::DarkGray)),
                    );
                }

                let mut projected = Vec::new();
                // Draw Particles
                for p in &self.system.particles {
                    // Scale down euclidean coordinates so they fit inside unit disk comfortably
                    let scale = 0.5;
                    let mut pt = Point::new(p.pos.x as f64 * scale, p.pos.y as f64 * scale);

                    // ensure inside disk
                    if pt.norm() >= 1.0 {
                        pt = pt * (0.99 / pt.norm());
                    }

                    pt = self.transform.apply(pt);
                    projected.push(pt);

                    let color = if p.inv_mass == 0.0 {
                        Color::Red
                    } else {
                        Color::Green
                    };
                    ctx.print(
                        pt.re,
                        pt.im,
                        ratatui::text::Span::styled("●", Style::default().fg(color)),
                    );
                }

                // Draw Constraints
                for c in &self.system.constraints {
                    match c {
                        physics_pbd::Constraint::Distance { p1, p2, .. } => {
                            let pt1 = projected[*p1];
                            let pt2 = projected[*p2];
                            ctx.draw(&ratatui::widgets::canvas::Line {
                                x1: pt1.re,
                                y1: pt1.im,
                                x2: pt2.re,
                                y2: pt2.im,
                                color: Color::White,
                            });
                        }
                        _ => {}
                    }
                }
            });

        frame.render_widget(canvas, chunks[0]);

        let info = Paragraph::new(
            "Euclidean rigid-body physics constrained and projected onto the Poincaré disk. [Q] Quit",
        )
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
        frame.render_widget(info, chunks[1]);
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("🧬 physics-poincare running in headless mode for 10 ticks.");
        let mut app = PhysicsPoincareApp::new();
        for _ in 0..10 {
            app.tick(0.016);
        }
        println!("Finished headless run.");
        return Ok(());
    }

    let mut tui = Tui::init()?;
    let mut app = PhysicsPoincareApp::new();
    let tick_rate = Duration::from_millis(33);
    let mut last_tick = Instant::now();

    loop {
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
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick(0.033);
            last_tick = Instant::now();
        }
    }

    tui.exit()?;
    Ok(())
}
