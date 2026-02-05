pub mod attractors;

use anyhow::Result;
use attractors::{AizawaAttractor, LorenzAttractor, RosslerAttractor, Vector3};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{
        Block, Borders, Paragraph,
        canvas::{Canvas, Points},
    },
};
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};
use tui_shared::Tui;

const TRAIL_LENGTH: usize = 2000;

enum AttractorType {
    Lorenz(LorenzAttractor),
    Rossler(RosslerAttractor),
    Aizawa(AizawaAttractor),
}

impl AttractorType {
    fn update(&self, p: Vector3, dt: f64) -> Vector3 {
        match self {
            Self::Lorenz(a) => a.update(p, dt),
            Self::Rossler(a) => a.update(p, dt),
            Self::Aizawa(a) => a.update(p, dt),
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::Lorenz(a) => a.name(),
            Self::Rossler(a) => a.name(),
            Self::Aizawa(a) => a.name(),
        }
    }

    fn default_point(&self) -> Vector3 {
        match self {
            Self::Lorenz(_) => Vector3::new(0.1, 0.0, 0.0),
            Self::Rossler(_) => Vector3::new(0.1, 0.1, 0.1),
            Self::Aizawa(_) => Vector3::new(0.1, 0.0, 0.0),
        }
    }

    fn scale(&self) -> f64 {
        match self {
            Self::Lorenz(_) => 2.0,
            Self::Rossler(_) => 3.0,
            Self::Aizawa(_) => 40.0,
        }
    }

    fn dt(&self) -> f64 {
        match self {
            Self::Lorenz(_) => 0.005,
            Self::Rossler(_) => 0.02,
            Self::Aizawa(_) => 0.01,
        }
    }
}

struct App {
    attractor: AttractorType,
    points: VecDeque<Vector3>,
    angle_x: f64,
    angle_y: f64,
    angle_z: f64,
    current_pos: Vector3,
    auto_rotate: bool,
}

impl App {
    fn new() -> Self {
        let attractor = AttractorType::Lorenz(LorenzAttractor::default());
        let current_pos = attractor.default_point();
        Self {
            attractor,
            points: VecDeque::with_capacity(TRAIL_LENGTH),
            angle_x: 0.5,
            angle_y: 0.5,
            angle_z: 0.0,
            current_pos,
            auto_rotate: true,
        }
    }

    fn next_attractor(&mut self) {
        self.attractor = match self.attractor {
            AttractorType::Lorenz(_) => AttractorType::Rossler(RosslerAttractor::default()),
            AttractorType::Rossler(_) => AttractorType::Aizawa(AizawaAttractor::default()),
            AttractorType::Aizawa(_) => AttractorType::Lorenz(LorenzAttractor::default()),
        };
        self.points.clear();
        self.current_pos = self.attractor.default_point();
    }

    fn update(&mut self) {
        let dt = self.attractor.dt();
        // Update multiple times per frame for smoother lines
        for _ in 0..5 {
            self.current_pos = self.attractor.update(self.current_pos, dt);
            self.points.push_back(self.current_pos);
            if self.points.len() > TRAIL_LENGTH {
                self.points.pop_front();
            }
        }

        if self.auto_rotate {
            self.angle_y += 0.01;
            // self.angle_z += 0.002;
        }
    }
}

fn rotate(p: Vector3, ax: f64, ay: f64, az: f64) -> (f64, f64) {
    // Rotate around X
    let y1 = p.y * ax.cos() - p.z * ax.sin();
    let z1 = p.y * ax.sin() + p.z * ax.cos();
    let x1 = p.x;

    // Rotate around Y
    let x2 = x1 * ay.cos() + z1 * ay.sin();
    let _z2 = -x1 * ay.sin() + z1 * ay.cos();
    let y2 = y1;

    // Rotate around Z
    let x3 = x2 * az.cos() - y2 * az.sin();
    let y3 = x2 * az.sin() + y2 * az.cos();

    (x3, y3)
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(16); // ~60 FPS

    loop {
        tui.terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char(' ') => app.auto_rotate = !app.auto_rotate,
                KeyCode::Tab => app.next_attractor(),
                KeyCode::Left => app.angle_y -= 0.1,
                KeyCode::Right => app.angle_y += 0.1,
                KeyCode::Up => app.angle_x -= 0.1,
                KeyCode::Down => app.angle_x += 0.1,
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    let scale = app.attractor.scale();

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Strange Loops "),
        )
        .x_bounds([-100.0, 100.0])
        .y_bounds([-100.0, 100.0])
        .paint(|ctx| {
            // Group by color to reduce draw calls
            let mut batches: Vec<(Color, Vec<(f64, f64)>)> = Vec::new();

            for (i, p) in app.points.iter().enumerate() {
                let (x, y) = rotate(*p, app.angle_x, app.angle_y, app.angle_z);
                let sx = x * scale;
                let sy = y * scale;

                let color = if i > app.points.len().saturating_sub(20) {
                    Color::White
                } else {
                    let t = i as f64 / app.points.len() as f64;
                    if t < 0.2 {
                        Color::Blue
                    } else if t < 0.4 {
                        Color::Cyan
                    } else if t < 0.6 {
                        Color::Green
                    } else if t < 0.8 {
                        Color::Yellow
                    } else {
                        Color::Red
                    }
                };

                // Add to batch
                if let Some((last_color, list)) = batches.last_mut()
                    && *last_color == color
                {
                    list.push((sx, sy));
                    continue;
                }
                batches.push((color, vec![(sx, sy)]));
            }

            for (color, points) in batches {
                ctx.draw(&Points {
                    coords: &points,
                    color,
                });
            }
        });

    f.render_widget(canvas, chunks[0]);

    let status = format!(
        "System: {} | [Tab] Switch | [Arrows] Rotate | [Space] Pause Rot | [Q] Quit",
        app.attractor.name()
    );
    let bar = Paragraph::new(status).style(Style::default().fg(Color::DarkGray));
    f.render_widget(bar, chunks[1]);
}
