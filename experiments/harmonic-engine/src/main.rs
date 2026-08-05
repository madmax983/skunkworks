//! # Harmonic Engine
//!
//! > "A mechanical computer that sings the differential equations it solves."
//!
//! **Genesis: The Horologist**
//!
//! This experiment simulates a **Mechanical Differential Analyzer** using `rapier2d` physics.
//! It constructs a system of **Ball-and-Disk Integrators** coupled together to solve differential equations physically.
//! The output of the integrators drives a **Music Box** mechanism, where the rotation of the solution cylinder plucks teeth to generate sound.
//!
//! ## Mechanism
//!
//! ### The Ball-and-Disk Integrator
//! The core component is the Ball-and-Disk integrator, a mechanical device used in early analog computers (like the Bush Differential Analyzer).
//! 1.  **Input Disk**: Rotates at a constant speed (representing Time $t$).
//! 2.  **Ball**: Positioned at distance $r$ from the center. This represents the value of the integrand $y$.
//! 3.  **Output Cylinder**: Driven by the ball via friction. Its angular velocity $\omega_{out}$ is proportional to the disk speed $\omega_{in}$ and the ball position $r$.
//!     $$ \omega_{out} = k \cdot \omega_{in} \cdot r $$
//!     Since $\omega_{in}$ is constant, the total rotation $\theta_{out} = \int \omega_{out} dt \propto \int r dt$.
//!     Thus, the cylinder integrates the ball's position.
//!
//! ### The Harmonic Oscillator
//! We couple two integrators to solve the harmonic oscillator equation:
//! $$ y'' = -y $$
//! Or as a system of first-order equations:
//! $$ y' = v $$
//! $$ v' = -y $$
//!
//! -   **Integrator 1 (y)**: Ball position is driven by $v$ (output of Integrator 2). Output is $y$.
//! -   **Integrator 2 (v)**: Ball position is driven by $-y$ (output of Integrator 1). Output is $v$.
//!
//! The result is a sinusoidal oscillation of the ball positions and cylinder rotations.
//!
//! ### The Music Box
//! The Output Cylinders are equipped with virtual "pins". As they rotate, these pins strike the teeth of a comb, generating notes.
//! The melody produced is a direct sonification of the solution curve.
//!
//! ## Controls
//! -   **Run**: `cargo run -p harmonic-engine`
//! -   **Quit**: Press `q`.
//!
//! ## Visualization
//! The TUI displays the rotating disks and the sliding balls.
//! -   **White Circle**: Input Disk.
//! -   **Red Dot**: The Ball (Value).
//! -   **Yellow Line**: The Output Cylinder (Rotation).
//! -   **Green Line**: The Mechanical Coupling (Data Flow).
//!
//! ## Stack
//! -   **Physics**: `rapier2d` (Rigid Body Dynamics + Kinematic Constraints).
//! -   **Rendering**: `ratatui` (Terminal UI).
//! -   **Input**: `crossterm`.
//!
use crossterm::event::{self, Event, KeyCode};
use harmonic_engine::{MusicBox, PhysicsWorld};
use nalgebra::Vector2;
use ratatui::{prelude::*, widgets::*};
use std::{error::Error, time::Duration};
use tui_shared::{Button, LogList, Tui};

