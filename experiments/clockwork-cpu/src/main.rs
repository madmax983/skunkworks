use clockwork_cpu::mechanism::Clockwork;
use clockwork_cpu::physics::{PhysicsWorld, RenderBody};
use crossterm::event::{self, Event, KeyCode};
use nalgebra::point;
use rapier2d::prelude::*;
use ratatui::{
    prelude::*,
    widgets::{
        canvas::{Line as CanvasLine, *},
        *,
    },
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

fn main() -> anyhow::Result<()> {
    let mut tui = Tui::init()?;

    let mut world = PhysicsWorld::new();
    let clockwork = Clockwork::build(&mut world);

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(16);

    let mut frame_count = 0;

    // Initial wheel angle to track cycles
    let mut last_angle = 0.0;
    let _cycles = 0;

    loop {
        // Handle input
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        // Update Physics
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();

            // Apply torque to the wheel to simulate mainspring
            if let Some(wheel) = world.rigid_body_set.get_mut(clockwork.wheel_handle) {
                wheel.reset_torques(true);
                // Apply constant torque.
                wheel.add_torque(-50.0, true);

                // Track cycles
                let angle = wheel.rotation().angle();
                if (angle - last_angle).abs() > 0.1 {
                    // Moving
                }
                last_angle = angle;
            }

            world.step();
            frame_count += 1;
        }

        // Render
        tui.terminal.draw(|f| {
            let area = f.area();

            let bodies = world.get_render_bodies();

            let canvas = Canvas::default()
                .block(Block::bordered().title("Clockwork CPU (Rapier2d + Ratatui)"))
                .x_bounds([-8.0, 8.0])
                .y_bounds([-8.0, 8.0])
                .paint(|ctx| {
                    for body in &bodies {
                        draw_body(ctx, body);
                    }

                    // Draw Pivot points (visual only)
                    ctx.draw(&Circle {
                        x: 0.0,
                        y: 0.0,
                        radius: 0.2,
                        color: Color::Red,
                    });
                    ctx.draw(&Circle {
                        x: 0.0,
                        y: 3.5,
                        radius: 0.2,
                        color: Color::Red,
                    });
                });

            f.render_widget(canvas, area);

            // Stats overlay
            let stats = Paragraph::new(format!(
                "Frames: {}\nTorque: -50.0\nPress 'q' to quit",
                frame_count
            ))
            .block(Block::default().borders(Borders::ALL));
            let stats_area = Rect::new(area.width.saturating_sub(25), 0, 25, 5);
            f.render_widget(stats, stats_area);
        })?;
    }

    Ok(())
}

fn draw_body(ctx: &mut Context, body: &RenderBody) {
    let pos = body.position;
    let shape = &body.shape;
    draw_shape(ctx, &pos, shape);
}

fn draw_shape(ctx: &mut Context, pos: &Isometry<Real>, shape: &SharedShape) {
    match shape.as_typed_shape() {
        TypedShape::Ball(b) => {
            ctx.draw(&Circle {
                x: pos.translation.x as f64,
                y: pos.translation.y as f64,
                radius: b.radius as f64,
                color: Color::Cyan,
            });
        }
        TypedShape::Cuboid(c) => {
            // Calculate 4 corners
            let hx = c.half_extents.x;
            let hy = c.half_extents.y;
            let corners = [
                point![-hx, -hy],
                point![hx, -hy],
                point![hx, hy],
                point![-hx, hy],
            ];

            let world_corners: Vec<(f64, f64)> = corners
                .iter()
                .map(|p| {
                    let wp = pos * p;
                    (wp.x as f64, wp.y as f64)
                })
                .collect();

            // Draw lines
            for i in 0..4 {
                let (x1, y1) = world_corners[i];
                let (x2, y2) = world_corners[(i + 1) % 4];
                ctx.draw(&CanvasLine {
                    x1,
                    y1,
                    x2,
                    y2,
                    color: Color::Green,
                });
            }
        }
        TypedShape::Compound(c) => {
            for (local_pos, sub_shape) in c.shapes() {
                let world_pos = pos * local_pos;
                draw_shape(ctx, &world_pos, sub_shape);
            }
        }
        _ => {}
    }
}
