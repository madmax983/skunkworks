use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{
        canvas::{Canvas, Line},
        Block, Borders,
    },
};
use std::{
    f64::consts::PI,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    time::{Duration, Instant},
};

struct CodeLine {
    indent: usize,
    length: usize,
    color: Color,
    #[allow(dead_code)]
    content: String,
}

struct App {
    should_quit: bool,
    lines: Vec<CodeLine>,
    rotation: f64,
    scroll: f64,
    auto_rotate: bool,
    auto_scroll: bool,
}

impl App {
    fn new(path: &str) -> Result<Self> {
        let lines = load_file(path)
            .or_else(|_| load_file("experiments/code-kaleidoscope/src/main.rs"))
            .or_else(|_| load_file("src/main.rs"))
            .unwrap_or_else(|_| {
                // Final fallback if nothing found
                vec![CodeLine {
                    indent: 0,
                    length: 10,
                    color: Color::Red,
                    content: "No source found".into(),
                }]
            });

        Ok(Self {
            should_quit: false,
            lines,
            rotation: 0.0,
            scroll: 0.0,
            auto_rotate: true,
            auto_scroll: true,
        })
    }

    fn update(&mut self) {
        if self.auto_rotate {
            self.rotation += 0.005;
        }
        if self.auto_scroll {
            self.scroll += 0.05;
        }
    }
}

fn load_file<P: AsRef<Path>>(path: P) -> Result<Vec<CodeLine>> {
    let file = File::open(path).context("Failed to open file")?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let indent = line.chars().take_while(|c| c.is_whitespace()).count();
        let trimmed = line.trim();
        let length = trimmed.len();

        let color = if trimmed.starts_with("use") || trimmed.starts_with("mod") {
            Color::Red
        } else if trimmed.starts_with("fn") {
            Color::Blue
        } else if trimmed.starts_with("struct") || trimmed.starts_with("enum") {
            Color::Yellow
        } else if trimmed.starts_with("impl") {
            Color::Cyan
        } else if trimmed.starts_with("let") || trimmed.starts_with("const") {
            Color::Green
        } else if trimmed.starts_with("//") {
            Color::DarkGray
        } else {
            Color::White
        };

        lines.push(CodeLine {
            indent,
            length,
            color,
            content: line,
        });
    }

    if lines.is_empty() {
        // Dummy data if empty
        lines.push(CodeLine {
            indent: 0,
            length: 10,
            color: Color::Red,
            content: "Empty".into(),
        });
    }

    Ok(lines)
}

fn main() -> Result<()> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Init App
    // Try to load self
    let mut app = App::new("experiments/code-kaleidoscope/src/main.rs")?;

    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                        KeyCode::Char(' ') => app.auto_rotate = !app.auto_rotate,
                        KeyCode::Char('s') => app.auto_scroll = !app.auto_scroll,
                        KeyCode::Up => app.scroll -= 1.0,
                        KeyCode::Down => app.scroll += 1.0,
                        KeyCode::Left => app.rotation -= 0.1,
                        KeyCode::Right => app.rotation += 0.1,
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let size = f.area();

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Code Kaleidoscope 💠 (Space: Rotate, S: Scroll) "),
        )
        .x_bounds([-150.0, 150.0])
        .y_bounds([-150.0, 150.0])
        .paint(|ctx| {
            let center_x = 0.0;
            let center_y = 0.0;
            let max_radius = 140.0;
            let sectors = 8;
            let sector_angle = 2.0 * PI / sectors as f64;

            // Render a window of lines
            let window_size = 60;
            let start_idx = (app.scroll as usize) % app.lines.len();

            for i in 0..window_size {
                let line_idx = (start_idx + i) % app.lines.len();
                let line = &app.lines[line_idx];

                // Map line properties to geometry
                // r = distance from center
                let r = (i as f64 / window_size as f64) * max_radius;

                // Theta within the sector
                // Base angle on indentation + length
                let local_theta = ((line.indent as f64 * 0.1) + (line.length as f64 * 0.01))
                    % (sector_angle / 2.0);

                let stroke_len = (line.length as f64 * 0.5).min(20.0);

                // For each sector
                for s in 0..sectors {
                    let base_angle = s as f64 * sector_angle + app.rotation;

                    // Draw Original
                    draw_mirrored_line(
                        ctx,
                        center_x,
                        center_y,
                        r,
                        base_angle + local_theta,
                        stroke_len,
                        line.color,
                    );

                    // Draw Mirror (within the sector)
                    draw_mirrored_line(
                        ctx,
                        center_x,
                        center_y,
                        r,
                        base_angle + sector_angle - local_theta,
                        stroke_len,
                        line.color,
                    );
                }
            }
        });

    f.render_widget(canvas, size);
}

fn calculate_line_points(
    cx: f64,
    cy: f64,
    r: f64,
    theta: f64,
    len: f64,
) -> ((f64, f64), (f64, f64)) {
    let x1 = cx + r * theta.cos();
    let y1 = cy + r * theta.sin();

    // Tangential-ish twist
    let x2 = cx + r * (theta + 0.05).cos() + len * theta.sin();
    let y2 = cy + r * (theta + 0.05).sin() - len * theta.cos();

    ((x1, y1), (x2, y2))
}

fn draw_mirrored_line(
    ctx: &mut ratatui::widgets::canvas::Context,
    cx: f64,
    cy: f64,
    r: f64,
    theta: f64,
    len: f64,
    color: Color,
) {
    let ((x1, y1), (x2, y2)) = calculate_line_points(cx, cy, r, theta, len);

    ctx.draw(&Line {
        x1,
        y1,
        x2,
        y2,
        color,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_rotation_symmetry() {
        let cx = 0.0;
        let cy = 0.0;
        let r = 10.0;
        let len = 5.0;

        // Point at angle 0
        let ((x1_0, y1_0), _) = calculate_line_points(cx, cy, r, 0.0, len);

        // Point at angle PI (180 deg)
        let ((x1_pi, y1_pi), _) = calculate_line_points(cx, cy, r, PI, len);

        // x at PI should be -x at 0
        assert!((x1_0 + x1_pi).abs() < 1e-10);
        // y at PI should be -y at 0 (approx 0)
        assert!((y1_0 + y1_pi).abs() < 1e-10);
    }

    #[test]
    fn test_point_on_circle() {
        let cx = 10.0;
        let cy = 10.0;
        let r = 5.0;
        let theta = 0.0;
        let len = 0.0;

        let ((x1, y1), _) = calculate_line_points(cx, cy, r, theta, len);

        // At theta 0, point should be at cx + r, cy
        assert!((x1 - (cx + r)).abs() < 1e-10);
        assert!((y1 - cy).abs() < 1e-10);
    }
}
