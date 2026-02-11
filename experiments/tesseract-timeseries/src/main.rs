mod math4d;
mod geometry;

use std::time::{Duration, Instant};
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{canvas::{Canvas, Line as CanvasLine}, Block, Borders, Paragraph},
};

use math4d::{Rotor4, project_4d_to_3d};
use geometry::{tesseract, time_series, combine, Mesh4D};

struct App {
    mesh: Mesh4D,

    // 4D Rotation angles
    angle_xw: f32,
    angle_yw: f32,
    angle_zw: f32,

    // 3D Rotation angles (embedded in 4D rotor logic or separate?)
    // We'll treat them as part of the 4D orientation.
    angle_xy: f32,
    angle_xz: f32,
    angle_yz: f32,

    // Camera
    camera_w: f32, // Distance in 4th dimension

    should_quit: bool,
    auto_rotate: bool,
}

impl App {
    fn new() -> Self {
        // Create a tesseract and a time series inside it
        let tess = tesseract();
        // Time series from W=-1.0 to W=1.0
        let series = time_series(100, (-1.0, 1.0));
        let mesh = combine(vec![tess, series]);

        Self {
            mesh,
            angle_xw: 0.0,
            angle_yw: 0.0,
            angle_zw: 0.0,
            angle_xy: 0.0,
            angle_xz: 0.0,
            angle_yz: 0.0,
            camera_w: -3.0, // Camera at W = -3
            should_quit: false,
            auto_rotate: true,
        }
    }

    fn on_tick(&mut self) {
        if self.auto_rotate {
            self.angle_xw += 0.01;
            self.angle_yw += 0.013;
            self.angle_xy += 0.005;
        }
    }
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    std::io::stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new();
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => app.should_quit = true,
                        KeyCode::Esc => app.should_quit = true,

                        // 4D Rotations
                        KeyCode::Char('1') => app.angle_xw += 0.05,
                        KeyCode::Char('2') => app.angle_yw += 0.05,
                        KeyCode::Char('3') => app.angle_zw += 0.05,

                        // 3D Rotations
                        KeyCode::Left => app.angle_xy -= 0.05,
                        KeyCode::Right => app.angle_xy += 0.05,
                        KeyCode::Up => app.angle_yz -= 0.05, // Rotate around X (Y-Z plane)
                        KeyCode::Down => app.angle_yz += 0.05,

                        KeyCode::Char(' ') => app.auto_rotate = !app.auto_rotate,
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    std::io::stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    // Info Bar
    let info_text = vec![
        Line::from(vec![
            Span::styled("Tesseract Time Series", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" | "),
            Span::raw(format!("XW: {:.2} YW: {:.2} ZW: {:.2}", app.angle_xw, app.angle_yw, app.angle_zw)),
        ]),
        Line::from(vec![
            Span::raw("Arrows: Rotate 3D | 1/2/3: Rotate 4D Planes | Space: Pause | Q: Quit"),
        ]),
    ];

    f.render_widget(
        Paragraph::new(info_text).block(Block::default().borders(Borders::ALL)),
        chunks[1]
    );

    // 4D Rendering
    let _canvas_area = chunks[0];

    // Create Rotor
    let rotor = Rotor4::new(
        app.angle_xw, app.angle_yw, app.angle_zw,
        app.angle_xy, app.angle_xz, app.angle_yz
    );

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("4D Projection"))
        .x_bounds([-2.0, 2.0])
        .y_bounds([-2.0, 2.0]) // Keep aspect ratio roughly consistent if possible
        .paint(move |ctx| {
            // Draw Edges
            for (start_idx, end_idx) in &app.mesh.edges {
                let v1_4d = app.mesh.vertices[*start_idx];
                let v2_4d = app.mesh.vertices[*end_idx];

                // Rotate
                let r1 = rotor.transform(v1_4d);
                let r2 = rotor.transform(v2_4d);

                // Project 4D -> 3D
                let p1_3d = project_4d_to_3d(r1, app.camera_w);
                let p2_3d = project_4d_to_3d(r2, app.camera_w);

                // Project 3D -> 2D (Simple Orthographic for now, or Perspective)
                // Let's do simple perspective 3D->2D: x' = x/ (z + dist)
                let cam_dist_3d = 4.0;

                let x1 = p1_3d.x * (cam_dist_3d / (cam_dist_3d + p1_3d.z));
                let y1 = p1_3d.y * (cam_dist_3d / (cam_dist_3d + p1_3d.z));

                let x2 = p2_3d.x * (cam_dist_3d / (cam_dist_3d + p2_3d.z));
                let y2 = p2_3d.y * (cam_dist_3d / (cam_dist_3d + p2_3d.z));

                // Color based on W depth?
                let color = if (r1.w + r2.w) > 0.0 { Color::Yellow } else { Color::Blue };

                // Draw Line
                ctx.draw(&CanvasLine {
                    x1: x1.into(),
                    y1: y1.into(),
                    x2: x2.into(),
                    y2: y2.into(),
                    color,
                });
            }
        });

    f.render_widget(canvas, chunks[0]);
}
