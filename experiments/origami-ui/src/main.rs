use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use nalgebra::{Rotation3, Vector3};
use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{
        canvas::{Canvas, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

mod miura;
use miura::MiuraGrid;

struct Panel {
    title: String,
    content: String,
}

struct App {
    grid: MiuraGrid,
    panels: Vec<Panel>,
    rotation_y: f64,
    rotation_x: f64,
    auto_rotate: bool,
}

impl App {
    fn new() -> Self {
        // Create some dummy panels
        let panels = (0..20)
            .map(|i| Panel {
                title: format!("Panel {:02}", i),
                content: format!(
                    "This is the content for panel {}.\nIt contains important data.",
                    i
                ),
            })
            .collect();

        Self {
            grid: MiuraGrid::new(5, 5), // 4x4 faces
            panels,
            rotation_y: 0.0,
            rotation_x: 0.2, // Slight tilt to show depth
            auto_rotate: false,
        }
    }

    fn update(&mut self, dt: f64) {
        if self.auto_rotate {
            self.rotation_y += dt * 0.5;
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

        // Responsive Logic: Calculate t based on width
        // Assume 'flat' width is around 100 cols.
        // If width < 60, we fold up.
        let max_width = 120.0;
        let min_width = 60.0;
        let current_width = area.width as f64;

        // t goes from 0.0 (flat) to 0.9 (folded)
        // larger width -> smaller t
        let t = if current_width >= max_width {
            0.0
        } else if current_width <= min_width {
            0.9
        } else {
            0.9 * (1.0 - (current_width - min_width) / (max_width - min_width))
        };

        let vertices = self.grid.compute_vertices(t);

        // Rotation matrices
        let rot = Rotation3::from_axis_angle(&Vector3::y_axis(), self.rotation_y)
            * Rotation3::from_axis_angle(&Vector3::x_axis(), self.rotation_x);

        // Project vertices
        let projected: Vec<(f64, f64)> = vertices
            .iter()
            .map(|v| {
                let rotated = rot * v;
                // Orthographic projection with scaling
                // Scale factor needs to adapt to fit screen?
                // Let's just use 1.0 for now and rely on Canvas bounds
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

        let width = (max_x - min_x).max(1.0) * 1.1;
        let height = (max_y - min_y).max(1.0) * 1.1;
        let center_x = (min_x + max_x) / 2.0;
        let center_y = (min_y + max_y) / 2.0;

        // Canvas coordinate system
        let x_bounds = [center_x - width / 2.0, center_x + width / 2.0];
        let y_bounds = [center_y - height / 2.0, center_y + height / 2.0];

        // 1. Draw Grid Lines via Canvas
        frame.render_widget(
            Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!("Origami UI (Width: {}, t: {:.2})", area.width, t)),
                )
                .x_bounds(x_bounds)
                .y_bounds(y_bounds)
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
                                let color = if r % 2 == 0 { Color::Cyan } else { Color::Blue };
                                ctx.draw(&CanvasLine {
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
                                ctx.draw(&CanvasLine {
                                    x1,
                                    y1,
                                    x2,
                                    y2,
                                    color,
                                });
                            }
                        }
                    }
                }),
            area,
        );

        // 2. Overlay Content
        // To do this, we need to map canvas coords to screen coords.
        // Canvas logic:
        // screen_x = area.left() + (x - x_bounds[0]) / (x_bounds[1] - x_bounds[0]) * area.width
        // screen_y = area.bottom() - (y - y_bounds[0]) / (y_bounds[1] - y_bounds[0]) * area.height
        // Note: Canvas Y is up, Screen Y is down.

        let map_to_screen = |x: f64, y: f64| -> Option<(u16, u16)> {
            let nx = (x - x_bounds[0]) / (x_bounds[1] - x_bounds[0]);
            let ny = (y - y_bounds[0]) / (y_bounds[1] - y_bounds[0]);

            if nx < 0.0 || nx > 1.0 || ny < 0.0 || ny > 1.0 {
                return None;
            }

            let sx = area.x as f64 + nx * (area.width as f64 - 1.0);
            let sy = (area.y as f64 + area.height as f64 - 1.0) - ny * (area.height as f64 - 1.0);

            Some((sx as u16, sy as u16))
        };

        // Draw panels on faces
        let rows = self.grid.rows;
        let cols = self.grid.cols;
        let mut panel_idx = 0;

        for r in 0..rows - 1 {
            for c in 0..cols - 1 {
                if panel_idx >= self.panels.len() {
                    break;
                }
                let panel = &self.panels[panel_idx];

                // Get face vertices
                if let Some(indices) = self.grid.get_face_indices(r, c) {
                    // Average position (centroid)
                    let mut cx = 0.0;
                    let mut cy = 0.0;
                    for &idx in &indices {
                        cx += projected[idx].0;
                        cy += projected[idx].1;
                    }
                    cx /= 4.0;
                    cy /= 4.0;

                    if let Some((sx, sy)) = map_to_screen(cx, cy) {
                        // Determine size roughly
                        // Compute width of face in screen space
                        let p0 = projected[indices[0]]; // Top Left
                        let p1 = projected[indices[1]]; // Top Right

                        // Roughly...
                        let dx = (p0.0 - p1.0).hypot(p0.1 - p1.1);
                        // Convert model length to screen length
                        let screen_len_x = dx / (x_bounds[1] - x_bounds[0]) * area.width as f64;

                        // If it's big enough, show content
                        if screen_len_x > 10.0 {
                            let rect_width = (screen_len_x * 0.8) as u16;
                            let rect_height = 3; // minimal height

                            let rect_x = sx.saturating_sub(rect_width / 2);
                            let rect_y = sy.saturating_sub(rect_height / 2);

                            let rect = Rect::new(rect_x, rect_y, rect_width, rect_height);

                            // Clip to screen
                            let rect = rect.intersection(area);

                            // Only draw if we have space
                            if rect.width > 4 && rect.height > 1 {
                                // If t is high (folded), just show title
                                let text = if t > 0.5 {
                                    vec![Line::from(panel.title.as_str()).bold()]
                                } else {
                                    vec![
                                        Line::from(panel.title.as_str()).bold().bg(Color::Blue),
                                        Line::from(panel.content.as_str()),
                                    ]
                                };

                                let p =
                                    Paragraph::new(text).style(Style::default().fg(Color::White));
                                frame.render_widget(p, rect);
                            }
                        }
                    }
                }
                panel_idx += 1;
            }
        }
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();
    let mut last_tick = Instant::now();

    loop {
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char(' ') => app.auto_rotate = !app.auto_rotate,
                    KeyCode::Left => app.rotation_y -= 0.1,
                    KeyCode::Right => app.rotation_y += 0.1,
                    KeyCode::Up => app.rotation_x -= 0.1,
                    KeyCode::Down => app.rotation_x += 0.1,
                    _ => {}
                }
            }
        }

        let now = Instant::now();
        let dt = now.duration_since(last_tick).as_secs_f64();
        last_tick = now;
        app.update(dt);

        tui.terminal.draw(|f| app.draw(f))?;
    }

    tui.exit()?;
    Ok(())
}
