use crate::physics::PhysicsWorld;
use crate::quipu_logic::QuipuMachine;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Circle, Line},
        Block, Borders, Paragraph,
    },
    Frame,
};

pub fn ui(f: &mut Frame, physics: &PhysicsWorld, machine: &QuipuMachine) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // Input Cord Log
            Constraint::Percentage(60), // Machine Visualization
            Constraint::Percentage(20), // Output Cord Log
        ])
        .split(f.area());

    // 1. Input Log
    let input_text = format!(
        "Input Cord:\n{}\n\nValue: {}",
        machine.input_cord,
        machine.input_cord.value()
    );
    let input_widget = Paragraph::new(input_text)
        .block(Block::default().borders(Borders::ALL).title("Input Source"))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(input_widget, chunks[0]);

    // 2. Output Log
    let output_text = format!(
        "Output Cord:\n{}\n\nValue: {}",
        machine.output_cord,
        machine.output_cord.value()
    );
    let output_widget = Paragraph::new(output_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Output Result"),
        )
        .style(Style::default().fg(Color::Green));
    f.render_widget(output_widget, chunks[2]);

    // 3. Machine Visualization (Canvas)
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Quipu Automaton"),
        )
        .x_bounds([-20.0, 20.0])
        .y_bounds([-20.0, 20.0]) // Physics coords: -20 (Bottom) to 20 (Top)
        .paint(|ctx| {
            // Draw Sensor (Ball)
            if let Some(sensor) = physics.rigid_body_set.get(physics.sensor_handle) {
                let pos = sensor.translation();
                let x = pos.x as f64;
                let y = pos.y as f64;
                ctx.draw(&Circle {
                    x,
                    y,
                    radius: 1.5,
                    color: Color::Red,
                });

                // Draw pivot point or anchor (spring connection)
                ctx.draw(&Line {
                    x1: x,
                    y1: y,
                    x2: -10.0,
                    y2: y, // Connection to left wall (anchor)
                    color: Color::DarkGray,
                });
            }

            // Draw Wheel
            if let Some(wheel) = physics.rigid_body_set.get(physics.wheel_handle) {
                let pos = wheel.translation();
                let x = pos.x as f64;
                let y = pos.y as f64;
                // Draw circle
                ctx.draw(&Circle {
                    x,
                    y,
                    radius: 4.0,
                    color: Color::Yellow,
                });

                // Draw spokes to show rotation
                let angle = machine.wheel_angle as f64;
                let r = 4.0;
                ctx.draw(&Line {
                    x1: x,
                    y1: y,
                    x2: x + r * angle.cos(),
                    y2: y + r * angle.sin(),
                    color: Color::Black,
                });
                ctx.draw(&Line {
                    x1: x,
                    y1: y,
                    x2: x + r * (angle + 3.14159).cos(),
                    y2: y + r * (angle + 3.14159).sin(),
                    color: Color::Black,
                });
            }

            // Draw Knots (Falling)
            for (knot, handle) in &machine.knot_bodies {
                if let Some(body) = physics.rigid_body_set.get(*handle) {
                    let pos = body.translation();
                    let radius = match knot {
                        quipu::Knot::Simple => 1.0,
                        quipu::Knot::Long(_) => 1.2,
                        quipu::Knot::FigureEight => 1.5,
                    };
                    let color = match knot {
                        quipu::Knot::Simple => Color::White,
                        quipu::Knot::Long(_) => Color::Blue,
                        quipu::Knot::FigureEight => Color::Magenta,
                    };

                    ctx.draw(&Circle {
                        x: pos.x as f64,
                        y: pos.y as f64,
                        radius,
                        color,
                    });
                }
            }
        });

    f.render_widget(canvas, chunks[1]);

    // Overlay Stats
    let stats = format!(
        "Integrator: {:.2}\nAngle: {:.2}",
        machine.integrator_value, machine.wheel_angle
    );
    let stats_widget = Paragraph::new(stats)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::White));
    // Render stats in top-left of canvas
    let stats_area = Rect {
        x: chunks[1].x + 1,
        y: chunks[1].y + 1,
        width: 20,
        height: 3,
    };
    f.render_widget(stats_widget, stats_area);
}
