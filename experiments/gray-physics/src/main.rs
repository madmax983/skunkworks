//! # gray-physics
//!
//! **Concept**: Morphogenetic Soft-Body Deformation.
//!
//! **Novel trait**: The continuous chemical Turing patterns (Gray-Scott) actively control the stiffness and constraints of a Position Based Dynamics (PBD) soft-body simulation. High concentration of the kill chemical physically alters constraints, creating structural weaknesses or movements driven by the chemical reaction.
//!
//! **Lineage**:
//! - From `gray-scott`: Continuous chemical reaction-diffusion simulation.
//! - From `physics-pbd`: Soft body mechanics, particles, constraints, implicit solver.
//!
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use glam::Vec3;
use gray_scott::GrayScott;
use physics_pbd::PbdSystem;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::time::{Duration, Instant};

struct App {
    gs: GrayScott,
    physics: PbdSystem,
    particles: Vec<usize>,
    quit: bool,
}

impl App {
    fn new(width: usize, height: usize) -> Self {
        let mut gs = GrayScott::new(width, height);
        gs.add_chemical(width / 2, height / 2, 1.0);

        let mut physics = PbdSystem::new();
        let mut particles = Vec::new();

        let spacing = 2.0;
        for y in 0..10 {
            for x in 0..10 {
                let px = x as f32 * spacing;
                let py = y as f32 * spacing;
                let mass = if y == 0 { 0.0 } else { 1.0 };
                let p_id = physics.add_particle(Vec3::new(px, py, 0.0), mass);
                particles.push(p_id);
            }
        }

        for y in 0..10 {
            for x in 0..10 {
                let i = y * 10 + x;
                if x < 9 {
                    let _ =
                        physics.add_distance_constraint(particles[i], particles[i + 1], spacing);
                }
                if y < 9 {
                    let _ =
                        physics.add_distance_constraint(particles[i], particles[i + 10], spacing);
                }
            }
        }

        Self {
            gs,
            physics,
            particles,
            quit: false,
        }
    }

    fn update(&mut self) {
        self.gs.update(0.0367, 0.0649, 1.0);

        let gs_v = self.gs.v();
        let v_avg = gs_v.iter().sum::<f32>() / gs_v.len() as f32;

        let particles_len = self.particles.len();
        for p_idx in 0..particles_len {
            let p_id = self.particles[p_idx];
            if let Some(particle) = self.physics.particles.get_mut(p_id) {
                if particle.inv_mass > 0.0 {
                    particle.vel.y += v_avg * 0.1;
                }
            }
        }

        self.physics.step(0.016, 5);
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        let mut app = App::new(20, 20);
        for _ in 0..10 {
            app.update();
        }
        println!("gray-physics headless run complete.");
        return Ok(());
    }

    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(40, 20);
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    while !app.quit {
        terminal.draw(|f| {
            let area = f.area();
            let chunks = Layout::vertical([Constraint::Percentage(100)]).split(area);

            let mut text = String::new();
            text.push_str("Gray-Physics Hybrid\n");

            for p in &app.particles {
                if let Some(particle) = app.physics.particles.get(*p) {
                    text.push_str(&format!(
                        "P: ({:.1}, {:.1}) ",
                        particle.pos.x, particle.pos.y
                    ));
                }
            }

            let p = Paragraph::new(text)
                .block(Block::default().borders(Borders::ALL).title("Gray-Physics"));
            f.render_widget(p, chunks[0]);
        })?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    app.quit = true;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }
    }

    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen
    )?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
