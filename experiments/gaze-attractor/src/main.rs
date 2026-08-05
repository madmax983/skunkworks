//! # Gaze Attractor 👁️
//!
//! > "The act of observing a chaotic system changes it."
//!
//! **Gaze Attractor** is a hybrid experiment combining `chaos-pendulum` (Double Pendulum Physics) and `foveated-code` (Eye/Gaze Simulation).
//!
//! It simulates a "fabric" of connected nodes (a dependency graph or coupled pendulums) that exhibits chaotic behavior. A simulated "Eye" tracks the most energetic parts of the system (smooth pursuit).
//!
//! ## The Observer Effect
//!
//! The user interacts with the system through the "Gaze".
//! - **Stabilize Mode (Cyan)**: The gaze acts as a damper, calming the chaos in the foveated region.
//! - **Excite Mode (Magenta)**: The gaze adds energy (kicks) to the nodes it looks at.
//!
//! ## Lineage
//!
//! - **Parent A**: `experiments/chaos-pendulum` (Physics Engine, Verlet Integration)
//! - **Parent B**: `experiments/foveated-code` (Eye Tracking, Foveal Logic)
//! - **Novel Trait**: **The Observer Effect**. Physics dependent on attention.
//!
//! ## Controls
//!
//! - `q`: Quit
//! - `m`: Toggle Mode (Stabilize <-> Excite)
//!
//! ## Phenotype
//!
//! The system visualizes the tension between "Entropy" (Natural Chaos) and "Attention" (The Observer). When you look at the chaos, does it calm down or explode?
//!
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::canvas::{Canvas, Context, Line, Rectangle},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};
use tui_shared::Tui;

mod app;
mod eye;
mod physics;

use app::{App, Mode};

fn main() -> Result<()> {
    // Setup terminal
    let mut tui = Tui::init()?;

    // Create app
    let size = tui.terminal.size()?;
    // Physics world size matches terminal size
    let mut app = App::new(size.width as f32, size.height as f32);

    let res = run_app(&mut tui.terminal, &mut app);

    // Restore terminal
    drop(tui);

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('m') => app.toggle_mode(),
                    _ => {}
                },
                Event::Resize(w, h) => {
                    app.resize(w as f32, h as f32);
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(f.area());

    // Physics Y grows DOWN (0 is top).
    // Canvas Y grows UP (0 is bottom).
    // So we flip Y when rendering: draw_y = height - phys_y.
    let height = app.height as f64;

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Gaze Attractor"),
        )
        .x_bounds([0.0, app.width as f64])
        .y_bounds([0.0, height]) // 0 at bottom, height at top
        .paint(|ctx: &mut Context| {
            // Draw Links
            for link in &app.system.links {
                let p1 = app.system.nodes[link.a].pos;
                let p2 = app.system.nodes[link.b].pos;
                ctx.draw(&Line {
                    x1: p1.x as f64,
                    y1: height - p1.y as f64,
                    x2: p2.x as f64,
                    y2: height - p2.y as f64,
                    color: Color::DarkGray,
                });
            }

            // Draw Nodes
            for node in &app.system.nodes {
                let color = if node.fixed {
                    Color::Red
                } else if app.eye.in_fovea(node.pos.x, node.pos.y) {
                    match app.mode {
                        Mode::Stabilize => Color::Cyan,
                        Mode::Excite => Color::Magenta,
                    }
                } else {
                    Color::Green
                };

                ctx.print(
                    node.pos.x as f64,
                    height - node.pos.y as f64,
                    Span::styled("O", Style::default().fg(color)),
                );
            }

            // Draw Eye/Fovea Box
            // Eye coords are also Top-Down (0 is top).
            let fx = app.eye.x as f64;
            let fy = height - app.eye.y as f64;
            let fw = app.eye.fovea_width as f64;
            let fh = app.eye.fovea_height as f64;

            ctx.draw(&Rectangle {
                x: fx - fw / 2.0,
                y: fy - fh / 2.0,
                width: fw,
                height: fh,
                color: Color::Yellow,
            });

            // Draw Reticle
            ctx.print(fx, fy, Span::styled("+", Style::default().fg(Color::White)));
        });

    f.render_widget(canvas, chunks[0]);

    let mode_str = match app.mode {
        Mode::Stabilize => "STABILIZE (Cyan - Damping)",
        Mode::Excite => "EXCITE (Magenta - Kicking)",
    };

    let info = Paragraph::new(format!(
        "Mode: {} | Nodes: {} | Eye: ({:.1}, {:.1}) | Press 'm' to toggle mode, 'q' to quit",
        mode_str,
        app.system.nodes.len(),
        app.eye.x,
        app.eye.y
    ))
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(info, chunks[1]);
}
