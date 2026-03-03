use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{
        Block, Borders, Paragraph,
        canvas::{Canvas, Points},
    },
};
use std::{
    io,
    time::{Duration, Instant},
};

mod boid;
mod hologram;
mod world;

use hologram::Hologram;
use world::World;

struct App {
    hologram: Hologram,
    reconstruction_angle_x: isize,
    reconstruction_angle_y: isize,
    text_buffer: String,
    status_msg: String,
    reconstruction_data: Vec<f64>,
    world: World,
    running: bool,
    hologram_points: Vec<(f64, f64)>, // Cached points for boids to seek
}

impl App {
    fn new(world_width: f64, world_height: f64) -> Self {
        let text = "SWARM";
        let mut app = Self {
            hologram: Hologram::new(world_width as usize, world_height as usize),
            reconstruction_angle_x: -20,
            reconstruction_angle_y: -10,
            text_buffer: text.to_string(),
            status_msg: "Use Arrow Keys to adjust Angle. Type to update text.".into(),
            reconstruction_data: vec![],
            world: World::new(world_width, world_height),
            running: true,
            hologram_points: vec![],
        };
        app.update_hologram();
        app
    }

    fn update_hologram(&mut self) {
        self.hologram = Hologram::from_text(&self.text_buffer);
        self.update_reconstruction();
    }

    fn update_reconstruction(&mut self) {
        self.reconstruction_data = self
            .hologram
            .reconstruct(self.reconstruction_angle_x, self.reconstruction_angle_y);

        // Calculate hologram points based on reconstruction
        let recon_mag = &self.reconstruction_data;
        let max_recon = recon_mag.iter().cloned().fold(0.0_f64, f64::max);
        let threshold = max_recon * 0.2;

        self.hologram_points.clear();
        for (i, &val) in recon_mag.iter().enumerate() {
            if val > threshold {
                let x = (i % self.hologram.width) as f64;
                let y = (i / self.hologram.width) as f64;
                // Flip Y for screen coords
                let y_flipped = self.hologram.height as f64 - y;
                self.hologram_points.push((x, y_flipped));
            }
        }
    }

    fn on_tick(&mut self) {
        self.world.update(&self.hologram_points);
    }

    #[allow(clippy::collapsible_if)]
    fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        let tick_rate = Duration::from_millis(33); // ~30 FPS
        let mut last_tick = Instant::now();

        loop {
            terminal
                .draw(|f| self.ui(f))
                .map_err(|e| io::Error::other(e.to_string()))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Esc => self.running = false,
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
                            KeyCode::Char(c) => {
                                if c.is_alphanumeric() || c == ' ' {
                                    self.text_buffer.push(c);
                                    self.update_hologram();
                                }
                            }
                            KeyCode::Backspace => {
                                self.text_buffer.pop();
                                self.update_hologram();
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

            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }

            if !self.running {
                return Ok(());
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
            " LUMINOUS HOLOGRAM - Text Buffer: {} ",
            self.text_buffer
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
                    .title(" Hologram Frequency Pattern "),
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

        // Right Panel: Reconstruction with Luminous Swarm
        let sync_index = self.world.synchronization_index();

        let canvas_recon = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" Holographic Swarm (Sync: {:.2}) ", sync_index)),
            )
            .marker(ratatui::symbols::Marker::Braille)
            .x_bounds([0.0, self.world.width])
            .y_bounds([0.0, self.world.height])
            .paint(|ctx| {
                // Optionally, we could draw the exact reconstruction points dimly
                // ctx.draw(&Points {
                //     coords: &self.hologram_points,
                //     color: Color::DarkGray,
                // });

                // Draw the swarm
                for boid in &self.world.boids {
                    let (char_str, color) = if boid.flash_timer > 0 {
                        ("★".to_string(), Color::White) // Flash
                    } else {
                        let base_char = boid.dna.char_representation.to_string();
                        let color = boid.dna.color;
                        (base_char, color)
                    };

                    ctx.print(
                        boid.position.x,
                        boid.position.y,
                        Span::styled(char_str, Style::default().fg(color)),
                    );
                }
            });
        f.render_widget(canvas_recon, main_chunks[1]);

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

    let mut app = App::new(256.0, 128.0);
    let res = app.run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}
