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
    symbols::Marker,
    text::Span,
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

mod biology;
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
    let tick_rate = Duration::from_millis(33);

    // Initialize Simulation
    // FluidSolver width/height
    let width = 100.0;
    let height = 100.0;
    let mut solver = FluidSolver::new(width, height);

    // Seed initial population
    let mut rng = rand::thread_rng();
    use rand::Rng;

    // 50 Algae
    for _ in 0..50 {
        solver.add_particle(
            rng.gen_range(0.0..width),
            rng.gen_range(0.0..height * 0.3),
            Species::Algae,
        );
    }
    // 10 Grazers
    for _ in 0..10 {
        solver.add_particle(
            rng.gen_range(0.0..width),
            rng.gen_range(height * 0.3..height),
            Species::Grazer,
        );
    }
    // 2 Predators
    for _ in 0..2 {
        solver.add_particle(
            rng.gen_range(0.0..width),
            rng.gen_range(0.0..height),
            Species::Predator,
        );
    }

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
                .split(f.area());

            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Primordial Soup (q to quit) "),
                )
                .x_bounds([0.0, width as f64])
                .y_bounds([0.0, height as f64])
                .marker(Marker::Block)
                .paint(|ctx| {
                    for p in &solver.particles {
                        let color = match p.species {
                            Species::Algae => Color::Green,
                            Species::Grazer => Color::Cyan,
                            Species::Predator => Color::Red,
                        };

                        ctx.print(
                            p.x as f64,
                            p.y as f64,
                            Span::styled("█", Style::default().fg(color)),
                        );
                    }
                });

            f.render_widget(canvas, chunks[0]);

            // Stats
            let algae_count = solver
                .particles
                .iter()
                .filter(|p| p.species == Species::Algae)
                .count();
            let grazer_count = solver
                .particles
                .iter()
                .filter(|p| p.species == Species::Grazer)
                .count();
            let predator_count = solver
                .particles
                .iter()
                .filter(|p| p.species == Species::Predator)
                .count();

            let stats = format!(
                "Algae: {} | Grazers: {} | Predators: {} | Total: {}",
                algae_count,
                grazer_count,
                predator_count,
                solver.particles.len()
            );

            let paragraph = Paragraph::new(stats).style(Style::default().fg(Color::White));
            f.render_widget(paragraph, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            solver.update(0.5); // dt
            solver.update_biology();
            last_tick = Instant::now();
        }
    }
}
