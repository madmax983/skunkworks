use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use neuro_sim::Network;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Frame, Terminal,
};
use std::{io, time::Duration};

mod hologram;
use hologram::Hologram;

struct App {
    hologram: Hologram,
    network: Network,
    reconstruction_angle_x: isize,
    reconstruction_angle_y: isize,
    status_msg: String,
    reconstruction_data: Vec<f64>,
}

impl App {
    fn new() -> Self {
        let mut network = Network::new();

        // Setup a small recurrent neural network for continuous activity
        let mut neurons = Vec::new();
        for _ in 0..16 {
            neurons.push(network.add_neuron());
        }

        // Connect neurons in a ring topology with some cross connections
        for i in 0..neurons.len() {
            let next = (i + 1) % neurons.len();
            let cross = (i + neurons.len() / 2) % neurons.len();

            // Excitatory ring connection
            network.add_synapse(neurons[i], neurons[next], 15.0);
            // Occasional inhibitory cross connection to create bursty rhythms
            if i % 3 == 0 {
                network.add_synapse(neurons[i], neurons[cross], -10.0);
            }
        }

        Self {
            hologram: Hologram::new(256, 128),
            network,
            reconstruction_angle_x: 0,
            reconstruction_angle_y: 0,
            status_msg: "Use Arrow Keys to adjust Angle.".into(),
            reconstruction_data: vec![],
        }
    }

    fn update(&mut self) {
        // Inject baseline noise current into the network to keep it active
        let mut input_currents = vec![0.0; self.network.neurons.len()];

        // Randomly stimulate one neuron occasionally
        if rand::random::<f32>() < 0.2 {
            let idx = rand::random::<usize>() % self.network.neurons.len();
            input_currents[idx] = 25.0; // Strong current pulse
        }

        // Add small constant noise to all
        for cur in input_currents.iter_mut() {
            *cur += 5.0 * rand::random::<f32>();
        }

        self.network.step(&input_currents);

        // Render network state to hologram
        self.hologram.clear();

        // Map neural voltage directly to holographic density field
        let num_neurons = self.network.neurons.len();

        // We will project the neurons into a 2D grid
        let grid_w = (num_neurons as f32).sqrt().ceil() as usize;

        let center_x = self.hologram.width / 2;
        let center_y = self.hologram.height / 2;
        let spacing = 15;

        for (i, neuron) in self.network.neurons.iter().enumerate() {
            let gx = i % grid_w;
            let gy = i / grid_w;

            let ox = center_x as isize + (gx as isize - (grid_w as isize / 2)) * spacing;
            let oy = center_y as isize + (gy as isize - (grid_w as isize / 2)) * spacing;

            // Normalize voltage roughly. Izhikevich typical resting is -65, peak is 30.
            // Map [-65, 30] to [0, 1] roughly.
            let v = neuron.v;
            let normalized_v: f64 = ((v + 65.0) / 95.0).clamp(0.0, 1.0) as f64;

            // Set a point in the interference field.
            // If the neuron spikes (v >= 30), it acts as a very intense point source.
            let intensity = if v >= 29.0 {
                5.0 // Spikes are bright flashes
            } else {
                normalized_v
            };

            // Spread out the neuron over a few pixels to give it a "size"
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let px = ox + dx;
                    let py = oy + dy;
                    if px >= 0 && px < self.hologram.width as isize && py >= 0 && py < self.hologram.height as isize {
                        // Blend slightly with distance
                        let dist = (dx*dx + dy*dy) as f64;
                        let val = intensity / (1.0 + dist);
                        self.hologram.data[py as usize * self.hologram.width + px as usize] = num_complex::Complex::new(val, 0.0);
                    }
                }
            }
        }

        // Add Reference Beam (Recording Angle)
        let reference_angle = 0.5; // Example angle
        for y in 0..self.hologram.height {
            for x in 0..self.hologram.width {
                let phase = (x as f64) * reference_angle;
                let ref_wave = num_complex::Complex::new(phase.cos(), phase.sin());
                self.hologram.data[y * self.hologram.width + x] += ref_wave;
            }
        }

        // Apply Intensity (Interference Pattern)
        for i in 0..self.hologram.data.len() {
            let magnitude = self.hologram.data[i].norm();
            self.hologram.data[i] = num_complex::Complex::new(magnitude * magnitude, 0.0);
        }

        let dx = self.reconstruction_angle_x;
        let dy = self.reconstruction_angle_y;

        self.reconstruction_data = self.hologram.reconstruct(dx, dy);
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()>
where
    <B as Backend>::Error: std::error::Error + Send + Sync + 'static
{
    loop {
        app.update();

        terminal.draw(|f| ui(f, &app))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Up => app.reconstruction_angle_y -= 1,
                        KeyCode::Down => app.reconstruction_angle_y += 1,
                        KeyCode::Left => app.reconstruction_angle_x -= 1,
                        KeyCode::Right => app.reconstruction_angle_x += 1,
                        _ => {}
                    }
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.area());

    let header_text = format!(
        "Angle: ({}, {}) | {}",
        app.reconstruction_angle_x, app.reconstruction_angle_y, app.status_msg
    );

    let header = Paragraph::new(header_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Neuro-Hologram Spectral Morphogenesis")
            .style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(header, chunks[0]);

    let w = app.hologram.width as f64;
    let h = app.hologram.height as f64;

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Reconstruction"))
        .paint(|ctx| {
            let mut points_by_intensity: Vec<Vec<(f64, f64)>> = vec![vec![]; 10];

            for y in 0..app.hologram.height {
                for x in 0..app.hologram.width {
                    let val = app.reconstruction_data[y * app.hologram.width + x];
                    if val > 0.1 {
                        let intensity_idx = ((val * 10.0) as usize).clamp(0, 9);
                        // Map coordinates to canvas bounds
                        // Let's use simple scaling
                        let cx = (x as f64 / w) * 200.0 - 100.0;
                        let cy = (y as f64 / h) * 200.0 - 100.0;
                        // Reverse Y for ratatui canvas coordinates
                        points_by_intensity[intensity_idx].push((cx, -cy));
                    }
                }
            }

            for (i, points) in points_by_intensity.into_iter().enumerate() {
                let color = match i {
                    0..=2 => Color::DarkGray,
                    3..=5 => Color::Gray,
                    6..=7 => Color::White,
                    8..=9 => Color::Cyan,
                    _ => Color::Cyan,
                };
                ctx.draw(&Points {
                    coords: &points,
                    color,
                });
            }
        })
        .x_bounds([-100.0, 100.0])
        .y_bounds([-100.0, 100.0]);

    f.render_widget(canvas, chunks[1]);
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}
