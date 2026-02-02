pub mod world;

use crossterm::event::{self, Event, KeyCode};
use rand::Rng;
use ratatui::{
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders,
    },
};
use std::collections::HashMap;
use std::{error::Error, time::Duration};
use tui_shared::Tui;
use world::{Material, World};

fn main() -> Result<(), Box<dyn Error>> {
    let mut tui = Tui::init()?;

    let width = 100;
    let height = 60;
    let mut world = World::new(width, height);
    let mut rng = rand::thread_rng();

    // Initialize scenario
    // Central server block
    for y in height / 2 - 5..height / 2 + 5 {
        for x in width / 2 - 5..width / 2 + 5 {
            world.add_server(x, y);
        }
    }

    // Scattered dirt
    // Reduced to 1500 for better movement
    for _ in 0..1500 {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        // Don't overwrite server
        if matches!(world.get_cell(x, y).material, Material::Empty) {
            world.add_wall(x, y);
        }
    }

    // Termites
    for _ in 0..1000 {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        world.add_termite(x, y);
    }

    loop {
        // Handle events
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        // Update
        world.update();

        // Draw
        tui.terminal.draw(|f| {
            let size = f.area();

            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Thermo-Termites (Press 'q' to quit)"),
                )
                .x_bounds([0.0, width as f64])
                .y_bounds([0.0, height as f64])
                .paint(|ctx| {
                    // Batch drawing
                    let mut points_by_color: HashMap<Color, Vec<(f64, f64)>> = HashMap::new();

                    for y in 0..height {
                        for x in 0..width {
                            let cell = world.get_cell(x, y);
                            let color = match cell.material {
                                Material::Server => Color::Red,
                                Material::Wall => Color::White,
                                Material::Empty => {
                                    // Heat map gradient
                                    if cell.heat > 100.0 {
                                        Color::Magenta
                                    } else if cell.heat > 50.0 {
                                        Color::Rgb(255, 100, 0)
                                    } else if cell.heat > 20.0 {
                                        Color::Rgb(100, 50, 0)
                                    } else if cell.heat > 5.0 {
                                        Color::DarkGray
                                    } else {
                                        continue;
                                    }
                                }
                            };

                            points_by_color
                                .entry(color)
                                .or_default()
                                .push((x as f64, height as f64 - y as f64 - 1.0));
                        }
                    }

                    for (color, coords) in points_by_color {
                        ctx.draw(&Points {
                            coords: &coords,
                            color,
                        });
                    }

                    // Draw termites
                    for termite in &world.termites {
                        let color = if termite.carrying {
                            Color::Green
                        } else {
                            Color::Blue
                        };
                        ctx.print(
                            termite.x as f64,
                            height as f64 - termite.y as f64 - 1.0,
                            ratatui::text::Span::styled("t", Style::default().fg(color)),
                        );
                    }
                });

            f.render_widget(canvas, size);
        })?;
    }

    Ok(())
}
