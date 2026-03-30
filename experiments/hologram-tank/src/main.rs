use anyhow::Result;
use crossbeam_channel::bounded;
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
use resonance_audio::audio::{AudioCommand, AudioModel};
use std::{io, thread, time::Duration};

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
    cmd_tx: crossbeam_channel::Sender<AudioCommand>,
    snap_rx: crossbeam_channel::Receiver<resonance_audio::audio::AudioSnapshot>,
    current_grid: Vec<f32>,
    tick_count: u64,
}

impl App {
    fn new() -> Self {
        let (cmd_tx, cmd_rx) = bounded(1024);
        let (snap_tx, snap_rx) = bounded(2);

        // Run simulation in background thread
        thread::spawn(move || {
            let mut model = AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx, None);
            let mut dummy_buffer = vec![0.0; 1024];
            loop {
                // We tick the audio model. If no one reads the snapshot, it drops.
                // In real audio, cpal pulls data. Here we just pump it as fast as needed.
                model.process(&mut dummy_buffer);
                thread::sleep(Duration::from_millis(16));
            }
        });

        // Add a wall to make it interesting
        let _ = cmd_tx.send(AudioCommand::AddWall {
            x: GRID_WIDTH / 2,
            y: GRID_HEIGHT / 2,
        });

        let mut app = Self {
            hologram: Hologram::new(GRID_WIDTH, GRID_HEIGHT),
            reconstruction_angle_x: -20,
            reconstruction_angle_y: -10,
            status_msg: "Use Arrow Keys to adjust Angle. Space to Pluck.".into(),
            reconstruction_data: vec![],
            cmd_tx,
            snap_rx,
            current_grid: vec![0.0; GRID_WIDTH * GRID_HEIGHT],
            tick_count: 0,
        };
        app.update_hologram();
        app
    }

    fn update_hologram(&mut self) {
        self.hologram = Hologram::from_grid(&self.current_grid, GRID_WIDTH, GRID_HEIGHT);
        self.update_reconstruction();
    }

    fn update_reconstruction(&mut self) {
        self.reconstruction_data = self
            .hologram
            .reconstruct(self.reconstruction_angle_x, self.reconstruction_angle_y);
    }

    fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            // Poll for latest snapshot from wave tank
            let mut new_snap = false;
            while let Ok(snap) = self.snap_rx.try_recv() {
                self.current_grid = snap.pressure;
                new_snap = true;
            }

            if new_snap {
                self.update_hologram();
            }

            self.tick_count = self.tick_count.wrapping_add(1);

            // Periodically pluck to keep the simulation active
            if self.tick_count % 100 == 0 {
                let _ = self.cmd_tx.send(AudioCommand::Pluck {
                    x: GRID_WIDTH / 4,
                    y: GRID_HEIGHT / 4,
                    strength: 1.0,
                });
            }

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
                            KeyCode::Enter => {
                                self.reconstruction_angle_x = -20;
                                self.reconstruction_angle_y = -10;
                                self.update_reconstruction();
                                self.status_msg = "Reset to Recording Angle (-20, -10).".into();
                            }
                            KeyCode::Char(' ') => {
                                // Pluck center
                                let _ = self.cmd_tx.send(AudioCommand::Pluck {
                                    x: GRID_WIDTH / 2,
                                    y: GRID_HEIGHT / 2,
                                    strength: 1.0,
                                });
                                self.status_msg = "Plucked center!".into();
                            }
                            KeyCode::Char('c') => {
                                let _ = self.cmd_tx.send(AudioCommand::ClearWaves);
                                self.status_msg = "Cleared waves!".into();
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

        // Header
        let title = Paragraph::new(" HOLOGRAM TANK - Acoustic Holography ")
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // Left Panel: Wave Tank (Spatial Domain)
        let canvas_tank = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Wave Tank (Physical Pressure) "),
            )
            .marker(ratatui::symbols::Marker::Braille)
            .x_bounds([0.0, self.hologram.width as f64])
            .y_bounds([0.0, self.hologram.height as f64])
            .paint(|ctx| {
                let mut points_pos = Vec::new();
                let mut points_neg = Vec::new();

                for (i, &val) in self.current_grid.iter().enumerate() {
                    let x = (i % self.hologram.width) as f64;
                    let y = (i / self.hologram.width) as f64;
                    let y_flipped = self.hologram.height as f64 - y;

                    if val > 0.1 {
                        points_pos.push((x, y_flipped));
                    } else if val < -0.1 {
                        points_neg.push((x, y_flipped));
                    }
                }

                ctx.draw(&Points {
                    coords: &points_pos[..],
                    color: Color::Red,
                });
                ctx.draw(&Points {
                    coords: &points_neg[..],
                    color: Color::Blue,
                });
            });
        f.render_widget(canvas_tank, main_chunks[0]);

        // Middle Panel: Hologram (Frequency Domain)
        let hologram_mag = self.hologram.get_magnitude();
        let max_mag = hologram_mag.iter().cloned().fold(0.0_f64, f64::max);

        let canvas_hologram = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Spectral Hologram (Frequency Domain) "),
            )
            .marker(ratatui::symbols::Marker::Braille)
            .x_bounds([0.0, self.hologram.width as f64])
            .y_bounds([0.0, self.hologram.height as f64])
            .paint(|ctx| {
                let threshold = max_mag * 0.3;
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
                    coords: &points[..],
                    color: Color::Magenta,
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
                    .title(" Reconstruction (Optical Output) "),
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
                    coords: &points[..],
                    color: Color::Green,
                });
            });
        f.render_widget(canvas_recon, main_chunks[2]);

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
