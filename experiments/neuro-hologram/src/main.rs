use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use neuro_sim::Network;
use rand::Rng;
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

mod hologram;
use hologram::Hologram;

const GRID_WIDTH: usize = 64;
const GRID_HEIGHT: usize = 64;

struct App {
    hologram: Hologram,
    reconstruction_angle_x: isize,
    reconstruction_angle_y: isize,
    status_msg: String,
    reconstruction_data: Vec<f64>,
    network: Network,
    grid: Vec<f64>,
}

impl App {
    fn new() -> Self {
        let mut network = Network::new();
        let total_neurons = GRID_WIDTH * GRID_HEIGHT;
        let mut rng = rand::thread_rng();

        // Initialize neural network grid
        for _ in 0..total_neurons {
            let n = network.add_neuron();
            // Randomly configure neurons to be excitatory or inhibitory
            if rng.gen_bool(0.8) {
                // Excitatory (RS)
                network.neurons[n].a = 0.02;
                network.neurons[n].b = 0.2;
                network.neurons[n].c = -65.0;
                network.neurons[n].d = 8.0;
            } else {
                // Inhibitory (FS)
                network.neurons[n].a = 0.1;
                network.neurons[n].b = 0.2;
                network.neurons[n].c = -65.0;
                network.neurons[n].d = 2.0;
            }
        }

        // Add local connections
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let from = y * GRID_WIDTH + x;
                // Connect to neighbors within a small radius
                for dy in -2..=2isize {
                    for dx in -2..=2isize {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if nx >= 0 && nx < GRID_WIDTH as isize && ny >= 0 && ny < GRID_HEIGHT as isize {
                            let to = (ny as usize) * GRID_WIDTH + (nx as usize);
                            let weight = if network.neurons[from].a == 0.02 {
                                rng.gen_range(1.0..5.0) // Excitatory
                            } else {
                                rng.gen_range(-5.0..-1.0) // Inhibitory
                            };
                            network.add_synapse(from, to, weight);
                        }
                    }
                }
            }
        }

        let grid = vec![0.0; total_neurons];
        let mut app = Self {
            hologram: Hologram::new(GRID_WIDTH, GRID_HEIGHT),
            reconstruction_angle_x: -20,
            reconstruction_angle_y: -10,
            status_msg: "Use Arrow Keys to adjust Angle. Space to pulse.".into(),
            reconstruction_data: vec![],
            network,
            grid,
        };
        app.update_hologram();
        app
    }

    fn update_hologram(&mut self) {
        // Map neuron voltage (typically -65 to 30) to [0, 1] range for the hologram grid
        for (i, neuron) in self.network.neurons.iter().enumerate() {
            let v = neuron.v;
            // Normalize assuming -65 is rest, 30 is spike
            let normalized = ((v + 65.0) / 95.0).clamp(0.0, 1.0) as f64;
            self.grid[i] = normalized;
        }

        // We use from_grid which applies the 2D FFT to the neural grid
        self.hologram = Hologram::from_grid(&self.grid, GRID_WIDTH, GRID_HEIGHT);
        self.update_reconstruction();
    }

    fn update_reconstruction(&mut self) {
        self.reconstruction_data = self
            .hologram
            .reconstruct(self.reconstruction_angle_x, self.reconstruction_angle_y);
    }

    fn pulse(&mut self, current_inputs: &mut [f32]) {
        let mut rng = rand::thread_rng();
        // Inject current into a random cluster of neurons
        let cx = rng.gen_range(10..GRID_WIDTH - 10);
        let cy = rng.gen_range(10..GRID_HEIGHT - 10);
        for y in cy - 5..=cy + 5 {
            for x in cx - 5..=cx + 5 {
                let idx = y * GRID_WIDTH + x;
                current_inputs[idx] += rng.gen_range(20.0..50.0);
            }
        }
        self.status_msg = format!("Pulsed cluster at ({}, {})", cx, cy);
    }

    fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        let mut rng = rand::thread_rng();
        let mut external_inputs = vec![0.0; self.network.neurons.len()];
        loop {
            // Apply background noise
            for i in 0..self.network.neurons.len() {
                // Don't reset completely, allow pulse to persist for 1 frame
                if external_inputs[i] > 0.0 {
                    external_inputs[i] *= 0.8;
                }
                if external_inputs[i] < 0.1 {
                    external_inputs[i] = 0.0;
                }
                if rng.gen_bool(0.01) {
                    external_inputs[i] += rng.gen_range(5.0..15.0);
                }
            }

            self.network.step(&external_inputs);
            self.update_hologram();

            terminal
                .draw(|f| self.ui(f))
                .map_err(|e| io::Error::other(e.to_string()))?;

            if event::poll(Duration::from_millis(30))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Esc => return Ok(()),
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
                                self.pulse(&mut external_inputs);
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
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .split(chunks[1]);

        let title = Paragraph::new(" NEURO HOLOGRAM ")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // Left Panel: Neural Grid
        let canvas_grid = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Spiking Neural Network (Spatial) "),
            )
            .marker(ratatui::symbols::Marker::Braille)
            .x_bounds([0.0, GRID_WIDTH as f64])
            .y_bounds([0.0, GRID_HEIGHT as f64])
            .paint(|ctx| {
                let mut points = Vec::new();
                for (i, &val) in self.grid.iter().enumerate() {
                    if val > 0.5 { // Only draw high voltages
                        let x = (i % GRID_WIDTH) as f64;
                        let y = (i / GRID_WIDTH) as f64;
                        let y_flipped = GRID_HEIGHT as f64 - y;
                        points.push((x, y_flipped));
                    }
                }
                ctx.draw(&Points {
                    coords: &points,
                    color: Color::Red,
                });
            });
        f.render_widget(canvas_grid, main_chunks[0]);

        // Center Panel: Hologram (Frequency Domain)
        let hologram_mag = self.hologram.get_magnitude();
        let max_mag = hologram_mag.iter().cloned().fold(0.0_f64, f64::max);

        let canvas_hologram = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Spectral Cognition (Frequency) "),
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
        f.render_widget(canvas_hologram, main_chunks[1]);

        // Right Panel: Reconstruction (Spatial Domain)
        let recon_mag = &self.reconstruction_data;
        let max_recon = recon_mag.iter().cloned().fold(0.0_f64, f64::max);

        let canvas_recon = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Holographic Reconstruction "),
            )
            .marker(ratatui::symbols::Marker::Braille)
            .x_bounds([0.0, self.hologram.width as f64])
            .y_bounds([0.0, self.hologram.height as f64])
            .paint(|ctx| {
                let threshold = max_recon * 0.5;
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
        f.render_widget(canvas_recon, main_chunks[2]);

        // Status
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
