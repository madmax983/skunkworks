use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{
        canvas::{Canvas, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
    Frame, Terminal,
};
use std::io;
use std::time::{Duration, Instant};

mod geo;
use geo::MiuraPattern;

// 3D Math helpers
use nalgebra::{Rotation3, Vector3};

struct Panel {
    content: String,
    color: Color,
}

struct App {
    pattern: MiuraPattern,
    rho: f64, // 0.0 (Folded) to 1.0 (Flat)
    should_quit: bool,
    auto_responsive: bool,
    rotation: (f64, f64), // (pitch, yaw)
    panels: Vec<Panel>,
}

impl App {
    fn new() -> Self {
        let mut panels = Vec::new();
        // Create some data
        let colors = [
            Color::Red,
            Color::Green,
            Color::Yellow,
            Color::Blue,
            Color::Magenta,
            Color::Cyan,
        ];
        for i in 0..100 {
            panels.push(Panel {
                content: format!("ID:{:02}", i),
                color: colors[i % colors.len()],
            });
        }

        Self {
            pattern: MiuraPattern::new(8, 8),
            rho: 0.8,
            should_quit: false,
            auto_responsive: true,
            rotation: (0.5, 0.5),
            panels,
        }
    }

    fn update(&mut self, area: Rect) {
        if self.auto_responsive {
            // Map width to rho
            // Let's say max width 120 -> rho 1.0
            // min width 40 -> rho 0.0
            let w = area.width as f64;
            let min_w = 40.0;
            let max_w = 120.0;
            self.rho = ((w - min_w) / (max_w - min_w)).clamp(0.0, 1.0);
        }
    }

    fn handle_event(&mut self, event: Event) {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                KeyCode::Char(' ') => self.auto_responsive = !self.auto_responsive,
                KeyCode::Left => {
                    self.auto_responsive = false;
                    self.rho = (self.rho - 0.05).clamp(0.0, 1.0)
                }
                KeyCode::Right => {
                    self.auto_responsive = false;
                    self.rho = (self.rho + 0.05).clamp(0.0, 1.0)
                }
                KeyCode::Char('w') => self.rotation.0 -= 0.1,
                KeyCode::Char('s') => self.rotation.0 += 0.1,
                KeyCode::Char('a') => self.rotation.1 -= 0.1,
                KeyCode::Char('d') => self.rotation.1 += 0.1,
                _ => {}
            }
        }
    }
}

