use crossterm::event::{self, Event, KeyCode};
use harmonic_engine::{
    audio::MusicBox,
    physics::PhysicsWorld,
    widgets::{Button, ButtonState, ButtonStyle, LogList},
};
use nalgebra::Vector2;
use ratatui::{prelude::*, widgets::*};
use std::{error::Error, time::Duration};
use tui_shared::Tui;

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
                            let pos = disk.translation();
                            ctx.draw(&ratatui::widgets::canvas::Circle {
                                x: pos.x as f64,
                                y: pos.y as f64,
                                radius: 10.0,
                                color: Color::White,
                            });
                        }
                        if let Some(ball) = world.rigid_body_set.get(integrator.ball_handle) {
                            let pos = ball.translation();
                            ctx.draw(&ratatui::widgets::canvas::Circle {
                                x: pos.x as f64,
                                y: pos.y as f64,
                                radius: 1.0,
                                color: Color::Red,
                            });
                        }
                        if let Some(cyl) = world.rigid_body_set.get(integrator.output_handle) {
                            let pos = cyl.translation();
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

            let log_list =
                LogList::new(event_log.clone().into_iter().rev().collect()).with_title("Music Log");
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
                .style_variant(ButtonStyle::Danger)
                .icon("🛑")
                .state(ButtonState::Normal); // Just visual for now
            f.render_widget(quit_btn, footer_layout[1]);
        })?;
    }

    // Restore Terminal happens automatically on Drop of `tui`
    Ok(())
}
