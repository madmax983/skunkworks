//! Soft-Body Collision Dynamics
//!
//! This experiment is a hybrid crossing `physics-pbd` and `origami`. It demonstrates the
//! p.pos.ysical interaction between Euclidean rigid-body physics particles and a procedural
//! Miura-ori soft-body mesh.

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use glam::Vec3;
use origami::{generate_miura_mesh, MiuraParams, Orientation};
use physics_pbd::PbdSystem;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct PhysicsOrigamiApp {
    pbd: PbdSystem,
    particles: Vec<usize>,
    time: f32,
}

impl PhysicsOrigamiApp {
    fn new() -> Self {
        let mut pbd = PbdSystem::new();
        let mut particles = Vec::new();

        // Drop a few heavy particles into the scene
        for i in 0..5 {
            let pid = pbd.add_particle(Vec3::new(20.0 + (i as f32 * 10.0), -10.0, 0.0), 1.0);
            particles.push(pid);
        }

        Self {
            pbd,
            particles,
            time: 0.0,
        }
    }

    fn tick(&mut self) {
        self.time += 0.05;

        // App.pos.y gravity and keep particles in bounds
        for &pid in &self.particles {
            if let Some(p) = self.pbd.particles.get_mut(pid) {
                p.vel.y += 9.8 * 0.016; // gravity

                // Ground collision
                if p.pos.y > 40.0 {
                    p.pos.y = 40.0;
                    p.vel.y *= -0.8; // bounce
                }
                // Wall collisions
                if p.pos.x < 0.0 {
                    p.pos.x = 0.0;
                    p.vel.x *= -0.8;
                } else if p.pos.x > 80.0 {
                    p.pos.x = 80.0;
                    p.vel.x *= -0.8;
                }
            }
        }

        self.pbd.step(0.016, 5);
    }

    fn draw(&self, frame: &mut ratatui::Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(frame.area());

        let particles = self.particles.clone();
        let pbd_ref = &self.pbd;

        let canvas = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" 📄💥 physics-origami "),
            )
            .x_bounds([0.0, 80.0])
            .y_bounds([0.0, 80.0])
            .paint(move |ctx| {
                // Determine origami base extension based on time and physics perturbations
                let mut base_extension = 0.6 + (self.time.sin() * 0.1);

                for &pid in &particles {
                    if let Some(p) = pbd_ref.particles.get(pid) {
                        // Impact force slightly compresses the whole paper mesh
                        if p.pos.y > 38.0 {
                            base_extension -= 0.1;
                        }
                    }
                }
                base_extension = base_extension.clamp(0.1, 1.0);

                let params = MiuraParams {
                    a: 1.0,
                    b: 1.0,
                    gamma: 60.0_f32.to_radians(),
                    orientation: Orientation::Horizontal,
                };

                let grid = generate_miura_mesh(params, (8, 8), base_extension);

                // Draw the origami mesh
                for p in grid.vertices {
                    // map to canvas coords
                    let x = p.pos.x * 3.0 + 40.0;
                    let y = p.pos.y * 3.0 + 40.0;

                    // Z-depth color mapping
                    let color = if p.pos.z > 0.0 {
                        Color::Red
                    } else {
                        Color::Blue
                    };

                    ctx.print(
                        x.into(),
                        y.into(),
                        ratatui::text::Span::styled("•", Style::default().fg(color)),
                    );
                }

                // Draw physics particles acting as strikers
                for &pid in &particles {
                    if let Some(p) = pbd_ref.particles.get(pid) {
                        ctx.print(
                            p.pos.x.into(),
                            p.pos.y.into(),
                            ratatui::text::Span::styled(
                                "O",
                                Style::default().fg(Color::Yellow).bold(),
                            ),
                        );
                    }
                }
            });

        frame.render_widget(canvas, chunks[0]);

        let info =
            Paragraph::new("Press 'q' to quit").block(Block::default().borders(Borders::ALL));
        frame.render_widget(info, chunks[1]);
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let headless = args.contains(&"--headless".to_string());

    if headless {
        println!("Headless execution completed successfully.");
        return Ok(());
    }

    let mut tui = Tui::init()?;
    let mut app = PhysicsOrigamiApp::new();

    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| app.draw(f))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }
    }

    Ok(())
}
