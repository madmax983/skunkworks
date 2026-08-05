//! # Luminous Flock (Web)
//!
//! A boids + firefly synchronization simulation running in the browser via WebAssembly.
//!
//! Ported from the terminal version using [Ratzilla](https://github.com/ratatui/ratzilla).
//!
//! ## Local Development
//!
//! ```bash
//! # Install trunk if you haven't
//! cargo install trunk
//!
//! # Add WASM target
//! rustup target add wasm32-unknown-unknown
//!
//! # Run dev server
//! trunk serve --open
//!
//! # Build optimized release
//! trunk build --release
//! ```
//!
//! ## Controls
//!
//! - **r** - Reset simulation
//!
//! ## Deploy to Render
//!
//! 1. Create a new Static Site on Render
//! 2. Connect your repository
//! 3. Set build command: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && source $HOME/.cargo/env && rustup target add wasm32-unknown-unknown && cargo install trunk && cd experiments/luminous-flock-web && trunk build --release`
//! 4. Set publish directory: `experiments/luminous-flock-web/dist`
//!
//! Or use the `render.yaml` Blueprint for automatic deployment.
//!
mod boid;
mod world;

use std::cell::RefCell;
use std::rc::Rc;

use ratzilla::ratatui::{
    Frame, Terminal,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, canvas::Canvas},
};
use ratzilla::{DomBackend, WebRenderer, event::KeyCode};

use world::World;

fn main() -> std::io::Result<()> {
    let world_width = 200.0;
    let world_height = 100.0;

    let world = Rc::new(RefCell::new(World::new(world_width, world_height)));

    let backend = DomBackend::new()?;
    let mut terminal = Terminal::new(backend)?;

    // Handle keyboard input
    let world_clone = world.clone();
    terminal.on_key_event(move |key_event| {
        if key_event.code == KeyCode::Char('r') {
            *world_clone.borrow_mut() = World::new(world_width, world_height);
        }
    });

    // Main render loop
    terminal.draw_web(move |f| {
        // Update simulation
        world.borrow_mut().update();

        // Draw UI
        ui(f, &world.borrow());
    });

    Ok(())
}

fn ui(f: &mut Frame, world: &World) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    let sync_index = world.synchronization_index();

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Luminous Flock (Boids + Fireflies)"),
        )
        .x_bounds([0.0, world.width])
        .y_bounds([0.0, world.height])
        .paint(|ctx| {
            for boid in &world.boids {
                let (char_str, color) = if boid.flash_timer > 0 {
                    ("*".to_string(), Color::White)
                } else {
                    let base_char = boid.dna.char_representation.to_string();
                    let color = boid.dna.color;
                    (base_char, color)
                };

                ctx.print(
                    boid.position.0,
                    boid.position.1,
                    Span::styled(char_str, Style::default().fg(color)),
                );
            }
        });

    f.render_widget(canvas, chunks[0]);

    let status = format!(
        "Sync Index: {:.3} | Population: {} | 'r': Reset",
        sync_index,
        world.boids.len()
    );
    let p = Paragraph::new(status).style(Style::default().fg(Color::Black).bg(Color::Blue));
    f.render_widget(p, chunks[1]);
}
