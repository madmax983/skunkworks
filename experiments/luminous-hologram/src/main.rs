use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::{io, time::Duration};

use locus::{flocking::{compute_force, FlockingParams}, Topology, Vec2};
use rand::Rng;

mod hologram;
use hologram::Hologram;

struct App {
    hologram: Hologram,
    reconstruction_angle_x: isize,
    reconstruction_angle_y: isize,
    status_msg: String,
    reconstruction_data: Vec<f64>,

    // Boid specific
    positions: Vec<Vec2>,
    velocities: Vec<Vec2>,
    params: FlockingParams,
    width: usize,
    height: usize,
}

impl App {
    fn new(width: usize, height: usize, boid_count: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut positions = Vec::with_capacity(boid_count);
        let mut velocities = Vec::with_capacity(boid_count);

        for _ in 0..boid_count {
            positions.push(Vec2::new(
                rng.gen_range(0.0..width as f64),
                rng.gen_range(0.0..height as f64),
            ));
            let angle = rng.gen_range(0.0..std::f64::consts::TAU);
            velocities.push(Vec2::new(angle.cos(), angle.sin()));
        }

        let params = FlockingParams {
            view_radius: 15.0,
            separation_radius: 5.0,
            max_speed: 1.0,
            max_force: 0.1,
            separation_weight: 1.5,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
        };

        let mut app = Self {
            hologram: Hologram::new(width, height),
            reconstruction_angle_x: -20,
            reconstruction_angle_y: -10,
            status_msg: "Use Arrow Keys to adjust Recon Angle.".into(),
            reconstruction_data: vec![],
            positions,
            velocities,
            params,
            width,
            height,
        };
        app.update_hologram();
        app
    }

    fn update_boids(&mut self) {
        let forces: Vec<Vec2> = (0..self.positions.len())
            .map(|i| compute_force(&self.positions, &self.velocities, i, &self.params))
            .collect();

        let topo = Topology::Torus;

        for (i, force) in forces.iter().enumerate() {
            self.velocities[i] += *force;
            self.velocities[i] = self.velocities[i].limit(self.params.max_speed);
            self.positions[i] += self.velocities[i];

            // Wrap around edges
            let y_idx = self.positions[i].y.round() as i64;
            let x_idx = self.positions[i].x.round() as i64;

            if let Some((ny, nx)) = topo.normalize(y_idx, x_idx, self.width, self.height) {
                self.positions[i].x = nx as f64;
                self.positions[i].y = ny as f64;
            }
        }
    }

    fn update_hologram(&mut self) {
        let boid_coords: Vec<(f64, f64)> = self.positions.iter().map(|p| (p.x, p.y)).collect();
        self.hologram = Hologram::from_boids(self.width, self.height, &boid_coords);
        self.update_reconstruction();
    }

    fn update_reconstruction(&mut self) {
        self.reconstruction_data = self
            .hologram
            .reconstruct(self.reconstruction_angle_x, self.reconstruction_angle_y);
    }

    fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            // Update step
            self.update_boids();
            self.update_hologram();

            // Render
            terminal
                .draw(|f| self.ui(f))
                .map_err(|e| io::Error::other(e.to_string()))?;

            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Esc | KeyCode::Char('q') => return Ok(()),
                            KeyCode::Left => {
                                self.reconstruction_angle_x -= 1;
                                self.status_msg = "Adjusting Angle X -".into();
                            }
                            KeyCode::Right => {
                                self.reconstruction_angle_x += 1;
                                self.status_msg = "Adjusting Angle X +".into();
                            }
                            KeyCode::Up => {
                                self.reconstruction_angle_y += 1;
                                self.status_msg = "Adjusting Angle Y +".into();
                            }
                            KeyCode::Down => {
                                self.reconstruction_angle_y -= 1;
                                self.status_msg = "Adjusting Angle Y -".into();
                            }
                            KeyCode::Enter => {
                                self.reconstruction_angle_x = -20;
                                self.reconstruction_angle_y = -10;
                                self.status_msg = "Reset to Recording Angle (-20, -10).".into();
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    fn ui(&self, f: &mut ratatui::Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
            ])
            .split(f.area());

        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        // Header
        let title = Paragraph::new(format!(
            " LUMINOUS HOLOGRAM - Boids: {} ",
            self.positions.len()
        ))
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // Left Panel: Hologram (Frequency Domain)
        let hologram_mag = self.hologram.get_magnitude();
        let max_mag = hologram_mag.iter().cloned().fold(0.0_f64, f64::max);

        let canvas_hologram = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Hologram (Spectral Domain) "),
            )
            .marker(ratatui::symbols::Marker::Braille)
            .x_bounds([0.0, self.width as f64])
            .y_bounds([0.0, self.height as f64])
            .paint(|ctx| {
                let threshold = max_mag * 0.4;
                let mut points = Vec::new();
                for (i, &val) in hologram_mag.iter().enumerate() {
                    if val > threshold {
                        let x = (i % self.width) as f64;
                        let y = (i / self.width) as f64;
                        let y_flipped = self.height as f64 - y;
                        points.push((x, y_flipped));
                    }
                }

                ctx.draw(&Points {
                    coords: &points,
                    color: Color::Blue,
                });
            });
        f.render_widget(canvas_hologram, main_chunks[0]);

        // Right Panel: Reconstruction (Spatial Domain)
        let recon_mag = &self.reconstruction_data;
        let max_recon = recon_mag.iter().cloned().fold(0.0_f64, f64::max);

        let canvas_recon = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Reconstruction (Spatial Swarm) "),
            )
            .marker(ratatui::symbols::Marker::Braille)
            .x_bounds([0.0, self.width as f64])
            .y_bounds([0.0, self.height as f64])
            .paint(|ctx| {
                let threshold = max_recon * 0.2;
                let mut recon_points = Vec::new();

                // Draw reconstructed points
                for (i, &val) in recon_mag.iter().enumerate() {
                    if val > threshold {
                        let x = (i % self.width) as f64;
                        let y = (i / self.width) as f64;
                        let y_flipped = self.height as f64 - y;
                        recon_points.push((x, y_flipped));
                    }
                }

                ctx.draw(&Points {
                    coords: &recon_points,
                    color: Color::Green,
                });

                // Also draw actual boids sparsely for reference, maybe in Cyan?
                let mut actual_points = Vec::new();
                for p in &self.positions {
                    actual_points.push((p.x, self.height as f64 - p.y));
                }
                ctx.draw(&Points {
                    coords: &actual_points,
                    color: Color::Cyan,
                });
            });
        f.render_widget(canvas_recon, main_chunks[1]);

        // Status / Controls
        let status = Paragraph::new(format!(
            "Angle: ({}, {}) | Target: (-20, -10) | {}",
            self.reconstruction_angle_x, self.reconstruction_angle_y, self.status_msg
        ))
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL));
        f.render_widget(status, chunks[2]);
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(256, 128, 100);
    let res = app.run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}
