use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{
        canvas::{Canvas, Line},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};
use tui_shared::Tui;

const WIDTH: usize = 120; // Number of X buckets
const DEPTH: usize = 60; // Number of Z rows (Visible Horizon)

struct App {
    terrain: VecDeque<Vec<u8>>, // Current visible terrain (Rows of heights)
    r: f64,                     // Current growth rate parameter
    speed: f64,                 // Speed of flight (delta r per second)
    paused: bool,
    accumulation: f64,
}

impl App {
    fn new() -> Result<Self> {
        // Initialize flat terrain
        let mut terrain = VecDeque::new();
        for _ in 0..DEPTH {
            terrain.push_back(vec![0; WIDTH]);
        }

        Ok(Self {
            terrain,
            r: 2.8,
            speed: 0.1, // r increases by 0.1 per second
            paused: false,
            accumulation: 0.0,
        })
    }

    fn update(&mut self, dt: f64) {
        if self.paused {
            return;
        }

        // speed is "r units per second"
        // We want fixed r resolution for smooth terrain continuity
        let r_step_resolution = 0.002;

        // Calculate how much r we should cover
        let r_delta_needed = dt * self.speed;

        self.accumulation += r_delta_needed;

        while self.accumulation >= r_step_resolution {
            self.accumulation -= r_step_resolution;
            self.step(r_step_resolution);
        }
    }

    fn step(&mut self, r_delta: f64) {
        self.r += r_delta;

        // Loop or Bounce
        if self.r > 4.0 {
            self.r = 2.8;
        }

        // Generate Row from Logistic Map
        let row = self.generate_row(self.r);

        // Move Terrain
        self.terrain.pop_front(); // Remove closest
        self.terrain.push_back(row); // Add new at horizon
    }

    fn generate_row(&self, r: f64) -> Vec<u8> {
        let mut row = vec![0u8; WIDTH];
        let mut x = 0.5;

        // Transient
        for _ in 0..100 {
            x = r * x * (1.0 - x);
        }

        // Stable / Sampling
        // Iterate more to catch the period doubling and chaos
        let samples = 200;
        for _ in 0..samples {
            x = r * x * (1.0 - x);

            // Map x [0, 1] to bucket [0, WIDTH]
            let bucket = (x * WIDTH as f64) as usize;
            if bucket < WIDTH {
                // Increment height, cap at 40
                row[bucket] = row[bucket].saturating_add(1).min(40);
            }
        }

        row
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new()?;

    run_app(&mut tui, &mut app)?;

    Ok(())
}

fn run_app(tui: &mut Tui, app: &mut App) -> Result<()> {
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char(' ') => app.paused = !app.paused,
                        KeyCode::Up => app.speed = (app.speed + 0.05).min(2.0),
                        KeyCode::Down => app.speed = (app.speed - 0.05).max(0.01),
                        _ => {}
                    }
                }
            }
        }

        let now = Instant::now();
        let dt = now.duration_since(last_tick).as_secs_f64();
        last_tick = now;

        app.update(dt);
    }
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Bifurcation Landscape: Flight over Chaos "),
        )
        .x_bounds([0.0, WIDTH as f64])
        .y_bounds([0.0, 100.0]) // Z + Height
        .paint(|ctx| {
            // Draw lines
            // Z=0 is CLOSE (Bottom). Z=DEPTH is HORIZON (Top).
            // Painter's Algorithm: Draw Far (High Z) first, then Close (Low Z).

            for (z, row) in app.terrain.iter().enumerate().rev() {
                let z_factor = z as f64;

                for (x, &h) in row.iter().enumerate() {
                    let h = h as f64;
                    if h == 0.0 {
                        continue;
                    }

                    let x_pos = x as f64;
                    // Simple perspective: lines just go up
                    let y_base = z_factor * 1.0;
                    let y_top = y_base + h;

                    let color = if h > 20.0 {
                        Color::Red
                    } else if h > 10.0 {
                        Color::Yellow
                    } else if h > 5.0 {
                        Color::Green
                    } else {
                        Color::Blue
                    };

                    // Draw vertical pillar
                    ctx.draw(&Line {
                        x1: x_pos,
                        y1: y_base,
                        x2: x_pos,
                        y2: y_top,
                        color,
                    });
                }
            }
        });

    f.render_widget(canvas, chunks[0]);

    // Status Bar
    let status = format!(
        "R: {:.5} | Speed: {:.3} | Space: Pause | Up/Down: Speed",
        app.r, app.speed
    );
    f.render_widget(
        Paragraph::new(status).block(Block::default().borders(Borders::ALL)),
        chunks[1],
    );
}
