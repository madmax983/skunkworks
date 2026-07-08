use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use flocking::{compute_force, FlockingParams};
use locus::{Topology, Vec2};
use platter::Platter;
use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct FlockPlatterApp {
    positions: Vec<Vec2>,
    velocities: Vec<Vec2>,
    params: FlockingParams,
    platter: Platter,
    width: usize,
    height: usize,
    topology: Topology,
}

impl FlockPlatterApp {
    fn new(width: usize, height: usize, boid_count: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut positions = Vec::with_capacity(boid_count);
        let mut velocities = Vec::with_capacity(boid_count);

        for _ in 0..boid_count {
            positions.push(Vec2::new(
                rng.gen_range(0.0..width as f64),
                rng.gen_range(0.0..height as f64),
            ));
            let angle = rng.gen_range(0.0..std::f64::consts::PI * 2.0);
            velocities.push(Vec2::new(angle.cos(), angle.sin()));
        }

        Self {
            positions,
            velocities,
            params: FlockingParams {
                view_radius: 8.0,
                separation_radius: 2.0,
                max_speed: 1.0,
                max_force: 0.05,
                separation_weight: 1.5,
                alignment_weight: 1.0,
                cohesion_weight: 1.0,
            },
            platter: Platter::new(width, height),
            width,
            height,
            topology: Topology::Torus,
        }
    }

    fn tick(&mut self) {
        // Calculate forces
        let forces: Vec<Vec2> = (0..self.positions.len())
            .map(|i| compute_force(&self.positions, &self.velocities, i, &self.params))
            .collect();

        // Update positions, wrap around topology, and deposit heat
        for (i, force) in forces.iter().enumerate() {
            self.velocities[i] += *force;
            self.velocities[i] = self.velocities[i].limit(self.params.max_speed);
            self.positions[i] += self.velocities[i];

            // Wrap coordinates using Torus topology
            let x = self.positions[i].x.round() as i64;
            let y = self.positions[i].y.round() as i64;
            if let Some((ny, nx)) = self.topology.normalize(y, x, self.height, self.width) {
                self.positions[i].x = nx as f64;
                self.positions[i].y = ny as f64;

                // Deposit heat at the current position
                self.platter.accumulate(nx, ny, 0.5);
            }
        }

        // Decay the heat map
        self.platter.decay(0.95);
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Headless mode: Simulating 100 ticks to verify logic in CI...");
        let mut app = FlockPlatterApp::new(100, 50, 150);
        for _ in 0..100 {
            app.tick();
        }
        println!("Headless execution successful.");
        return Ok(());
    }

    let mut tui = Tui::init()?;
    let res = run_app(&mut tui);
    tui.exit()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app(tui: &mut Tui) -> Result<()> {
    let width = 100;
    let height = 50;
    let boid_count = 150;
    let mut app = FlockPlatterApp::new(width, height, boid_count);

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(50);

    loop {
        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.area());

            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Flock Heatmap "),
                )
                .x_bounds([0.0, app.width as f64])
                .y_bounds([0.0, app.height as f64])
                .paint(|ctx| {
                    // Draw Heatmap from platter
                    let mut max_heat = 0.0;
                    for y in 0..app.height {
                        for x in 0..app.width {
                            let heat = app.platter.get(x, y);
                            if heat > max_heat {
                                max_heat = heat;
                            }
                            if heat > 0.1 {
                                let color = if heat > 2.0 {
                                    Color::Red
                                } else if heat > 1.0 {
                                    Color::Yellow
                                } else if heat > 0.5 {
                                    Color::Green
                                } else {
                                    Color::DarkGray
                                };
                                ctx.print(
                                    x as f64,
                                    y as f64,
                                    ratatui::text::Span::styled("█", Style::default().fg(color)),
                                );
                            }
                        }
                    }

                    // Draw Boids
                    for pos in &app.positions {
                        ctx.print(
                            pos.x,
                            pos.y,
                            ratatui::text::Span::styled("v", Style::default().fg(Color::White)),
                        );
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let mut max_heat = 0.0;
            for y in 0..app.height {
                for x in 0..app.width {
                    let heat = app.platter.get(x, y);
                    if heat > max_heat {
                        max_heat = heat;
                    }
                }
            }

            let stats = Paragraph::new(format!(
                "Boids: {} | Topology: Torus | Max Heat: {:.2} | [Q] Quit",
                app.positions.len(),
                max_heat
            ))
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(stats, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }
    }
}
