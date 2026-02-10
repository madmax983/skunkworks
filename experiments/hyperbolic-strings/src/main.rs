use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
    Terminal,
};
use std::{
    io::{self, Stdout},
    time::{Duration, Instant},
};
use num_complex::Complex;
use poincare_disk::{Point, Mobius};
use rand::Rng;

mod physics;
mod render;

use physics::HyperbolicString;
use render::Camera;

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let res = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    // Initialize Simulation
    // String from (-0.5, 0) to (0.5, 0)
    let start = Point::new(-0.5, 0.0);
    let end = Point::new(0.5, 0.0);
    let mut string = HyperbolicString::new(start, end, 20, 50.0, 0.5); // 20 segments, Tension 50, Damping 0.5

    let mut camera = Camera::new();

    let mut last_tick = Instant::now();
    loop {
        let now = Instant::now();
        let dt = now.duration_since(last_tick).as_secs_f32();
        if dt >= 0.016 {
            string.update(dt.min(0.05) as f64);
            last_tick = now;
        }

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(f.area());

            // Render Canvas
            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Hyperbolic Strings"))
                .x_bounds([-1.1, 1.1])
                .y_bounds([-1.1, 1.1])
                .paint(|ctx| {
                    render::draw_hyperbolic_string(ctx, &string, &camera);

                    // Draw disk boundary
                    // (Already handled in draw_hyperbolic_string but good to be sure)
                });
            f.render_widget(canvas, chunks[0]);

            // Render Info
            let info_text = format!(
                "Tension: {:.1} | Damping: {:.2} | Nodes: {}\nControls: WASD (Move), [Space] Pluck, [R] Reset, [Q] Quit",
                string.tension,
                string.damping,
                string.nodes.len()
            );
            let info = Paragraph::new(info_text)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(info, chunks[1]);
        })?;

        // Handle Input
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char('w') | KeyCode::Up => {
                            // Move forward (translate in +Y direction)
                            let m = Mobius::translation(Complex::new(0.0, 0.05));
                            camera.transform = camera.transform.then(&m);
                        }
                        KeyCode::Char('s') | KeyCode::Down => {
                            // Move backward
                            let m = Mobius::translation(Complex::new(0.0, -0.05));
                            camera.transform = camera.transform.then(&m);
                        }
                        KeyCode::Char('a') | KeyCode::Left => {
                            // Move left
                            let m = Mobius::translation(Complex::new(-0.05, 0.0));
                            camera.transform = camera.transform.then(&m);
                        }
                        KeyCode::Char('d') | KeyCode::Right => {
                            // Move right
                            let m = Mobius::translation(Complex::new(0.05, 0.0));
                            camera.transform = camera.transform.then(&m);
                        }
                        KeyCode::Char(' ') => {
                            // Pluck random node
                            let mut rng = rand::thread_rng();
                            let idx = rng.gen_range(1..string.nodes.len() - 1);
                            // Random force vector
                            let angle = rng.gen_range(0.0..std::f64::consts::PI * 2.0);
                            let mag = rng.gen_range(5.0..15.0);
                            let force = Complex::from_polar(mag, angle);
                            string.pluck(idx, force);
                        }
                        KeyCode::Char('r') => {
                            // Reset
                            let start = Point::new(-0.5, 0.0);
                            let end = Point::new(0.5, 0.0);
                            string = HyperbolicString::new(start, end, 20, 50.0, 0.5);
                            camera = Camera::new();
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
