use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use locus::Vec2;
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{
        Block, Borders, Paragraph,
        canvas::{Canvas, Points},
    },
};
use std::{io, time::Duration};

mod chaos;
mod hologram;

use chaos::DoublePendulum;
use hologram::Hologram;

struct App {
    hologram: Hologram,
    pendulum: DoublePendulum,
    reconstruction_angle_x: f64,
    reconstruction_angle_y: f64,
    text_buffer: String,
    status_msg: String,
    reconstruction_data: Vec<f64>,
}

impl App {
    fn new() -> Self {
        let initial_text = "CHAOS";
        let hologram = Hologram::from_text(initial_text);

        let pendulum = DoublePendulum::new(Vec2::new(0.0, 0.0));

        // Target correctly reconstructing angles
        let reconstruction_angle_x = -20.0;
        let reconstruction_angle_y = -10.0;

        let reconstruction_data = hologram.reconstruct(
            reconstruction_angle_x as isize,
            reconstruction_angle_y as isize,
        );

        Self {
            hologram,
            pendulum,
            reconstruction_angle_x,
            reconstruction_angle_y,
            text_buffer: initial_text.to_string(),
            status_msg: "Recording: [CHAOS] - Press K to kick pendulum".to_string(),
            reconstruction_data,
        }
    }

    fn update_hologram(&mut self) {
        self.hologram = Hologram::from_text(&self.text_buffer);
        self.update_reconstruction();
    }

    fn update_reconstruction(&mut self) {
        // Perturb the reconstruction angle with the chaos pendulum tip
        let p = self.pendulum.p2();

        // Base angles are -20, -10
        // Pendulum coordinates are roughly between -50 and 50
        // We'll scale them down slightly to map to bin shifts
        let shift_x = self.reconstruction_angle_x + p.x * 0.5;
        let shift_y = self.reconstruction_angle_y + p.y * 0.5;

        self.reconstruction_data = self
            .hologram
            .reconstruct(shift_x as isize, shift_y as isize);
    }
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
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()>
where
    std::io::Error: From<<B as ratatui::backend::Backend>::Error>,
{
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = std::time::Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => return Ok(()),
                        KeyCode::Char('k') | KeyCode::Char('K') => {
                            // Kick the pendulum
                            app.pendulum.a1_v += 10.0;
                            app.pendulum.a2_v -= 10.0;
                        }
                        KeyCode::Char(c) => {
                            app.text_buffer.push(c);
                            app.status_msg = format!("Recording: [{}]", app.text_buffer);
                            app.update_hologram();
                        }
                        KeyCode::Backspace => {
                            app.text_buffer.pop();
                            app.status_msg = format!("Recording: [{}]", app.text_buffer);
                            app.update_hologram();
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            // Integrate chaotic pendulum
            app.pendulum.update(0.05);
            // Reconstruct based on new pendulum state
            app.update_reconstruction();

            last_tick = std::time::Instant::now();
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.area());

    // Top Bar
    let title = Paragraph::new(format!(
        " 🧬 chaos-hologram | {} | Reconstruct Angle X/Y: {:.1}/{:.1} ",
        app.status_msg,
        app.reconstruction_angle_x + app.pendulum.p2().x * 0.5,
        app.reconstruction_angle_y + app.pendulum.p2().y * 0.5
    ))
    .style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Main Layout: Spectrum vs Reconstruction
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(chunks[1]);

    // Spectrum View (Left)
    let mag_data = app.hologram.get_magnitude();
    let width = app.hologram.width as f64;
    let height = app.hologram.height as f64;

    let mut spectrum_points = vec![];
    for (i, &val) in mag_data.iter().enumerate() {
        if val > 1.0 {
            let x = (i % app.hologram.width) as f64;
            let y = (i / app.hologram.width) as f64;

            // Map FFT indices to viewable coords (centered)
            let shifted_x = if x < width / 2.0 {
                x + width / 2.0
            } else {
                x - width / 2.0
            };
            let shifted_y = if y < height / 2.0 {
                y + height / 2.0
            } else {
                y - height / 2.0
            };

            spectrum_points.push((shifted_x, shifted_y));
        }
    }

    let spectrum_canvas = Canvas::default()
        .block(
            Block::default()
                .title(" FFT Spectrum ")
                .borders(Borders::ALL),
        )
        .marker(ratatui::symbols::Marker::Braille)
        .x_bounds([0.0, width])
        .y_bounds([0.0, height])
        .paint(|ctx| {
            ctx.draw(&Points {
                coords: &spectrum_points,
                color: Color::Blue,
            });
            // Draw Pendulum trace in Frequency Domain
            let center_x = width / 2.0;
            let center_y = height / 2.0;

            // Map pendulum's physical (-50..50) coordinates to spectrum coordinate space
            let p1 = app.pendulum.p1();
            let p2 = app.pendulum.p2();

            ctx.draw(&ratatui::widgets::canvas::Line {
                x1: center_x,
                y1: center_y,
                x2: center_x + p1.x,
                y2: center_y + p1.y,
                color: Color::DarkGray,
            });

            ctx.draw(&ratatui::widgets::canvas::Line {
                x1: center_x + p1.x,
                y1: center_y + p1.y,
                x2: center_x + p2.x,
                y2: center_y + p2.y,
                color: Color::Red,
            });
        });

    f.render_widget(spectrum_canvas, main_chunks[0]);

    // Reconstruction View (Right)
    let mut recon_points = vec![];
    let max_val = app
        .reconstruction_data
        .iter()
        .copied()
        .fold(0.0_f64, f64::max);
    let threshold = max_val * 0.5;

    for (i, &val) in app.reconstruction_data.iter().enumerate() {
        if val > threshold {
            let x = (i % app.hologram.width) as f64;
            let y = (i / app.hologram.width) as f64;
            recon_points.push((x, y));
        }
    }

    let recon_canvas = Canvas::default()
        .block(
            Block::default()
                .title(" Reconstruction ")
                .borders(Borders::ALL),
        )
        .marker(ratatui::symbols::Marker::Block)
        .x_bounds([0.0, width])
        .y_bounds([0.0, height])
        .paint(|ctx| {
            ctx.draw(&Points {
                coords: &recon_points,
                color: Color::Green,
            });
        });

    f.render_widget(recon_canvas, main_chunks[1]);
}
