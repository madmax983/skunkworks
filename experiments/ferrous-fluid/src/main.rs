mod physics;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use physics::Universe;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, event::EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        event::DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<()>
where
    <B as Backend>::Error: Send + Sync + 'static,
{
    // 200x100 world
    let mut universe = Universe::new(200.0, 100.0);
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(33);

    // Initial magnet
    universe.add_magnet(100.0, 50.0, true);

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.area());

            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title(" Ferrous Fluid "))
                .x_bounds([0.0, universe.width as f64])
                .y_bounds([0.0, universe.height as f64]) // 0 at bottom
                .paint(|ctx| {
                    // Draw Particles
                    for p in &universe.particles {
                        ctx.print(
                            p.pos.x as f64,
                            p.pos.y as f64,
                            ratatui::text::Span::styled("·", Style::default().fg(Color::Cyan)),
                        );
                    }

                    // Draw Magnets
                    for mag in &universe.magnets {
                        let color = if mag.polarity { Color::Red } else { Color::Blue };
                        let label = if mag.polarity { "N" } else { "S" };
                        ctx.print(
                            mag.pos.x as f64,
                            mag.pos.y as f64,
                            ratatui::text::Span::styled(label, Style::default().fg(color).bg(Color::White)),
                        );
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let stats = Paragraph::new(format!(
                "Particles: {} | Magnets: {} | [WASD] Move Magnet | [Space] Toggle | [Enter] Add Magnet | [Q] Quit",
                universe.particles.len(),
                universe.magnets.len()
            ))
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(stats, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                            KeyCode::Char(' ') => {
                                if let Some(last) = universe.magnets.last_mut() {
                                    last.polarity = !last.polarity;
                                }
                            }
                            KeyCode::Char('r') => {
                                universe = Universe::new(200.0, 100.0);
                                universe.add_magnet(100.0, 50.0, true);
                            }
                            KeyCode::Enter => {
                                universe.add_magnet(100.0, 50.0, true);
                            }
                            KeyCode::Char('w') => {
                                if let Some(last) = universe.magnets.last_mut() {
                                    last.pos.y += 5.0;
                                }
                            }
                            KeyCode::Char('s') => {
                                if let Some(last) = universe.magnets.last_mut() {
                                    last.pos.y -= 5.0;
                                }
                            }
                            KeyCode::Char('a') => {
                                if let Some(last) = universe.magnets.last_mut() {
                                    last.pos.x -= 5.0;
                                }
                            }
                            KeyCode::Char('d') => {
                                if let Some(last) = universe.magnets.last_mut() {
                                    last.pos.x += 5.0;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            universe.update(0.05); // Fixed time step
            last_tick = Instant::now();
        }
    }
}
