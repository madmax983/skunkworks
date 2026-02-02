use crate::simulation::World;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Circle},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn ui(f: &mut Frame, world: &World) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Code Bio-Dome"),
        )
        .x_bounds([0.0, world.width])
        .y_bounds([0.0, world.height])
        .paint(|ctx| {
            // Draw Creatures
            for creature in &world.creatures {
                if creature.energy <= 0.0 {
                    // Dead/Hibernating
                    ctx.draw(&Circle {
                        x: creature.pos.0,
                        y: creature.pos.1,
                        radius: 2.0,
                        color: Color::Gray,
                    });
                } else {
                    let color = string_to_color(&creature.output);

                    // Body
                    ctx.draw(&Circle {
                        x: creature.pos.0,
                        y: creature.pos.1,
                        radius: 3.0,
                        color,
                    });

                    // Energy indicator (inner circle size?)
                    // Or just visual style.
                }

                // Name label if high energy or special?
                // Too cluttering.
            }

            // Draw Particles
            for particle in &world.particles {
                let color = string_to_color(&particle.type_name);
                ctx.draw(&Circle {
                    x: particle.pos.0,
                    y: particle.pos.1,
                    radius: 1.0,
                    color,
                });
            }
        });

    f.render_widget(canvas, chunks[0]);

    // Status Bar
    let status = format!(
        "Creatures: {} | Particles: {} | Frame: {} | [Q] Quit | [S] Semantic Dump",
        world.creatures.len(),
        world.particles.len(),
        world.frame
    );
    let p = Paragraph::new(status).style(Style::default().bg(Color::White).fg(Color::Black));
    f.render_widget(p, chunks[1]);
}

fn string_to_color(s: &str) -> Color {
    if s.is_empty() {
        return Color::DarkGray;
    }

    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    let hash = hasher.finish();

    // Simple mapping to bright colors
    match hash % 6 {
        0 => Color::Red,
        1 => Color::Green,
        2 => Color::Yellow,
        3 => Color::Blue,
        4 => Color::Magenta,
        _ => Color::Cyan,
    }
}
