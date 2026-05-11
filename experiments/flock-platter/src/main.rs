use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use flocking::{compute_force, FlockingParams};
use locus::Vec2;
use platter::Platter;
use tui_shared::ratatui::{
    self,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct FlockPlatterApp {
    platter: Platter,
    positions: Vec<Vec2>,
    velocities: Vec<Vec2>,
    params: FlockingParams,
    width: usize,
    height: usize,
    time: f32,
}

impl FlockPlatterApp {
    fn new(width: usize, height: usize) -> Self {
        let mut positions = Vec::new();
        let mut velocities = Vec::new();
        for i in 0..80 {
            positions.push(Vec2::new(
                (i * 13 % 100) as f64,
                (i * 7 % 100) as f64,
            ));
            velocities.push(Vec2::new(
                ((i % 5) as f64 - 2.0) * 0.5,
                ((i % 7) as f64 - 3.0) * 0.5,
            ));
        }

        let params = FlockingParams {
            view_radius: 20.0,
            separation_radius: 5.0,
            max_speed: 1.5,
            max_force: 0.1,
            separation_weight: 1.5,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
        };

        Self {
            platter: Platter::new(width, height),
            positions,
            velocities,
            params,
            width,
            height,
            time: 0.0,
        }
    }

    fn tick(&mut self) {
        self.time += 0.05;

        // Update the flock simulation
        let forces: Vec<Vec2> = (0..self.positions.len())
            .map(|i| compute_force(&self.positions, &self.velocities, i, &self.params))
            .collect();

        for (i, force) in forces.iter().enumerate() {
            self.velocities[i] += *force;
            self.velocities[i] = self.velocities[i].limit(self.params.max_speed);
            self.positions[i] += self.velocities[i];

            // Wrap around edges
            if self.positions[i].x < 0.0 { self.positions[i].x += 100.0; }
            if self.positions[i].x > 100.0 { self.positions[i].x -= 100.0; }
            if self.positions[i].y < 0.0 { self.positions[i].y += 100.0; }
            if self.positions[i].y > 100.0 { self.positions[i].y -= 100.0; }
        }

        // Project the boids onto the 2D platter scalar field
        for pos in &self.positions {
            let boid_x = pos.x;
            let boid_y = pos.y;

            // Map x from [0.0, 100.0] to [0, width]
            let grid_x = (boid_x / 100.0 * self.width as f64) as isize;
            // Map y from [0.0, 100.0] to [0, height]
            let grid_y = (boid_y / 100.0 * self.height as f64) as isize;

            if grid_x >= 0
                && grid_x < self.width as isize
                && grid_y >= 0
                && grid_y < self.height as isize
            {
                // The heat represents the presence of the boid
                self.platter.accumulate(grid_x as usize, grid_y as usize, 0.4);
            }
        }

        // Decay the field to create fading trails
        self.platter.decay(0.92);
    }

    fn draw(&self, frame: &mut ratatui::Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(frame.area());

        let title = " 🧬 Splice: flocking × platter (Swarm Heatmap / Pheromone Trails) ";
        let canvas = Canvas::default()
            .block(Block::default().title(title).borders(Borders::ALL))
            .paint(|ctx| {
                // Draw Platter Heatmap
                for y in 0..self.height {
                    for x in 0..self.width {
                        let val = self.platter.get(x, y);
                        if val > 0.05 {
                            // Map scalar value to a color density
                            let color = if val > 1.2 {
                                Color::Red
                            } else if val > 0.8 {
                                Color::LightRed
                            } else if val > 0.4 {
                                Color::Yellow
                            } else {
                                Color::DarkGray
                            };

                            // Map grid to abstract coords
                            let render_x = (x as f64 / self.width as f64) * 100.0;
                            let render_y = (y as f64 / self.height as f64) * 100.0;

                            ctx.print(
                                render_x,
                                render_y,
                                ratatui::text::Span::styled("█", Style::default().fg(color)),
                            );
                        }
                    }
                }

                // Overlay Boids
                for pos in &self.positions {
                    ctx.print(
                        pos.x,
                        pos.y,
                        ratatui::text::Span::styled("v", Style::default().fg(Color::Cyan)),
                    );
                }
            })
            .x_bounds([0.0, 100.0])
            .y_bounds([0.0, 100.0]);

        frame.render_widget(canvas, chunks[0]);

        let info = Paragraph::new(format!(
            "Boids: {} | Swarm leaving behind pheromone heat map...",
            self.positions.len()
        ))
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
        frame.render_widget(info, chunks[1]);
    }
}

fn main() -> Result<()> {
    // Determine if we should run in headless mode (for CI/Testing)
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("🧬 flock-platter running in headless mode for 10 ticks.");
        let mut app = FlockPlatterApp::new(100, 100);
        for _ in 0..10 {
            app.tick();
        }
        println!("Finished headless run.");
        return Ok(());
    }

    let mut tui = Tui::init()?;

    let mut app = FlockPlatterApp::new(200, 100);
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| app.draw(f))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && (key.code == KeyCode::Char('q') || key.code == KeyCode::Esc)
                {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }
    }

    Ok(())
}
