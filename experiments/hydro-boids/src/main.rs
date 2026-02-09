use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
    Terminal,
};
use std::{
    f32::consts::PI,
    io,
    time::{Duration, Instant},
};

mod physics;
use physics::{FluidSolver, Species};

fn main() -> Result<()> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run App
    let res = run_app(&mut terminal);

    // Restore Terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(33); // ~30 FPS

    let mut last_frame = Instant::now();

    // Simulation width/height
    let width = 100.0;
    let height = 100.0;

    let mut solver = FluidSolver::new(width, height);

    // Seed Fluid
    // Create a pool at the bottom (Y=0 is bottom in Ratatui Canvas)
    // Seed from 0 to 40
    for x in (0..100).step_by(3) {
        for y in (0..40).step_by(3) {
            solver.add_particle(x as f32, y as f32, Species::Fluid);
        }
    }

    // Seed Boids
    // A flock in the air/water
    use rand::Rng;
    let mut rng = rand::thread_rng();
    for _ in 0..30 {
        solver.add_particle(
            rng.gen_range(20.0..80.0),
            rng.gen_range(50.0..80.0), // Seed higher up
            Species::Boid,
        );
    }

    loop {
        let current_time = Instant::now();
        let frame_duration = current_time.duration_since(last_frame);
        last_frame = current_time;

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
                .split(f.area());

            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Hydro-Boids (q to quit) "),
                )
                .x_bounds([0.0, width as f64])
                .y_bounds([0.0, height as f64])
                .paint(|ctx| {
                    for p in &solver.particles {
                        let (char, color) = match p.species {
                            Species::Fluid => {
                                // Density visualization
                                if p.rho > 0.15 {
                                    ('#', Color::Blue)
                                } else if p.rho > 0.10 {
                                    ('=', Color::Blue)
                                } else if p.rho > 0.05 {
                                    ('-', Color::Cyan)
                                } else {
                                    ('.', Color::DarkGray)
                                }
                            }
                            Species::Boid => {
                                // Directional character
                                // Y is Up.
                                let angle = p.vy.atan2(p.vx);
                                let c = if angle.abs() < PI / 4.0 {
                                    '>'
                                }
                                // Right
                                else if (angle - PI / 2.0).abs() < PI / 4.0 {
                                    '^'
                                }
                                // Up
                                else if (angle + PI / 2.0).abs() < PI / 4.0 {
                                    'v'
                                }
                                // Down
                                else {
                                    '<'
                                }; // Left
                                (c, Color::Yellow)
                            }
                        };
                        ctx.print(
                            p.x as f64,
                            p.y as f64,
                            Span::styled(char.to_string(), Style::default().fg(color)),
                        );
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let boid_count = solver
                .particles
                .iter()
                .filter(|p| p.species == Species::Boid)
                .count();
            let fluid_count = solver
                .particles
                .iter()
                .filter(|p| p.species == Species::Fluid)
                .count();
            // Simple FPS
            let fps = 1.0 / frame_duration.as_secs_f64().max(0.001);

            let status = format!(
                "Boids: {} | Fluid: {} | FPS: {:.0}",
                boid_count, fluid_count, fps
            );
            f.render_widget(
                Paragraph::new(status).style(Style::default().bg(Color::White).fg(Color::Black)),
                chunks[1],
            );
        })?;

        // Input
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }

        // Update Physics
        if last_tick.elapsed() >= tick_rate {
            solver.update(0.5);
            last_tick = Instant::now();
        }
    }
}
