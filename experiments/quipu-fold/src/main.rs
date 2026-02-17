use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{
        canvas::{Canvas, Line as CanvasLine, Points},
        Block, Borders, Paragraph,
    },
    Frame, Terminal,
};
use std::io;
use std::time::{Duration, Instant};
use rand::Rng;

mod geo;
use geo::MiuraPattern;

use quipu::Cord;
use nalgebra::{Rotation3, Vector3};

struct App {
    pattern: MiuraPattern,
    rho: f64,        // Fold state
    target_rho: f64,
    velocity: f64,
    should_quit: bool,
    rotation: (f64, f64),
    cords: Vec<Cord>,
}

impl App {
    fn new() -> Self {
        let rows = 8;
        let cols = 8;
        let mut rng = rand::thread_rng();
        let mut cords = Vec::new();
        for _ in 0..cols {
            // Generate random value with variable number of digits
            // Ensure some fit within 8 digits (up to 99,999,999)
            let val = rng.gen_range(100..99_999_999);
            cords.push(Cord::from(val));
        }

        Self {
            pattern: MiuraPattern::new(rows, cols),
            rho: 0.8,
            target_rho: 0.8,
            velocity: 0.0,
            should_quit: false,
            rotation: (0.5, 0.5),
            cords,
        }
    }

    fn update(&mut self) {
        // Spring Physics for smooth folding animation
        let k = 0.08;
        let c = 0.15;
        let diff = self.target_rho - self.rho;
        let force = diff * k;

        self.velocity += force;
        self.velocity *= 1.0 - c;
        self.rho += self.velocity;

        // Hard stops
        if self.rho < 0.0 { self.rho = 0.0; self.velocity *= -0.5; }
        else if self.rho > 1.0 { self.rho = 1.0; self.velocity *= -0.5; }
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

    // Project to 2D
    let projected: Vec<(f64, f64)> = vertices
        .iter()
        .map(|v| {
            let rv = rot * v;
            (rv.x, rv.y) // Ratatui Canvas: Y is up
        })
        .collect();

    // Determine bounds
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

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(" Quipu Fold 🧶 "))
        .x_bounds(x_bounds)
        .y_bounds(y_bounds)
        .paint(|ctx| {
            let rows = app.pattern.rows;
            let cols = app.pattern.cols;

            // Draw Mesh
            for r in 0..rows {
                for c in 0..cols {
                    let idx = r * cols + c;
                    let (x1, y1) = projected[idx];

                    // Draw Horizontal connections (Mountain/Valley)
                    if c + 1 < cols {
                        let idx2 = r * cols + (c + 1);
                        let (x2, y2) = projected[idx2];
                        let color = if (r+c)%2 == 0 { Color::Red } else { Color::Blue };
                        ctx.draw(&CanvasLine { x1, y1, x2, y2, color });
                    }

                    // Draw Vertical connections (Cords)
                    if r + 1 < rows {
                        let idx2 = (r + 1) * cols + c;
                        let (x2, y2) = projected[idx2];
                        let color = Color::Yellow; // Cords are yellow
                        ctx.draw(&CanvasLine { x1, y1, x2, y2, color });
                    }
                }
            }

            // Draw Knots
            for c in 0..cols {
                if c >= app.cords.len() { continue; }
                let cord = &app.cords[c];

                // Map Cord clusters to rows.
                // Row (rows-1) is Units (cluster 0).
                // Row 0 is highest power (cluster rows-1).

                for (k_idx, cluster) in cord.clusters.iter().enumerate() {
                    // k_idx 0 = units -> should be at row = rows - 1.
                    if k_idx >= rows { break; } // Cord too long for grid

                    let r = rows - 1 - k_idx;
                    let idx = r * cols + c;
                    let (x, y) = projected[idx];

                    // Draw cluster of knots
                    if !cluster.is_empty() {
                         // Simplify: Just draw a dot with color based on value sum
                         let val: u8 = cluster.iter().map(|k| k.value()).sum();
                         let color = match val {
                             0 => Color::Gray,
                             1..=4 => Color::Green,
                             5..=9 => Color::Magenta,
                             _ => Color::White,
                         };

                         // Draw a point/marker
                         // Ratatui canvas Points takes iterator
                         ctx.draw(&Points {
                             coords: &[(x, y)],
                             color,
                         });

                         // Or label it if possible? Canvas doesn't support text easily.
                         // But we can infer value from color/brightness or just the fact there is a knot.
                    }
                }
            }
        });

    f.render_widget(canvas, canvas_area);

    // Status Bar
    let status_text = format!(
        "Rho: {:.2} | Controls: Arrow Keys (Fold/Unfold), WASD (Rotate), Q (Quit)",
        app.rho
    );
    f.render_widget(
        Paragraph::new(status_text).block(Block::default().borders(Borders::ALL)),
        chunks[1],
    );
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            app.update();
            draw_ui(f, &app);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
             if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                    KeyCode::Left => app.target_rho = (app.target_rho - 0.1).clamp(0.0, 1.0),
                    KeyCode::Right => app.target_rho = (app.target_rho + 0.1).clamp(0.0, 1.0),
                    KeyCode::Up => app.target_rho = (app.target_rho + 0.1).clamp(0.0, 1.0),
                    KeyCode::Down => app.target_rho = (app.target_rho - 0.1).clamp(0.0, 1.0),
                    KeyCode::Char('w') => app.rotation.0 -= 0.1,
                    KeyCode::Char('s') => app.rotation.0 += 0.1,
                    KeyCode::Char('a') => app.rotation.1 -= 0.1,
                    KeyCode::Char('d') => app.rotation.1 += 0.1,
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
