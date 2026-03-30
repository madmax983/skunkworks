use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{
        canvas::{Canvas},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::{io, time::Duration};

mod git;
mod hologram;
use git::{get_commit_history, Commit};
use hologram::Hologram;

struct App {
    hologram: Hologram,
    reconstruction_angle_x: isize,
    reconstruction_angle_y: isize,
    commits: Vec<Commit>,
    status_msg: String,
    reconstruction_data: Vec<f64>,
    width: usize,
    height: usize,
}

impl App {
    fn new() -> Self {
        let width = 128;
        let height = 64;
        let commits = get_commit_history().unwrap_or_default();

        let mut app = Self {
            hologram: Hologram::new(width, height),
            reconstruction_angle_x: -20,
            reconstruction_angle_y: -10,
            commits,
            status_msg: "Use Arrow Keys to adjust Angle. R to Reset.".into(),
            reconstruction_data: vec![],
            width,
            height,
        };
        app.update_hologram();
        app
    }

    fn update_hologram(&mut self) {
        let mut grid = vec![0.0; self.width * self.height];

        // Map commits to spatial density field
        for (i, commit) in self.commits.iter().enumerate() {
            // Very simple pseudo-hash mapping for coordinates
            let x_hash = commit.hash.chars().nth(0).unwrap_or('a') as usize;
            let y_hash = commit.hash.chars().nth(1).unwrap_or('a') as usize;

            // Map author string length to intensity
            let intensity = (commit.author.len() as f64).clamp(1.0, 10.0) / 2.0;

            // Distribute across grid, adding some temporal spread based on index `i`
            let cx = (x_hash * 13 + i * 5) % self.width;
            let cy = (y_hash * 7 + i * 3) % self.height;

            // Drop a localized Gaussian-like density blob
            for dy in -2..=2 {
                for dx in -2..=2 {
                    let px = cx as isize + dx;
                    let py = cy as isize + dy;
                    if px >= 0 && px < self.width as isize && py >= 0 && py < self.height as isize {
                        let dist_sq = (dx*dx + dy*dy) as f64;
                        let weight = (-dist_sq / 2.0).exp();
                        grid[py as usize * self.width + px as usize] += intensity * weight;
                    }
                }
            }
        }

        self.hologram = Hologram::from_density_field(&grid, self.width, self.height);
        self.update_reconstruction();
    }

    fn update_reconstruction(&mut self) {
        self.reconstruction_data = self
            .hologram
            .reconstruct(self.reconstruction_angle_x, self.reconstruction_angle_y);
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()>
where std::io::Error: From<<B as Backend>::Error>, <B as Backend>::Error: Send + Sync + std::error::Error + 'static {
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = std::time::Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Right => {
                            app.reconstruction_angle_x += 1;
                            app.update_reconstruction();
                        }
                        KeyCode::Left => {
                            app.reconstruction_angle_x -= 1;
                            app.update_reconstruction();
                        }
                        KeyCode::Up => {
                            app.reconstruction_angle_y -= 1;
                            app.update_reconstruction();
                        }
                        KeyCode::Down => {
                            app.reconstruction_angle_y += 1;
                            app.update_reconstruction();
                        }
                        KeyCode::Char('r') | KeyCode::Char('R') => {
                            app.reconstruction_angle_x = -20;
                            app.reconstruction_angle_y = -10;
                            app.update_reconstruction();
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = std::time::Instant::now();
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.area());

    let header_text = format!(
        " 🧬 Git Hologram | Angle: ({}, {}) | Commits: {} | {}",
        app.reconstruction_angle_x, app.reconstruction_angle_y, app.commits.len(), app.status_msg
    );

    let p = Paragraph::new(header_text).block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(p, chunks[0]);

    let display_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    // 1. Frequency Domain (Interference Pattern)
    let mag = app.hologram.get_magnitude();
    let max_mag = mag.iter().cloned().fold(0.0_f64, f64::max).max(1.0);

    let canvas_freq = Canvas::default()
        .block(Block::default().title(" Interference Pattern (FFT Magnitude) ").borders(Borders::ALL))
        .x_bounds([0.0, app.width as f64])
        .y_bounds([0.0, app.height as f64])
        .paint(|ctx| {
            for y in 0..app.height {
                for x in 0..app.width {
                    let v = mag[y * app.width + x] / max_mag;
                    if v > 0.1 {
                        ctx.print(x as f64, (app.height - 1 - y) as f64, "█".to_string());
                    }
                }
            }
        });
    f.render_widget(canvas_freq, display_chunks[0]);

    // 2. Spatial Domain (Reconstruction)
    let recon = &app.reconstruction_data;
    let max_recon = recon.iter().cloned().fold(0.0_f64, f64::max).max(1.0);

    let canvas_recon = Canvas::default()
        .block(Block::default().title(" Holographic Reconstruction ").borders(Borders::ALL))
        .x_bounds([0.0, app.width as f64])
        .y_bounds([0.0, app.height as f64])
        .paint(|ctx| {
            for y in 0..app.height {
                for x in 0..app.width {
                    let v = recon[y * app.width + x] / max_recon;
                    if v > 0.05 {
                        ctx.print(x as f64, (app.height - 1 - y) as f64, "█".to_string());
                    }
                }
            }
        });
    f.render_widget(canvas_recon, display_chunks[1]);
}