fn main() -> Result<(), Box<dyn Error>> {
    // Setup Terminal
    let mut tui = Tui::init()?;

    // Setup Simulation
    let mut world = PhysicsWorld::new();

    // Position Integrators
    // Int 0: Position y (Left)
    let idx_y = world.add_integrator(Vector2::new(-20.0, 0.0));
    // Int 1: Velocity v (Right)
    let idx_v = world.add_integrator(Vector2::new(20.0, 0.0));

    // Couple them
    // dy/dt = v -> v (Output of 1) drives y-integrator ball (Ball of 0)
    world.add_coupling(idx_v, idx_y, 0.5);
    // dv/dt = -y -> y (Output of 0) drives v-integrator ball (Ball of 1)
    world.add_coupling(idx_y, idx_v, -0.5);

    let mut music_box = MusicBox::new(world.integrators.len());
    let mut event_log: Vec<String> = Vec::new();

    // Initial success message to show off the icon
    event_log.push("Success: Harmonic Engine Initialized".to_string());

    loop {
        // Input
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    break;
                }
            }
        }

        // Update
        world.step();
        let events = music_box.update(&world);
        for event in events {
            event_log.push(format!("Note: {} (Vel: {:.1})", event.note, event.velocity));
            if event_log.len() > 20 {
                event_log.remove(0);
            }
        }

        // Draw
        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.area());

            let main_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(chunks[0]);

            let canvas = ratatui::widgets::canvas::Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Harmonic Engine"),
                )
                .paint(|ctx| {
                    // Draw integrators
                    for integrator in &world.integrators {
                        if let Some(disk) = world.rigid_body_set.get(integrator.disk_handle) {
                            let pos = *disk.translation();
                            ctx.draw(&ratatui::widgets::canvas::Circle {
                                x: pos.x as f64,
                                y: pos.y as f64,
                                radius: 10.0,
                                color: Color::White,
                            });
                        }
                        if let Some(ball) = world.rigid_body_set.get(integrator.ball_handle) {
                            let pos = *ball.translation();
                            ctx.draw(&ratatui::widgets::canvas::Circle {
                                x: pos.x as f64,
                                y: pos.y as f64,
                                radius: 1.0,
                                color: Color::Red,
                            });
                        }
                        if let Some(cyl) = world.rigid_body_set.get(integrator.output_handle) {
                            let pos = *cyl.translation();
                            let rot = cyl.rotation().angle();
                            // Draw Output Cylinder as a rotating line/bar
                            ctx.draw(&ratatui::widgets::canvas::Line {
                                x1: pos.x as f64,
                                y1: pos.y as f64,
                                x2: (pos.x + 8.0 * rot.cos()) as f64,
                                y2: (pos.y + 8.0 * rot.sin()) as f64,
                                color: Color::Yellow,
                            });
                            // Draw a perpendicular line to show rotation clearly
                            ctx.draw(&ratatui::widgets::canvas::Line {
                                x1: pos.x as f64,
                                y1: pos.y as f64,
                                x2: (pos.x + 8.0 * (rot + 1.57).cos()) as f64,
                                y2: (pos.y + 8.0 * (rot + 1.57).sin()) as f64,
                                color: Color::Yellow,
                            });
                        }
                    }

                    // Draw Coupling Lines (Abstract)
                    // From Output of 1 to Ball of 0
                    if world.integrators.len() > 1 {
                        // idx_v (1) -> idx_y (0)
                        if let Some(cyl) = world
                            .rigid_body_set
                            .get(world.integrators[idx_v].output_handle)
                        {
                            if let Some(ball) = world
                                .rigid_body_set
                                .get(world.integrators[idx_y].ball_handle)
                            {
                                ctx.draw(&ratatui::widgets::canvas::Line {
                                    x1: cyl.translation().x as f64,
                                    y1: cyl.translation().y as f64,
                                    x2: ball.translation().x as f64,
                                    y2: ball.translation().y as f64,
                                    color: Color::Green,
                                });
                            }
                        }
                        // idx_y (0) -> idx_v (1)
                        if let Some(cyl) = world
                            .rigid_body_set
                            .get(world.integrators[idx_y].output_handle)
                        {
                            if let Some(ball) = world
                                .rigid_body_set
                                .get(world.integrators[idx_v].ball_handle)
                            {
                                ctx.draw(&ratatui::widgets::canvas::Line {
                                    x1: cyl.translation().x as f64,
                                    y1: cyl.translation().y as f64,
                                    x2: ball.translation().x as f64,
                                    y2: ball.translation().y as f64,
                                    color: Color::Green,
                                });
                            }
                        }
                    }
                })
                .x_bounds([-40.0, 40.0])
                .y_bounds([-30.0, 30.0]);

            f.render_widget(canvas, main_chunks[0]);

            let log_list = LogList::new(event_log.iter().cloned().rev()).with_title("Music Log");
            f.render_widget(log_list, main_chunks[1]);

            // Footer with buttons
            let footer_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Min(0), Constraint::Length(20)])
                .split(chunks[1]);

            let footer_text = Paragraph::new("Controls: Q to Quit")
                .block(Block::default().borders(Borders::ALL))
                .alignment(Alignment::Center);
            f.render_widget(footer_text, footer_layout[0]);

            let quit_btn = Button::new("Quit")
                .style(
                    ratatui::style::Style::default()
                        .bg(ratatui::style::Color::Red)
                        .fg(ratatui::style::Color::White)
                        .add_modifier(ratatui::style::Modifier::BOLD),
                )
                .icon("🛑"); // Just visual for now
            f.render_widget(quit_btn, footer_layout[1]);
        })?;
    }

    // Restore Terminal happens automatically on Drop of `tui`
    Ok(())
}
