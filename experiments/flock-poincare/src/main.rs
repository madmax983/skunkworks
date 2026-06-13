use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use flocking::{compute_force, FlockingParams};
use locus::Vec2;
use poincare_disk::{hyperbolic_dist, Point};
use rand::Rng;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},

    widgets::{canvas::Canvas, Block, Borders, Paragraph},
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};
use tui_shared::Tui;

const NUM_BOIDS: usize = 150;

struct App {
    positions: Vec<Vec2>,
    velocities: Vec<Vec2>,
    params: FlockingParams,
}

impl App {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut positions = Vec::with_capacity(NUM_BOIDS);
        let mut velocities = Vec::with_capacity(NUM_BOIDS);

        for _ in 0..NUM_BOIDS {
            // Keep them somewhat near the center initially
            positions.push(Vec2::new(
                rng.gen_range(-0.5..0.5),
                rng.gen_range(-0.5..0.5),
            ));
            velocities.push(Vec2::new(
                rng.gen_range(-0.02..0.02),
                rng.gen_range(-0.02..0.02),
            ));
        }

        Self {
            positions,
            velocities,
            params: FlockingParams {
                view_radius: 0.2, // smaller view radius in hyperbolic space
                separation_radius: 0.05,
                max_speed: 0.05,
                max_force: 0.002,
                separation_weight: 1.5,
                alignment_weight: 1.0,
                cohesion_weight: 1.0,
            },
        }
    }

    fn update(&mut self) {
        let forces: Vec<Vec2> = (0..NUM_BOIDS)
            .map(|i| compute_force(&self.positions, &self.velocities, i, &self.params))
            .collect();

        for i in 0..NUM_BOIDS {
            self.velocities[i] += forces[i];
            self.velocities[i] = self.velocities[i].limit(self.params.max_speed);
            self.positions[i] += self.velocities[i];

            // Hyperbolic constraint: if they try to leave the Poincaré disk (radius = 1.0)
            // wrap or reflect them.
            let r2 = self.positions[i].x * self.positions[i].x + self.positions[i].y * self.positions[i].y;
            if r2 >= 0.95 * 0.95 { // keep slightly inside boundary
                // Reflect velocity
                let normal_x = -self.positions[i].x;
                let normal_y = -self.positions[i].y;
                let _dot = self.velocities[i].x * normal_x + self.velocities[i].y * normal_y;

                // Add a small push back to the center
                self.velocities[i].x += normal_x * 0.01;
                self.velocities[i].y += normal_y * 0.01;

                // Dampen speed
                self.velocities[i].x *= 0.5;
                self.velocities[i].y *= 0.5;

                // Push inside
                let mag = r2.sqrt();
                self.positions[i].x = (self.positions[i].x / mag) * 0.94;
                self.positions[i].y = (self.positions[i].y / mag) * 0.94;
            }
        }
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
    let center_pt = Point::new(0.0, 0.0);

    loop {
        if !headless {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(0), Constraint::Length(3)])
                    .split(f.area());

                let title = " 🧬 Splice: flocking × poincare-disk (Hyperbolic Swarm Dynamics) ";
                let canvas = Canvas::default()
                    .block(Block::default().title(title).borders(Borders::ALL))
                    .paint(|ctx| {
                        // Draw unit disk boundary
                        for theta in 0..100 {
                            let angle = (theta as f64 / 100.0) * std::f64::consts::PI * 2.0;
                            let x = angle.cos();
                            let y = angle.sin();
                            ctx.print(
                                (x + 1.0) * 50.0,
                                (y + 1.0) * 50.0,
                                ratatui::text::Span::styled(".", Style::default().fg(Color::DarkGray)),
                            );
                        }

                        // Draw boids
                        for pos in &app.positions {
                            let pt = Point::new(pos.x, pos.y);

                            // Color based on hyperbolic depth
                            let h_dist = hyperbolic_dist(pt, center_pt);
                            let color = if h_dist > 3.0 {
                                Color::Red
                            } else if h_dist > 1.5 {
                                Color::Yellow
                            } else if h_dist > 0.5 {
                                Color::Cyan
                            } else {
                                Color::Blue
                            };

                            let render_x = (pos.x + 1.0) * 50.0;
                            let render_y = (pos.y + 1.0) * 50.0;

                            if render_x >= 0.0 && render_x <= 100.0 && render_y >= 0.0 && render_y <= 100.0 {
                                ctx.print(
                                    render_x,
                                    render_y,
                                    ratatui::text::Span::styled("v", Style::default().fg(color)),
                                );
                            }
                        }
                    })
                    .x_bounds([0.0, 100.0])
                    .y_bounds([0.0, 100.0]);

                f.render_widget(canvas, chunks[0]);

                let info = Paragraph::new("Boids navigating the Poincaré disk. Distances warp near the infinite boundary.")
                    .style(Style::default().fg(Color::Cyan))
                    .block(Block::default().borders(Borders::ALL));
                f.render_widget(info, chunks[1]);
            })?;
        }

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if headless {
            app.update();
            tick_count += 1;
            if tick_count > 10 {
                break;
            }
        } else if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && (key.code == KeyCode::Char('q') || key.code == KeyCode::Esc) {
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
        println!("🧬 flock-poincare running in headless mode.");
        let backend = CrosstermBackend::new(std::io::stdout());
        let mut terminal = Terminal::new(backend)?;
        run_app(&mut terminal, app, tick_rate, true)?;
        println!("Finished headless run.");
    } else {
        let mut tui = Tui::init()?;
        run_app(&mut tui.terminal, app, tick_rate, false)?;
    }

    Ok(())
}
