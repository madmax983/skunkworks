use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use flocking::{compute_force, FlockingParams};
use locus::Vec2;
use platter::Platter;
use rand::Rng;
use ratatui::{
    backend::CrosstermBackend,
    layout::Margin,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders},
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};
use tui_shared::Tui;

const WIDTH: usize = 120;
const HEIGHT: usize = 60;
const NUM_BOIDS: usize = 100;

struct App {
    positions: Vec<Vec2>,
    velocities: Vec<Vec2>,
    params: FlockingParams,
    platter: Platter,
}

impl App {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut positions = Vec::with_capacity(NUM_BOIDS);
        let mut velocities = Vec::with_capacity(NUM_BOIDS);

        for _ in 0..NUM_BOIDS {
            positions.push(Vec2::new(
                rng.gen_range(0.0..WIDTH as f64),
                rng.gen_range(0.0..HEIGHT as f64),
            ));
            velocities.push(Vec2::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            ));
        }

        Self {
            positions,
            velocities,
            params: FlockingParams {
                view_radius: 10.0,
                separation_radius: 3.0,
                max_speed: 1.0,
                max_force: 0.05,
                separation_weight: 1.5,
                alignment_weight: 1.0,
                cohesion_weight: 1.0,
            },
            platter: Platter::new(WIDTH, HEIGHT),
        }
    }

    fn update(&mut self) {
        // Compute forces and update velocities/positions
        let forces: Vec<Vec2> = (0..NUM_BOIDS)
            .map(|i| compute_force(&self.positions, &self.velocities, i, &self.params))
            .collect();

        for i in 0..NUM_BOIDS {
            self.velocities[i] += forces[i];
            self.velocities[i] = self.velocities[i].limit(self.params.max_speed);
            self.positions[i] += self.velocities[i];

            // Wrap around the screen
            if self.positions[i].x < 0.0 {
                self.positions[i].x += WIDTH as f64;
            } else if self.positions[i].x >= WIDTH as f64 {
                self.positions[i].x -= WIDTH as f64;
            }
            if self.positions[i].y < 0.0 {
                self.positions[i].y += HEIGHT as f64;
            } else if self.positions[i].y >= HEIGHT as f64 {
                self.positions[i].y -= HEIGHT as f64;
            }
        }

        // Apply heat to the platter based on boid positions
        for pos in &self.positions {
            let x = pos.x.round() as usize;
            let y = pos.y.round() as usize;
            if x < WIDTH && y < HEIGHT {
                self.platter.accumulate(x, y, 0.2); // Add heat
            }
        }

        // Decay the heat
        self.platter.decay(0.9);
    }
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    mut app: App,
    tick_rate: Duration,
    headless: bool,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let mut tick_count = 0;

    loop {
        if !headless {
            terminal.draw(|f| {
                let size = f.area();
                let b = Block::default()
                    .title("Swarm Thermal Deposition")
                    .borders(Borders::ALL);
                f.render_widget(b, size);

                // Render the heat map from the platter
                let inner = size.inner(Margin {
                    horizontal: 1,
                    vertical: 1,
                });
                // We'll just render it as characters directly for simplicity
                for y in 0..inner.height {
                    for x in 0..inner.width {
                        let px = (x as usize * WIDTH) / inner.width as usize;
                        let py = (y as usize * HEIGHT) / inner.height as usize;
                        if px < WIDTH && py < HEIGHT {
                            let heat = app.platter.get(px, py);
                            if heat > 0.1 {
                                let ch = if heat > 0.8 {
                                    '#'
                                } else if heat > 0.5 {
                                    '+'
                                } else {
                                    '.'
                                };
                                let color = if heat > 0.8 {
                                    Color::Red
                                } else if heat > 0.5 {
                                    Color::Yellow
                                } else {
                                    Color::DarkGray
                                };
                                // Quick hack for rendering raw positions
                                f.buffer_mut().set_span(
                                    inner.x + x,
                                    inner.y + y,
                                    &Span::styled(ch.to_string(), Style::default().fg(color)),
                                    1,
                                );
                            }
                        }
                    }
                }

                // Render the boids
                for pos in &app.positions {
                    let rx = ((pos.x / WIDTH as f64) * inner.width as f64).round() as u16;
                    let ry = ((pos.y / HEIGHT as f64) * inner.height as f64).round() as u16;
                    if rx < inner.width && ry < inner.height {
                        f.buffer_mut().set_span(
                            inner.x + rx,
                            inner.y + ry,
                            &Span::styled("O", Style::default().fg(Color::Cyan)),
                            1,
                        );
                    }
                }
            })?;
        }

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if headless {
            app.update();
            tick_count += 1;
            if tick_count > 100 {
                break;
            }
        } else if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            if !headless {
                app.update();
            }
            last_tick = Instant::now();
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let headless = args.contains(&"--headless".to_string());

    let app = App::new();
    let tick_rate = Duration::from_millis(50);

    if headless {
        // Create a dummy backend to satisfy the function signature.
        let backend = CrosstermBackend::new(std::io::stdout());
        let mut terminal = Terminal::new(backend)?;
        run_app(&mut terminal, app, tick_rate, true)?;
    } else {
        let mut tui = Tui::init()?;
        run_app(&mut tui.terminal, app, tick_rate, false)?;
    }

    Ok(())
}
