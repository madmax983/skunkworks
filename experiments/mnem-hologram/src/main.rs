use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use locus::Vec2;
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

mod graph;
mod hologram;

use graph::Graph;
use hologram::Hologram;

struct App {
    hologram: Hologram,
    reconstruction_angle_x: isize,
    reconstruction_angle_y: isize,
    status_msg: String,
    reconstruction_data: Vec<f64>,
    graph: Graph,
    entropy: f64,
}

impl App {
    fn new() -> Self {
        let mut graph = Graph::new();
        graph.scan_directory("src");
        if graph.nodes.is_empty() {
            graph.scan_directory(".");
        }

        for node in &mut graph.nodes {
            node.pos = Vec2::new(rand::random::<f64>() * 256.0, rand::random::<f64>() * 128.0);
        }

        let mut app = Self {
            hologram: Hologram::new(256, 128),
            reconstruction_angle_x: -20,
            reconstruction_angle_y: -10,
            status_msg: "Use Arrow Keys to adjust Angle. Space to add entropy. H to heal.".into(),
            reconstruction_data: vec![],
            graph,
            entropy: 0.0,
        };
        app.update_physics();
        app
    }

    fn update_physics(&mut self) {
        let mut forces = vec![Vec2::new(0.0, 0.0); self.graph.nodes.len()];
        let center = Vec2::new(128.0, 64.0);

        for (i, force) in forces.iter_mut().enumerate() {
            let dir_to_center = center - self.graph.nodes[i].pos;
            *force += dir_to_center * 0.01;

            for j in 0..self.graph.nodes.len() {
                if i == j {
                    continue;
                }
                let diff = self.graph.nodes[i].pos - self.graph.nodes[j].pos;
                let dist = diff.magnitude();
                if dist > 0.1 && dist < 50.0 {
                    *force += diff.normalize() * (200.0 / dist.powi(2));
                }
            }
        }

        for edge in &self.graph.edges {
            let diff = self.graph.nodes[edge.to].pos - self.graph.nodes[edge.from].pos;
            let dist = diff.magnitude();
            let force = diff.normalize() * (dist - 30.0) * 0.05;
            forces[edge.from] += force;
            forces[edge.to] -= force;
        }

        for (i, node) in self.graph.nodes.iter_mut().enumerate() {
            node.vel = (node.vel + forces[i]) * 0.5;
            node.pos += node.vel;

            node.pos.x = node.pos.x.clamp(10.0, 246.0);
            node.pos.y = node.pos.y.clamp(10.0, 118.0);

            node.health = (node.health as f64 - 0.001 * self.entropy).max(0.0) as f32;

            if self.entropy > 0.0 {
                node.pos.x += rand::random::<f64>() * self.entropy - (self.entropy / 2.0);
                node.pos.y += rand::random::<f64>() * self.entropy - (self.entropy / 2.0);
            }
        }

        self.update_hologram();
    }

    fn update_hologram(&mut self) {
        let width = 256;
        let height = 128;
        let mut grid = vec![0.0_f64; width * height];

        for node in &self.graph.nodes {
            let cx = node.pos.x as isize;
            let cy = node.pos.y as isize;

            let spread = (1.0 - node.health as f64) * 5.0 + 1.0;
            let r = spread as isize;

            for dy in -r..=r {
                for dx in -r..=r {
                    let px = cx + dx;
                    let py = cy + dy;
                    if px >= 0 && px < width as isize && py >= 0 && py < height as isize {
                        let idx = (py as usize) * width + (px as usize);
                        grid[idx] += node.health as f64
                            * (1.0 - (dx.abs() as f64 + dy.abs() as f64) / (r as f64 * 2.0))
                                .max(0.1);
                    }
                }
            }
        }

        self.hologram = Hologram::from_density(width, height, &grid);
        self.update_reconstruction();
    }

    fn update_reconstruction(&mut self) {
        self.reconstruction_data = self
            .hologram
            .reconstruct(self.reconstruction_angle_x, self.reconstruction_angle_y);
    }

    fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            self.update_physics();

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
                                self.update_reconstruction();
                            }
                            KeyCode::Right => {
                                self.reconstruction_angle_x += 1;
                                self.update_reconstruction();
                            }
                            KeyCode::Up => {
                                self.reconstruction_angle_y += 1;
                                self.update_reconstruction();
                            }
                            KeyCode::Down => {
                                self.reconstruction_angle_y -= 1;
                                self.update_reconstruction();
                            }
                            KeyCode::Char(' ') => {
                                self.entropy += 1.0;
                            }
                            KeyCode::Char('h') => {
                                self.entropy = (self.entropy - 2.0).max(0.0);
                                for node in &mut self.graph.nodes {
                                    node.health = (node.health + 0.5).min(1.0);
                                }
                            }
                            KeyCode::Enter => {
                                self.reconstruction_angle_x = -20;
                                self.reconstruction_angle_y = -10;
                                self.update_reconstruction();
                                self.status_msg = "Reset to Recording Angle.".into();
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

        let title = Paragraph::new(format!(
            " MNEMONIC HOLOGRAM - Nodes: {} - Entropy: {:.2} ",
            self.graph.nodes.len(),
            self.entropy
        ))
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        let hologram_mag = self.hologram.get_magnitude();
        let max_mag = hologram_mag.iter().cloned().fold(0.0_f64, f64::max);

        let canvas_hologram = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Hologram (Frequency Domain) "),
            )
            .marker(ratatui::symbols::Marker::Braille)
            .x_bounds([0.0, self.hologram.width as f64])
            .y_bounds([0.0, self.hologram.height as f64])
            .paint(|ctx| {
                let threshold = max_mag * 0.5;
                let mut points = Vec::new();
                for (i, &val) in hologram_mag.iter().enumerate() {
                    if val > threshold {
                        let x = (i % self.hologram.width) as f64;
                        let y = (i / self.hologram.width) as f64;
                        let y_flipped = self.hologram.height as f64 - y;
                        points.push((x, y_flipped));
                    }
                }
                ctx.draw(&Points {
                    coords: &points,
                    color: Color::Blue,
                });
            });
        f.render_widget(canvas_hologram, main_chunks[0]);

        let recon_mag = &self.reconstruction_data;
        let max_recon = recon_mag.iter().cloned().fold(0.0_f64, f64::max);

        let canvas_recon = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Reconstruction (Spatial Domain) "),
            )
            .marker(ratatui::symbols::Marker::Braille)
            .x_bounds([0.0, self.hologram.width as f64])
            .y_bounds([0.0, self.hologram.height as f64])
            .paint(|ctx| {
                let threshold = max_recon * 0.2;
                let mut points = Vec::new();
                for (i, &val) in recon_mag.iter().enumerate() {
                    if val > threshold {
                        let x = (i % self.hologram.width) as f64;
                        let y = (i / self.hologram.width) as f64;
                        let y_flipped = self.hologram.height as f64 - y;
                        points.push((x, y_flipped));
                    }
                }
                ctx.draw(&Points {
                    coords: &points,
                    color: Color::Green,
                });
            });
        f.render_widget(canvas_recon, main_chunks[1]);

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

    let mut app = App::new();
    let res = app.run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}
