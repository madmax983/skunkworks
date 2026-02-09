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

mod hologram;
use hologram::Hologram;

struct App {
    hologram: Hologram,
    reconstruction_angle_x: isize,
    reconstruction_angle_y: isize,
    text_buffer: String,
    status_msg: String,
    reconstruction_data: Vec<f64>,
}

impl App {
    fn new() -> Self {
        let text = "Hologram";
        let mut app = Self {
            hologram: Hologram::new(256, 128),
            reconstruction_angle_x: 0,
            reconstruction_angle_y: 0,
            text_buffer: text.to_string(),
            status_msg: "Use Arrow Keys to adjust Angle. Type to update text.".into(),
            reconstruction_data: vec![],
        };
        app.update_hologram();
        app
    }

    fn update_hologram(&mut self) {
        self.hologram = Hologram::from_text(&self.text_buffer);
        self.update_reconstruction();
    }

    fn update_reconstruction(&mut self) {
        // The recording angles in hologram.rs are hardcoded to ~20 bins X and ~10 bins Y.
        // So perfect reconstruction is at (-20, -10).
        // But let's verify that logic.
        // If we shift the spectrum by +20 bins during recording, it's centered at +20.
        // To center it at DC for viewing, we need to shift by -20 bins.
        // So reconstruction angle should be -20.

        self.reconstruction_data = self.hologram.reconstruct(self.reconstruction_angle_x, self.reconstruction_angle_y);
    }

    fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            terminal.draw(|f| self.ui(f))
                .map_err(|e| io::Error::other(e.to_string()))?;

            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Esc => return Ok(()),
                            KeyCode::Left => {
                                self.reconstruction_angle_x -= 1;
                                self.update_reconstruction();
                            },
                            KeyCode::Right => {
                                self.reconstruction_angle_x += 1;
                                self.update_reconstruction();
                            },
                            KeyCode::Up => {
                                self.reconstruction_angle_y += 1;
                                self.update_reconstruction();
                            },
                            KeyCode::Down => {
                                self.reconstruction_angle_y -= 1;
                                self.update_reconstruction();
                            },
                            KeyCode::Char(c) => {
                                if c.is_alphanumeric() || c == ' ' {
                                    self.text_buffer.push(c);
                                    self.update_hologram();
                                }
                            },
                            KeyCode::Backspace => {
                                self.text_buffer.pop();
                                self.update_hologram();
                            },
                            KeyCode::Enter => {
                                // Reset angles to match recording
                                // Recording is at +20, +10. We need shift -20, -10 to bring it back to DC.
                                self.reconstruction_angle_x = -20;
                                self.reconstruction_angle_y = -10;
                                self.update_reconstruction();
                                self.status_msg = "Reset to Recording Angle.".into();
                            },
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
        let title = Paragraph::new(format!(" HOLOGRAM TEXT - Text Buffer: {} ", self.text_buffer))
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // Left Panel: Hologram (Frequency Domain)
        let hologram_mag = self.hologram.get_magnitude(); // Log scale
        let max_mag = hologram_mag.iter().cloned().fold(0.0_f64, f64::max);

        // We draw points where magnitude is high
        let canvas_hologram = Canvas::default()
            .block(Block::default().borders(Borders::ALL).title(" Hologram (Frequency Domain) "))
            .marker(ratatui::symbols::Marker::Braille)
            .x_bounds([0.0, self.hologram.width as f64])
            .y_bounds([0.0, self.hologram.height as f64])
            .paint(|ctx| {
                // Determine threshold based on max
                let threshold = max_mag * 0.5; // Only show peaks

                // Collect points
                let mut points = Vec::new();
                for (i, &val) in hologram_mag.iter().enumerate() {
                    if val > threshold {
                        let x = (i % self.hologram.width) as f64;
                        let y = (i / self.hologram.width) as f64;
                        // Flip Y because Canvas coordinates are bottom-up usually?
                        // Or just draw as is. text/hologram logic is top-down usually.
                        // Let's flip Y to match screen coords.
                        let y_flipped = self.hologram.height as f64 - y;
                        points.push((x, y_flipped));
                    }
                }

                // Draw all points at once
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
            .block(Block::default().borders(Borders::ALL).title(" Reconstruction (Spatial Domain) "))
            .marker(ratatui::symbols::Marker::Braille)
            .x_bounds([0.0, self.hologram.width as f64])
            .y_bounds([0.0, self.hologram.height as f64])
            .paint(|ctx| {
                let threshold = max_recon * 0.2; // Lower threshold for text
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
