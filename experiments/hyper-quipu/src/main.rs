use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use hyper_system::{math::Vec4, monitor::SystemMonitor};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Line},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

struct Knot {
    pos: Vec4,
    load: f32, // 0.0 to 1.0 (actually sysinfo returns 0-100)
}

struct Cord {
    knots: Vec<Knot>,
    color: Color,
    base_angle: f32, // Angle in circle layout
}

struct App {
    monitor: SystemMonitor,
    cords: Vec<Cord>,

    // 4D Transform
    rotation_xw: f32,
    rotation_yw: f32,
    rotation_zw: f32,
    camera_w: f32,
    zoom: f32,

    // Time
    start_time: Instant,
    last_tick: Instant,
    paused: bool,
}

impl App {
    fn new() -> Self {
        Self {
            monitor: SystemMonitor::new(),
            cords: Vec::new(),
            rotation_xw: 0.0,
            rotation_yw: 0.0,
            rotation_zw: 0.0,
            camera_w: 5.0,
            zoom: 1.0,
            start_time: Instant::now(),
            last_tick: Instant::now(),
            paused: false,
        }
    }

    fn update(&mut self) {
        if self.paused {
            return;
        }

        let now = Instant::now();
        if now.duration_since(self.last_tick) < Duration::from_millis(100) {
            return;
        }
        self.last_tick = now;

        // Manual update of monitor since we are not using macroquad
        let time_secs = self.start_time.elapsed().as_secs_f64();
        self.monitor.update_with_time(0.1, time_secs);

        // Get per-core usage
        // Note: sysinfo returns usage as 0.0 to 100.0 usually? Or 0.0 to 1.0?
        // documentation says 0..100 usually.
        let cpus = self.monitor.sys.cpus();

        // Initialize cords if needed
        if self.cords.len() != cpus.len() {
            self.cords.clear();
            let n = cpus.len();
            for i in 0..n {
                let angle = (i as f32 / n as f32) * std::f32::consts::TAU;
                let color = match i % 6 {
                    0 => Color::Red,
                    1 => Color::Green,
                    2 => Color::Yellow,
                    3 => Color::Blue,
                    4 => Color::Magenta,
                    5 => Color::Cyan,
                    _ => Color::White,
                };
                self.cords.push(Cord {
                    knots: Vec::new(),
                    color,
                    base_angle: angle,
                });
            }
        }

        let time_w = self.start_time.elapsed().as_secs_f32();

        // Add knots
        for (i, cpu) in cpus.iter().enumerate() {
            if i >= self.cords.len() {
                break;
            }

            let load = cpu.cpu_usage() / 100.0; // Normalize to 0.0 - 1.0
            let cord = &mut self.cords[i];

            // Base position in circle
            let radius = 1.0 + load * 0.5; // High load expands outwards
            let x = cord.base_angle.cos() * radius;
            let y = cord.base_angle.sin() * radius;
            let z = (load - 0.5) * 0.5; // Z varies slightly
            let w = time_w;

            cord.knots.push(Knot {
                pos: Vec4::new(x, y, z, w),
                load,
            });

            // Prune old knots (keep last 100)
            if cord.knots.len() > 100 {
                cord.knots.remove(0);
            }
        }

        // Auto-rotate slowly
        self.rotation_xw += 0.01;
        self.rotation_zw += 0.005;

        // Keep camera following time
        self.camera_w = time_w + 5.0;
    }
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ])
                .split(f.area());

            // Header
            let title = Paragraph::new("🧬 HYPER-QUIPU: 4D System Monitor")
                .style(Style::default().fg(Color::Cyan).bg(Color::Black))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);

            // Canvas
            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("4D Viewport"))
                .x_bounds([-2.0, 2.0])
                .y_bounds([-2.0, 2.0])
                .paint(|ctx| {
                    for cord in &app.cords {
                        let mut prev_point: Option<(f64, f64)> = None;

                        for knot in &cord.knots {
                            // Transform
                            // 1. Relative W
                            let mut p = knot.pos;

                            // Center W relative to camera
                            let center_w = app.camera_w - 5.0;
                            p.w -= center_w;

                            // 2. Rotate
                            p = p.rotate_xw(app.rotation_xw);
                            p = p.rotate_yw(app.rotation_yw);
                            p = p.rotate_zw(app.rotation_zw);

                            // 3. Project to 3D
                            // Camera is at W = 5.0 (relative)
                            let p3 = p.project_to_3d(5.0);

                            // 4. Project to 2D
                            // Simple perspective: x / (z + dist)
                            let dist = 3.0 / app.zoom;
                            // Avoid division by zero
                            let z_factor = 1.0 / (p3.z + dist).max(0.1);
                            let x2 = p3.x * z_factor;
                            let y2 = p3.y * z_factor;

                            // Draw
                            if let Some((px, py)) = prev_point {
                                ctx.draw(&Line {
                                    x1: px,
                                    y1: py,
                                    x2: x2 as f64,
                                    y2: y2 as f64,
                                    color: cord.color,
                                });
                            }
                            prev_point = Some((x2 as f64, y2 as f64));
                        }
                    }
                });
            f.render_widget(canvas, chunks[1]);

            // Footer
            let stats = format!(
                "CPU: {:.0}% | Load: {:.2} | Rot XW: {:.2} | Rot ZW: {:.2}",
                app.monitor.cpu_usage * 100.0,
                app.monitor.load_avg,
                app.rotation_xw,
                app.rotation_zw
            );
            let footer = Paragraph::new(stats)
                .style(Style::default().fg(Color::White))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(footer, chunks[2]);
        })?;

        // Input
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(' ') => app.paused = !app.paused,
                    KeyCode::Up => app.rotation_yw += 0.1,
                    KeyCode::Down => app.rotation_yw -= 0.1,
                    KeyCode::Left => app.rotation_xw -= 0.1,
                    KeyCode::Right => app.rotation_xw += 0.1,
                    KeyCode::Char('w') => app.zoom *= 1.1,
                    KeyCode::Char('s') => app.zoom *= 0.9,
                    _ => {}
                }
            }
        }

        app.update();
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