fn draw_ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let canvas_area = chunks[0];

    // Compute vertices
    let vertices = app.pattern.compute_vertices(app.rho);

    // Rotation
    let rot = Rotation3::from_axis_angle(&Vector3::x_axis(), app.rotation.0)
        * Rotation3::from_axis_angle(&Vector3::y_axis(), app.rotation.1);

    // Project
    let projected: Vec<(f64, f64)> = vertices
        .iter()
        .map(|v| {
            let rv = rot * v;
            // Simple orthographic for now, scale fits to canvas
            (rv.x, rv.y) // In Ratatui Canvas, Y is up
        })
        .collect();

    // Determine bounds to center
    let (min_x, max_x, min_y, max_y) = projected.iter().fold(
        (f64::MAX, f64::MIN, f64::MAX, f64::MIN),
        |(minx, maxx, miny, maxy), (x, y)| (minx.min(*x), maxx.max(*x), miny.min(*y), maxy.max(*y)),
    );

    let width = (max_x - min_x).max(1.0) * 1.2;
    let height = (max_y - min_y).max(1.0) * 1.2;
    let cx = (min_x + max_x) / 2.0;
    let cy = (min_y + max_y) / 2.0;

    let x_bounds = [cx - width / 2.0, cx + width / 2.0];
    let y_bounds = [cy - height / 2.0, cy + height / 2.0];

    // Helper to map 3D projected coords to Screen coords
    let to_screen = |x: f64, y: f64| -> Option<(u16, u16)> {
        // Normalize 0..1
        let nx = (x - x_bounds[0]) / (x_bounds[1] - x_bounds[0]);
        let ny = (y - y_bounds[0]) / (y_bounds[1] - y_bounds[0]);

        if nx < 0.0 || nx > 1.0 || ny < 0.0 || ny > 1.0 {
            return None;
        }

        // Map to area
        let sx = canvas_area.x as f64 + nx * (canvas_area.width as f64 - 1.0);
        // Invert Y for screen
        let sy = (canvas_area.y as f64 + canvas_area.height as f64 - 1.0)
            - ny * (canvas_area.height as f64 - 1.0);

        Some((sx as u16, sy as u16))
    };

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Origami Layout Engine "),
        )
        .x_bounds(x_bounds)
        .y_bounds(y_bounds)
        .paint(|ctx| {
            // Draw edges
            let rows = app.pattern.rows;
            let cols = app.pattern.cols;

            for r in 0..rows {
                for c in 0..cols {
                    let idx = r * cols + c;
                    let (x1, y1) = projected[idx];

                    // Right
                    if c + 1 < cols {
                        let idx2 = r * cols + (c + 1);
                        let (x2, y2) = projected[idx2];
                        ctx.draw(&CanvasLine {
                            x1,
                            y1,
                            x2,
                            y2,
                            color: Color::DarkGray,
                        });
                    }

                    // Down
                    if r + 1 < rows {
                        let idx2 = (r + 1) * cols + c;
                        let (x2, y2) = projected[idx2];
                        ctx.draw(&CanvasLine {
                            x1,
                            y1,
                            x2,
                            y2,
                            color: Color::DarkGray,
                        });
                    }
                }
            }
        });

    f.render_widget(canvas, canvas_area);

    // Overlay Panels
    let mut panel_idx = 0;
    for r in 0..app.pattern.rows - 1 {
        for c in 0..app.pattern.cols - 1 {
            if panel_idx >= app.panels.len() {
                break;
            }

            // Get Quad vertices
            if let Some(indices) = app.pattern.get_quad_indices(r, c) {
                // Calculate Centroid
                let mut cx_w = 0.0;
                let mut cy_w = 0.0;
                for &i in &indices {
                    cx_w += projected[i].0;
                    cy_w += projected[i].1;
                }
                cx_w /= 4.0;
                cy_w /= 4.0;

                if let Some((sx, sy)) = to_screen(cx_w, cy_w) {
                    // Check if face is big enough to draw
                    let p0 = projected[indices[0]];
                    let p1 = projected[indices[1]]; // Horizontal neighbor
                                                    // dist in world
                    let dist = (p0.0 - p1.0).hypot(p0.1 - p1.1);
                    // dist in screen pixels
                    let screen_width =
                        dist / (x_bounds[1] - x_bounds[0]) * canvas_area.width as f64;

                    if screen_width > 4.0 {
                        let rect_w = (screen_width * 0.8) as u16;
                        let rect_h = 1;

                        let rx = sx.saturating_sub(rect_w / 2);
                        let ry = sy;

                        // Clip
                        let visible_rect =
                            Rect::new(rx, ry, rect_w, rect_h).intersection(canvas_area);

                        if visible_rect.width > 0 && visible_rect.height > 0 {
                            let panel = &app.panels[panel_idx];
                            // If folded tight, just show color block
                            if screen_width < 8.0 {
                                let b = Block::default().bg(panel.color);
                                f.render_widget(b, visible_rect);
                            } else {
                                // Show text
                                let p = Paragraph::new(panel.content.as_str())
                                    .style(Style::default().fg(Color::Black).bg(panel.color));
                                f.render_widget(p, visible_rect);
                            }
                        }
                    }
                }
            }
            panel_idx += 1;
        }
    }

    let status_text = format!(
        "Rho: {:.2} | Auto: {} | WASD Rotate, Arrows Fold",
        app.rho, app.auto_responsive
    );
    f.render_widget(
        Paragraph::new(status_text).block(Block::default().borders(Borders::ALL)),
        chunks[1],
    );
}

fn main() -> Result<()> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let tick_rate = Duration::from_millis(30);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            // Update logic here inside draw because we need frame size
            app.update(f.area());
            draw_ui(f, &app);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            app.handle_event(event::read()?);
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    // Restore Terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
