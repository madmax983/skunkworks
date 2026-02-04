use std::error::Error;
use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use rayon::prelude::*;
use storm_monitor::lorenz::{LorenzState, LorenzSystem};
use storm_monitor::monitor::SystemMonitor;

// Particle count
const NUM_PARTICLES: usize = 2000;
const TIME_STEP: f64 = 0.005;

struct App {
    monitor: SystemMonitor,
    particles: Vec<LorenzState>,
    // Store colors or velocities if needed
}

impl App {
    fn new() -> Self {
        let monitor = SystemMonitor::new();
        // Initialize particles in a small cloud around a point
        let mut particles = Vec::with_capacity(NUM_PARTICLES);
        for i in 0..NUM_PARTICLES {
            let offset_x = (i as f64 / NUM_PARTICLES as f64) * 2.0 - 1.0;
            let offset_y = ((i % 100) as f64 / 100.0) * 2.0 - 1.0;
            particles.push(LorenzState {
                x: 0.1 + offset_x * 0.1,
                y: 0.1 + offset_y * 0.1,
                z: 0.1,
            });
        }

        Self {
            monitor,
            particles,
        }
    }

    fn on_tick(&mut self) {
        let sigma = self.monitor.get_sigma();
        let rho = self.monitor.get_rho();
        let beta = self.monitor.get_beta();

        let system = LorenzSystem::new(sigma, rho, beta);

        // Update particles in parallel
        self.particles.par_iter_mut().for_each(|p| {
            *p = system.step(*p, TIME_STEP);
        });
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new();

    let tick_rate = Duration::from_millis(30);
    let mut last_tick = Instant::now();
    let mut last_monitor_update = Instant::now();
    let monitor_rate = Duration::from_secs(1);

    loop {
        if last_monitor_update.elapsed() >= monitor_rate {
            app.monitor.update();
            last_monitor_update = Instant::now();
        }

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(f.area());

            // Get params for display
            let sigma = app.monitor.get_sigma();
            let rho = app.monitor.get_rho();
            let beta = app.monitor.get_beta();
            let cpu = app.monitor.get_cpu_usage();
            let mem = app.monitor.get_memory_usage();

            // Canvas
            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Storm Monitor: Lorenz Attractor"))
                .x_bounds([-30.0, 30.0])
                .y_bounds([0.0, 60.0])
                .paint(|ctx| {
                    // Draw particles projected on X-Z plane
                    // X is x, Y is z (because Z goes up in Lorenz usually)
                    // Let's iterate and draw points.
                    // Ratatui Points widget takes slice of (f64, f64).

                    let points: Vec<(f64, f64)> = app.particles.iter()
                        .map(|p| (p.x, p.z))
                        .collect();

                    ctx.draw(&Points {
                        coords: &points,
                        color: Color::Cyan,
                    });
                });

            f.render_widget(canvas, chunks[0]);

            // Status Bar
            let status = Paragraph::new(vec![
                Line::from(vec![
                    Span::styled("CPU: ", Style::default().fg(Color::Red)),
                    Span::raw(format!("{:.1}% ", cpu)),
                    Span::styled("σ (Sigma): ", Style::default().fg(Color::Red)),
                    Span::raw(format!("{:.2} | ", sigma)),

                    Span::styled("MEM: ", Style::default().fg(Color::Green)),
                    Span::raw(format!("{:.1}% ", mem * 100.0)),
                    Span::styled("ρ (Rho): ", Style::default().fg(Color::Green)),
                    Span::raw(format!("{:.2} | ", rho)),

                    Span::styled("β (Beta): ", Style::default().fg(Color::Blue)),
                    Span::raw(format!("{:.2}", beta)),
                ]),
                Line::from("Press 'q' to quit"),
            ]).block(Block::default().borders(Borders::ALL));

            f.render_widget(status, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
