use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, MouseButton, MouseEventKind};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{
        canvas::{Canvas, Line},
        Block, Borders, Paragraph,
    },
    Frame, Terminal,
};
use std::{
    f64::consts::PI,
    time::{Duration, Instant},
};
use tui_shared::Tui;

mod math;
use math::{dft, Complex, Epicycle};

#[derive(Debug, PartialEq)]
enum AppState {
    Drawing,
    Playing,
}

struct App {
    state: AppState,
    points: Vec<Complex>,
    epicycles: Vec<Epicycle>,
    trace: Vec<Complex>,
    time: f64,
    dt: f64,
    num_epicycles: usize,
    running: bool,
}

impl App {
    fn new() -> Self {
        Self {
            state: AppState::Drawing,
            points: Vec::new(),
            epicycles: Vec::new(),
            trace: Vec::new(),
            time: 0.0,
            dt: 0.01,
            num_epicycles: 0,
            running: true,
        }
    }

    fn reset(&mut self) {
        self.state = AppState::Drawing;
        self.points.clear();
        self.epicycles.clear();
        self.trace.clear();
        self.time = 0.0;
        self.num_epicycles = 0;
    }

    fn compute_dft(&mut self) {
        if self.points.is_empty() {
            return;
        }

        // Compute DFT
        self.epicycles = dft(&self.points);
        self.num_epicycles = self.epicycles.len();
        self.dt = 1.0 / self.epicycles.len() as f64; // Adjust speed based on complexity
        self.state = AppState::Playing;
        self.time = 0.0;
        self.trace.clear();
    }

    fn update(&mut self) {
        if self.state == AppState::Playing {
            // Calculate current point
            let mut current = Complex::zero();

            // Reconstruct from epicycles
            // We only use the first 'num_epicycles' sorted by amplitude
            let count = self.num_epicycles.min(self.epicycles.len());

            for i in 0..count {
                let epi = &self.epicycles[i];
                let angle = epi.freq * 2.0 * PI * self.time + epi.phase;
                let c = Complex::new(angle.cos(), angle.sin());
                // val = amp * (cos + i*sin)
                let val = Complex::new(c.re * epi.amp, c.im * epi.amp);
                current = current.add(val);
            }

            // Add to trace
            // Only add if it's different enough or periodically to avoid memory explosion?
            // For now just add every frame.
            self.trace.push(current);
            if self.trace.len() > 2000 {
                self.trace.remove(0);
            }

            self.time += self.dt;
            if self.time > 1.0 {
                self.time = 0.0;
                self.trace.clear();
            }
        }
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    let res = run_app(&mut tui.terminal);

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    let mut app = App::new();
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    while app.running {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => app.running = false,
                    KeyCode::Char('r') => app.reset(),
                    KeyCode::Up => {
                        if app.state == AppState::Playing && app.num_epicycles < app.epicycles.len()
                        {
                            app.num_epicycles += 1;
                        }
                    }
                    KeyCode::Down => {
                        if app.state == AppState::Playing && app.num_epicycles > 0 {
                            app.num_epicycles -= 1;
                        }
                    }
                    _ => {}
                },
                Event::Mouse(mouse) => {
                    match mouse.kind {
                        MouseEventKind::Down(MouseButton::Left)
                        | MouseEventKind::Drag(MouseButton::Left) => {
                            if app.state == AppState::Drawing {
                                // Invert Y because terminal 0,0 is top-left but canvas usually 0,0 is bottom-left
                                // However, we will set canvas y_bounds to [height, 0] to match screen coords?
                                // Let's just store raw screen coords and configure canvas to match.
                                app.points
                                    .push(Complex::new(mouse.column as f64, mouse.row as f64));
                            }
                        }
                        MouseEventKind::Up(MouseButton::Left) => {
                            if app.state == AppState::Drawing && !app.points.is_empty() {
                                app.compute_dft();
                            }
                        }
                        _ => {}
                    }
                }
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

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let width = chunks[0].width as f64;
    let height = chunks[0].height as f64;

    // Use a Canvas that matches the terminal coordinates
    // Top-Left is (0,0). Bottom-Right is (width, height).
    // Ratatui Canvas default is Bottom-Left (0,0).
    // So we set y_bounds to [height, 0] to flip Y axis.

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Epicycle Draw"),
        )
        .x_bounds([0.0, width])
        .y_bounds([height, 0.0])
        .paint(|ctx| {
            match app.state {
                AppState::Drawing => {
                    ctx.print(
                        width / 2.0 - 10.0,
                        height / 2.0,
                        "Draw something with Mouse!",
                    );

                    for i in 0..app.points.len().saturating_sub(1) {
                        let p1 = app.points[i];
                        let p2 = app.points[i + 1];
                        ctx.draw(&Line {
                            x1: p1.re,
                            y1: p1.im,
                            x2: p2.re,
                            y2: p2.im,
                            color: Color::White,
                        });
                    }
                }
                AppState::Playing => {
                    // Draw trace
                    for i in 0..app.trace.len().saturating_sub(1) {
                        let p1 = app.trace[i];
                        let p2 = app.trace[i + 1];
                        ctx.draw(&Line {
                            x1: p1.re,
                            y1: p1.im,
                            x2: p2.re,
                            y2: p2.im,
                            color: Color::Cyan,
                        });
                    }

                    // Draw Epicycles
                    let mut center = Complex::zero();
                    let count = app.num_epicycles.min(app.epicycles.len());

                    for i in 0..count {
                        let epi = &app.epicycles[i];
                        let angle = epi.freq * 2.0 * PI * app.time + epi.phase;

                        let prev_center = center;

                        let offset = Complex::new(epi.amp * angle.cos(), epi.amp * angle.sin());
                        center = center.add(offset);

                        // Draw Circle (Radius)
                        // Canvas doesn't have Circle primitive easily, but we can draw the radius line
                        ctx.draw(&Line {
                            x1: prev_center.re,
                            y1: prev_center.im,
                            x2: center.re,
                            y2: center.im,
                            color: Color::DarkGray,
                        });

                        // Optionally draw the circle outline using points?
                        // Too expensive for many circles. Radius line is enough visualization.
                    }

                    // Highlight the tip
                    if !app.trace.is_empty() {
                        let tip = app.trace.last().unwrap();
                        ctx.print(tip.re, tip.im, "o");
                    }
                }
            }
        });

    f.render_widget(canvas, chunks[0]);

    let status_text = match app.state {
        AppState::Drawing => {
            "DRAWING MODE | Hold Left Click to Draw | Release to Calculate".to_string()
        }
        AppState::Playing => format!(
            "PLAYING | Epicycles: {}/{} | UP/DOWN: Adjust Precision | R: Reset | Q: Quit",
            app.num_epicycles,
            app.epicycles.len()
        ),
    };

    f.render_widget(
        Paragraph::new(status_text).block(Block::default().borders(Borders::ALL)),
        chunks[1],
    );
}
