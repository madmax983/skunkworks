use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use nalgebra::{Rotation3, Vector3};
use ratatui::{
    style::Color,
    widgets::{
        canvas::{Canvas, Line},
        Block, Borders,
    },
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

mod miura;
use miura::MiuraGrid;

struct App {
    grid: MiuraGrid,
    t: f64,         // Fold parameter [0, 1]
    direction: f64, // +1 or -1
    speed: f64,
    paused: bool,
    rotation_y: f64,
    rotation_x: f64,
}

impl App {
    fn new() -> Self {
        Self {
            grid: MiuraGrid::new(15, 15),
            t: 0.0,
            direction: 1.0, // units per second
            speed: 0.5,
            paused: false,
            rotation_y: 45.0_f64.to_radians(),
            rotation_x: 35.0_f64.to_radians(),
        }
    }

    fn update(&mut self, dt: f64) {
        if !self.paused {
            self.t += self.direction * self.speed * dt;
            if self.t >= 1.0 {
                self.t = 1.0;
                self.direction = -1.0;
            } else if self.t <= 0.0 {
                self.t = 0.0;
                self.direction = 1.0;
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();
        let vertices = self.grid.compute_vertices(self.t);

        // Rotation matrices
        let rot = Rotation3::from_axis_angle(&Vector3::y_axis(), self.rotation_y)
            * Rotation3::from_axis_angle(&Vector3::x_axis(), self.rotation_x);

        // Project vertices
        let projected: Vec<(f64, f64)> = vertices
            .iter()
            .map(|v| {
                let rotated = rot * v;
                // Orthographic projection
                (rotated.x, rotated.y)
            })
            .collect();

        // Calculate bounds for dynamic scaling
        let (min_x, max_x, min_y, max_y) = projected.iter().fold(
            (
                f64::INFINITY,
                f64::NEG_INFINITY,
                f64::INFINITY,
                f64::NEG_INFINITY,
            ),
            |(mix, max, miy, may), (x, y)| (mix.min(*x), max.max(*x), miy.min(*y), may.max(*y)),
        );

        // Add some padding
        let width = (max_x - min_x).max(1.0) * 1.2;
        let height = (max_y - min_y).max(1.0) * 1.2;
        let center_x = (min_x + max_x) / 2.0;
        let center_y = (min_y + max_y) / 2.0;

        let canvas = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Origami Singularity"),
            )
            .x_bounds([center_x - width / 2.0, center_x + width / 2.0])
            .y_bounds([center_y - height / 2.0, center_y + height / 2.0])
            .paint(|ctx| {
                let rows = self.grid.rows;
                let cols = self.grid.cols;

                for r in 0..rows {
                    for c in 0..cols {
                        let idx = r * cols + c;
                        let (x1, y1) = projected[idx];

                        // Draw horizontal connection (to right)
                        if c + 1 < cols {
                            let idx_right = r * cols + (c + 1);
                            let (x2, y2) = projected[idx_right];
                            // Mountain or valley?
                            let color = if r % 2 == 0 { Color::Cyan } else { Color::Blue };
                            ctx.draw(&Line {
                                x1,
                                y1,
                                x2,
                                y2,
                                color,
                            });
                        }

                        // Draw vertical connection (down)
                        if r + 1 < rows {
                            let idx_down = (r + 1) * cols + c;
                            let (x2, y2) = projected[idx_down];
                            let color = if c % 2 == 0 {
                                Color::Magenta
                            } else {
                                Color::Red
                            };
                            ctx.draw(&Line {
                                x1,
                                y1,
                                x2,
                                y2,
                                color,
                            });
                        }
                    }
                }
            });

        frame.render_widget(canvas, area);
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();
    let mut last_tick = Instant::now();

    loop {
        // Event handling
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char(' ') => app.paused = !app.paused,
                    KeyCode::Left => app.rotation_y -= 0.1,
                    KeyCode::Right => app.rotation_y += 0.1,
                    KeyCode::Up => app.rotation_x -= 0.1,
                    KeyCode::Down => app.rotation_x += 0.1,
                    _ => {}
                }
            }
        }

        // Update
        let now = Instant::now();
        let dt = now.duration_since(last_tick).as_secs_f64();
        last_tick = now;
        app.update(dt);

        // Draw
        tui.terminal.draw(|f| app.draw(f))?;
    }

    tui.exit()?;
    Ok(())
}
