//! # Origami Hologram
//!
//! **Status:** CONDEMNED (Grace Period)
//! **Lineage:** `origami` × FFT holography concepts.
//!
//! ## Description
//!
//! This experiment crosses a procedural Miura-ori mesh generation algorithm (`origami`) with a simple Fast Fourier Transform (FFT) based holographic simulation using `rustfft`. The simulation projects the three-dimensional geometry of the folded origami sheet onto a two-dimensional holographic recording plane, treating the `Z` height of the folds as the amplitude (or phase) of a scattered wavefront. It then computationally reconstructs the wavefield using an inverse FFT to display the resulting 2D spatial intensity pattern.
//!
//! - **Organism Core:** Procedural Miura-ori mesh structure.
//! - **Holographic Substrate:** 2D Complex Wavefield array processed via `rustfft`.
//! - **Emergence:** As the origami mesh breathes (expanding and contracting via an oscillating `extension_factor`), the spatial frequencies of its folds change. The reconstructed hologram allows the viewer to observe this structural change not as a 3D object, but as a shifting interference pattern.
//!
//! ## Execution Quality
//! Currently functional but minimal. Shows the structural mapping but lacks advanced phase modulation or deeper interaction.
//!
//! ## Controls
//! - **Arrow Keys:** Adjust the phase shift angle of the reconstruction beam (`reconstruction_angle_x`, `reconstruction_angle_y`).
//! - **Q / Esc:** Quit.
//!
//! ## Lineage Details
//! - **Parent A (origami):** Provides `generate_miura_mesh` and structural constraints.
//! - **Parent B (rustfft):** Provides the mathematical substrate for calculating the 2D Inverse Fast Fourier Transform, converting frequency space back into a spatial intensity map.
//!
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use origami::{generate_miura_mesh, MiuraParams, Orientation};
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
    reconstruction_angle_x: isize,
    reconstruction_angle_y: isize,
    status_msg: String,
    reconstruction_data: Vec<f64>,
    extension_factor: f32,
    time: f32,
}

impl App {
    fn new() -> Self {
        Self {
            hologram: Hologram::new(256, 128),
            reconstruction_angle_x: 0,
            reconstruction_angle_y: 0,
            status_msg: "Use Arrow Keys to adjust Angle.".into(),
            reconstruction_data: vec![],
            extension_factor: 0.1,
            time: 0.0,
        }
    }

    fn update(&mut self) {
        self.time += 0.05;
        self.extension_factor = 0.5 + 0.4 * self.time.sin(); // Oscillate between 0.1 and 0.9

        let params = MiuraParams {
            a: 1.0,
            b: 1.0,
            gamma: 1.4,
            orientation: Orientation::Horizontal,
        };

        // Generate origami mesh
        let mesh = generate_miura_mesh(params, (20, 20), self.extension_factor);

        self.hologram.clear();

        // Project the mesh into the hologram plane
        let center_x = self.hologram.width as f32 / 2.0;
        let center_y = self.hologram.height as f32 / 2.0;
        let scale = 10.0; // Scale mesh coordinates to pixels

        for vertex in mesh.vertices {
            // Treat Z height as wave source intensity
            let px = (center_x + vertex.pos.x * scale) as isize;
            let py = (center_y + vertex.pos.y * scale) as isize;

            if px >= 0
                && px < self.hologram.width as isize
                && py >= 0
                && py < self.hologram.height as isize
            {
                // Let Z height map to intensity/phase
                let intensity = (vertex.pos.z.abs() * 0.5).clamp(0.0, 1.0) as f64;

                let idx = py as usize * self.hologram.width + px as usize;
                self.hologram.data[idx] = num_complex::Complex::new(intensity, 0.0);
            }
        }

        // Apply 2D FFT logic by adding a reference beam and squaring
        let reference_angle_x = 0.5;
        for y in 0..self.hologram.height {
            for x in 0..self.hologram.width {
                let phase = (x as f64) * reference_angle_x;
                let ref_wave = num_complex::Complex::new(phase.cos(), phase.sin());
                let idx = y * self.hologram.width + x;
                self.hologram.data[idx] += ref_wave;
            }
        }

        // Convert to Intensity (Interference Pattern)
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
    <B as Backend>::Error: std::error::Error + Send + Sync + 'static,
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
        "Extension: {:.2} | Angle: ({}, {}) | {}",
        app.extension_factor,
        app.reconstruction_angle_x,
        app.reconstruction_angle_y,
        app.status_msg
    );

    let header = Paragraph::new(header_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Origami Hologram - Spectral Fold")
            .style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(header, chunks[0]);

    let w = app.hologram.width as f64;
    let h = app.hologram.height as f64;

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Reconstruction"),
        )
        .paint(|ctx| {
            let mut points_by_intensity: Vec<Vec<(f64, f64)>> = vec![vec![]; 10];

            for y in 0..app.hologram.height {
                for x in 0..app.hologram.width {
                    let val = app.reconstruction_data[y * app.hologram.width + x];
                    if val > 0.1 {
                        let intensity_idx = ((val * 10.0) as usize).clamp(0, 9);
                        let cx = (x as f64 / w) * 200.0 - 100.0;
                        let cy = (y as f64 / h) * 200.0 - 100.0;
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
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}
