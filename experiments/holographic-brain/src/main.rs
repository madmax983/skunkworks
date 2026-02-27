use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, MouseButton, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

mod brain;
mod hologram;
// mod neuron; // Not needed if we use neuro_sim directly in brain.rs

use brain::Brain;

struct App {
    brain: Brain,
    running: bool,
}

impl App {
    fn new() -> Self {
        Self {
            brain: Brain::new(64, 32),
            running: true,
        }
    }

    fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
        let tick_rate = Duration::from_millis(33); // ~30 FPS
        let mut last_tick = Instant::now();

        while self.running {
            terminal.draw(|f| self.ui(f))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout)? {
                match event::read()? {
                    Event::Key(key) => self.handle_key(key),
                    Event::Mouse(mouse) => self.handle_mouse(mouse),
                    _ => {}
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.brain.update();
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.running = false,
            KeyCode::Char('l') => self.brain.is_learning = !self.brain.is_learning,
            KeyCode::Char('c') => self.brain.clear_neurons(),
            KeyCode::Char('r') => self.brain.reset_memory(),
            KeyCode::Char('n') => self.brain.inject_noise(10.0),
            _ => {}
        }
    }

    fn handle_mouse(&mut self, mouse: event::MouseEvent) {
        match mouse.kind {
            MouseEventKind::Down(MouseButton::Left) | MouseEventKind::Drag(MouseButton::Left) => {
                // Map mouse to neuron grid coordinates
                // This is tricky because TUI coordinates are screen-space.
                // We need to know where the canvas is.
                // For simplicity, we can inject noise globally or implement raycasting.
                // But raycasting in TUI is hard without knowing the rect.
                // Let's just use 'n' key for noise.
                // Or maybe simulate touch if we can.
                // Ratatui doesn't easily give back the rect in event handler unless we store it.
                // We'll skip precise mouse interaction for now.
            }
            _ => {}
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
        let title =
            Paragraph::new(" HOLOGRAPHIC BRAIN - Neural Network with Frequency Domain Memory ")
                .style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )
                .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // 1. Neurons (Spatial Domain)
        let canvas_neurons = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Neurons (Spatial) "),
            )
            .marker(ratatui::symbols::Marker::Block)
            .x_bounds([0.0, self.brain.width as f64])
            .y_bounds([0.0, self.brain.height as f64])
            .paint(|ctx| {
                for (i, neuron) in self.brain.neurons.iter().enumerate() {
                    let x = (i % self.brain.width) as f64;
                    let y = (i / self.brain.width) as f64;
                    // Flip Y
                    let y_flipped = self.brain.height as f64 - y;

                    // Color based on voltage
                    // -65 is resting (Blue), 30 is spike (Red)
                    let v = neuron.v;
                    let color = if v > 0.0 {
                        Color::Red
                    } else if v > -50.0 {
                        Color::Yellow
                    } else if v > -60.0 {
                        Color::Green
                    } else {
                        Color::Blue
                    };

                    if v > -65.0 {
                        ctx.draw(&Points {
                            coords: &[(x, y_flipped)],
                            color,
                        });
                    }
                }
            });
        f.render_widget(canvas_neurons, main_chunks[0]);

        // 2. Hologram (Frequency Domain)
        let spectrum = self.brain.memory.get_magnitude_spectrum();
        let max_spec = spectrum.iter().cloned().fold(0.0_f64, f64::max);

        let canvas_hologram = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Hologram (Frequency) "),
            )
            .marker(ratatui::symbols::Marker::Block) // Braille is finer but Block is brighter
            .x_bounds([0.0, self.brain.width as f64])
            .y_bounds([0.0, self.brain.height as f64])
            .paint(|ctx| {
                for (i, &val) in spectrum.iter().enumerate() {
                    let x = (i % self.brain.width) as f64;
                    let y = (i / self.brain.width) as f64;
                    let y_flipped = self.brain.height as f64 - y;

                    if val > max_spec * 0.5 {
                        ctx.draw(&Points {
                            coords: &[(x, y_flipped)],
                            color: Color::Magenta,
                        });
                    }
                }
            });
        f.render_widget(canvas_hologram, main_chunks[1]);

        // 3. Reconstruction (Ghost Image)
        let reconstruction = self.brain.memory.reconstruct();
        let max_recon = reconstruction.iter().cloned().fold(0.0_f64, f64::max);

        let canvas_recon = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Reconstruction (Ghost) "),
            )
            .marker(ratatui::symbols::Marker::Block)
            .x_bounds([0.0, self.brain.width as f64])
            .y_bounds([0.0, self.brain.height as f64])
            .paint(|ctx| {
                for (i, &val) in reconstruction.iter().enumerate() {
                    let x = (i % self.brain.width) as f64;
                    let y = (i / self.brain.width) as f64;
                    let y_flipped = self.brain.height as f64 - y;

                    if val > max_recon * 0.2 {
                        ctx.draw(&Points {
                            coords: &[(x, y_flipped)],
                            color: Color::White,
                        });
                    }
                }
            });
        f.render_widget(canvas_recon, main_chunks[2]);

        // Controls
        let controls = Paragraph::new(format!(
            "Controls: [L]earn: {} | [N]oise | [C]lear Neurons | [R]eset Hologram | [Q]uit",
            if self.brain.is_learning { "ON" } else { "OFF" }
        ))
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL));
        f.render_widget(controls, chunks[2]);
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
