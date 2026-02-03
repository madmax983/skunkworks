use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use num_complex::Complex;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{
        canvas::{Canvas, Context, Line},
        Block, Borders, Paragraph,
    },
    Frame, Terminal,
};
use std::f64::consts::PI;
use std::io::{self, Stdout};
use std::time::{Duration, Instant};

mod math;
mod tiling;

use math::{Geodesic, Mobius};
use tiling::Tiling;

struct App {
    tiling: Tiling,
    player_pos: Mobius, // Transformation from World to Player View?
    // Actually better: Transformation from Player Frame to World Frame (isometry).
    // View transform is inverse.
    running: bool,
}

impl App {
    fn new() -> Self {
        Self {
            tiling: Tiling::new_5_4(4), // Depth 4
            player_pos: Mobius::identity(),
            running: true,
        }
    }

    fn run(mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        let tick_rate = Duration::from_millis(16);
        let mut last_tick = Instant::now();

        while self.running {
            terminal.draw(|f| self.ui(f))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    self.handle_input(key);
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn handle_input(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        let move_step = 0.05;
        let rot_step = 0.1;

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.running = false,
            // Movement: apply translation to player_pos
            // In player frame, 'up' is +y (or whatever forward is).
            // We apply a small translation T_d on the right?
            // New pos = Old pos * T_d.
            // Moving forward (up): z -> z + i*step (approx)
            KeyCode::Char('w') | KeyCode::Up => {
                let m = Mobius::translation(Complex::new(0.0, move_step));
                self.player_pos = self.player_pos.compose(&m);
            }
            KeyCode::Char('s') | KeyCode::Down => {
                let m = Mobius::translation(Complex::new(0.0, -move_step));
                self.player_pos = self.player_pos.compose(&m);
            }
            KeyCode::Char('a') | KeyCode::Left => {
                let m = Mobius::translation(Complex::new(-move_step, 0.0));
                self.player_pos = self.player_pos.compose(&m);
            }
            KeyCode::Char('d') | KeyCode::Right => {
                let m = Mobius::translation(Complex::new(move_step, 0.0));
                self.player_pos = self.player_pos.compose(&m);
            }
            // Rotation:
            KeyCode::Char('e') => {
                let m = Mobius::rotation(-rot_step);
                self.player_pos = self.player_pos.compose(&m);
            }
            KeyCode::Char('r') => {
                let m = Mobius::rotation(rot_step);
                self.player_pos = self.player_pos.compose(&m);
            }

            _ => {}
        }
    }

    fn ui(&self, f: &mut Frame) {
        let area = f.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(area);

        // Calculate view transform
        let view_transform = self.player_pos.inverse();

        let canvas = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Poincaré Crawl"),
            )
            .x_bounds([-1.1, 1.1])
            .y_bounds([-1.1, 1.1])
            .paint(move |ctx| {
                // Draw boundary circle
                draw_circle(ctx, 0.0, 0.0, 1.0, Color::White);

                // Draw tiling
                for poly in &self.tiling.polygons {
                    let n = poly.vertices.len();
                    for i in 0..n {
                        let v1_world = poly.vertices[i];
                        let v2_world = poly.vertices[(i + 1) % n];

                        let v1 = view_transform.apply(v1_world);
                        let v2 = view_transform.apply(v2_world);

                        // Cull if outside visible range?
                        // Actually, points can be anywhere in unit disk.

                        draw_hyperbolic_segment(ctx, v1, v2, Color::Cyan);
                    }
                }

                // Draw player (center)
                ctx.print(0.0, 0.0, ratatui::text::Span::raw("@"));
            });

        f.render_widget(canvas, chunks[0]);

        f.render_widget(
            Paragraph::new("WASD to move (hyperbolic), Q/R to rotate. Esc to quit.")
                .block(Block::default().borders(Borders::ALL)),
            chunks[1],
        );
    }
}

// Re-implement circle drawing inside the paint closure or loop
fn draw_circle(ctx: &mut Context, cx: f64, cy: f64, r: f64, color: Color) {
    let steps = 64;
    for i in 0..steps {
        let t1 = 2.0 * PI * (i as f64) / (steps as f64);
        let t2 = 2.0 * PI * ((i + 1) as f64) / (steps as f64);
        let x1 = cx + r * t1.cos();
        let y1 = cy + r * t1.sin();
        let x2 = cx + r * t2.cos();
        let y2 = cy + r * t2.sin();
        ctx.draw(&Line {
            x1,
            y1,
            x2,
            y2,
            color,
        });
    }
}

fn draw_hyperbolic_segment(ctx: &mut Context, p1: Complex<f64>, p2: Complex<f64>, color: Color) {
    let geo = Geodesic::new(p1, p2);

    if let Some((center, radius)) = geo.euclidean_circle() {
        // Draw arc from p1 to p2
        // We need angles of p1 and p2 relative to center
        let ang1 = (p1 - center).arg();
        let ang2 = (p2 - center).arg();

        // Shortest arc?
        // Determine direction.
        // We can just interpolate along the arc.
        // Or simpler: interpolate in hyperbolic space (straight line in parameter) and map to euclidean.
        // But that requires inverse hyperbolic trig.

        // Let's just interpolate angles.
        // We need to know if we go clockwise or counter-clockwise.
        // The arc must be within the unit disk.

        let mut diff = ang2 - ang1;
        while diff > PI {
            diff -= 2.0 * PI;
        }
        while diff < -PI {
            diff += 2.0 * PI;
        }

        let steps = 10;
        for i in 0..steps {
            let t1 = ang1 + diff * (i as f64) / (steps as f64);
            let t2 = ang1 + diff * ((i + 1) as f64) / (steps as f64);

            let x1 = center.re + radius * t1.cos();
            let y1 = center.im + radius * t1.sin();
            let x2 = center.re + radius * t2.cos();
            let y2 = center.im + radius * t2.sin();

            // Clip to unit disk? (Should be inside by definition)
            ctx.draw(&Line {
                x1,
                y1,
                x2,
                y2,
                color,
            });
        }
    } else {
        // Straight line
        ctx.draw(&Line {
            x1: p1.re,
            y1: p1.im,
            x2: p2.re,
            y2: p2.im,
            color,
        });
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = app.run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(e) = res {
        eprintln!("Error: {:?}", e);
    }

    Ok(())
}
