//! # Klein Flock 🧬
//!
//! > "The flock flies not in space, but in the topology itself."
//!
//! **Parents**: `klein-magnetron` × `luminous-flock`
//!
//! A simulation of "Luminous Boids" (firefly-boids) inhabiting the surface of a Klein Bottle.
//!
//! ## 🧬 Genetic Lineage
//!
//! *   **From `luminous-flock`**: The boid behavior (separation, alignment, cohesion) and the pulse-synchronization mechanic (Kuramoto model).
//! *   **From `klein-magnetron`**: The topological manifold (Figure-8 immersion) and the visualization logic (wireframe rendering, projection).
//! *   **Novel Mutation**: The flocking logic has been adapted to handle non-orientable wrapping. When a boid traverses the "twist" boundary (U-wrap), its vertical coordinate (V) and vertical velocity are inverted.
//!
//! ## 🕹️ Controls
//!
//! *   **Arrows**: Rotate camera (Left/Right) / Zoom (A/D mapped in code, but UI says Arrows/WS?)
//!     *   Left/Right: Rotate
//!     *   W/S: Height
//!     *   A/D: Zoom
//! *   **P**: Pause
//! *   **R**: Reset
//! *   **Q**: Quit
//!
//! ## 🧪 Observations
//!
//! The flock exhibits "Moebius Flocking". Sub-flocks that separate and travel around the twist can collide with themselves *upside down*. The synchronization waves must also traverse this twist, potentially leading to phase cancellation or complex interference patterns.
//!
mod boid;
mod renderer;
mod topology;
mod world;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use glam::Vec3;
use ratatui::{
    prelude::*,
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};

use renderer::{draw_boids, Camera};
use world::World;

struct App {
    world: World,

    // Camera
    camera_angle: f32,
    camera_height: f32,
    camera_radius: f32,

    should_quit: bool,
    paused: bool,
}

impl App {
    fn new() -> Self {
        Self {
            world: World::new(),
            camera_angle: 0.0,
            camera_height: 1.5,
            camera_radius: 5.5,
            should_quit: false,
            paused: false,
        }
    }

    fn on_tick(&mut self) {
        if !self.paused {
            self.world.update();
        }
        // Slowly rotate camera for cinematic effect
        self.camera_angle += 0.005;
    }
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    std::io::stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let tick_rate = Duration::from_millis(30); // ~30 FPS
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                    KeyCode::Left => app.camera_angle -= 0.1,
                    KeyCode::Right => app.camera_angle += 0.1,
                    KeyCode::Char('w') => app.camera_height += 0.5,
                    KeyCode::Char('s') => app.camera_height -= 0.5,
                    KeyCode::Char('a') => app.camera_radius -= 0.5,
                    KeyCode::Char('d') => app.camera_radius += 0.5,
                    KeyCode::Char('p') => app.paused = !app.paused,
                    KeyCode::Char('r') => app.world = World::new(),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    std::io::stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main
            Constraint::Length(3), // Footer
        ])
        .split(f.area());

    // Header
    let sync_index = app.world.synchronization_index();
    let title = Paragraph::new(format!(
        "KLEIN-FLOCK | Population: {} | Sync: {:.3}",
        app.world.boids.len(),
        sync_index
    ))
    .block(Block::default().borders(Borders::ALL).title("Status"))
    .style(Style::default().fg(if sync_index > 0.8 {
        Color::Green
    } else {
        Color::Cyan
    }));
    f.render_widget(title, chunks[0]);

    // Canvas
    let canvas_area = chunks[1];
    let aspect = (canvas_area.width as f32 * 0.5) / canvas_area.height as f32; // * 0.5 because char aspect ratio

    let camera_pos = Vec3::new(
        app.camera_radius * app.camera_angle.cos(),
        app.camera_height,
        app.camera_radius * app.camera_angle.sin(),
    );
    let camera = Camera::new(camera_pos, Vec3::ZERO);

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Non-Orientable Aviary"),
        )
        .x_bounds([-2.0, 2.0])
        .y_bounds([-1.5, 1.5])
        .paint(|ctx| {
            draw_boids(ctx, &camera, aspect, &app.world);
        });
    f.render_widget(canvas, chunks[1]);

    // Footer
    let help = Paragraph::new("Arrows: Rotate/Zoom | W/S: Height | P: Pause | R: Reset | Q: Quit")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[2]);
}
