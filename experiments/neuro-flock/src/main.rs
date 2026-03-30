use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::Span,
    widgets::{Block, Borders, Paragraph, canvas::Canvas},
    Terminal,
};

use tui_shared::Tui;

mod boid;
mod world;
use world::World;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    let res = run_app(&mut tui.terminal);

    drop(tui);

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    let mut world = World::new(200.0, 100.0);
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(16);

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(f.area());

            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Neuro-Flock 🧠🐦"))
                .paint(|ctx| {
                    for boid in &world.boids {
                        ctx.print(
                            boid.position.x,
                            boid.position.y,
                            Span::styled(
                                boid.char_representation.to_string(),
                                Style::default().fg(boid.color),
                            ),
                        );
                    }
                })
                .x_bounds([0.0, world.width])
                .y_bounds([0.0, world.height]);

            f.render_widget(canvas, chunks[0]);

            let status = Paragraph::new(format!(
                "Boids: {} | Sync Index: {:.2} | [Q]uit | [R]eset",
                world.boids.len(),
                world.synchronization_index()
            ))
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(status, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char('r') => {
                            world = World::new(200.0, 100.0);
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            world.update();
            last_tick = Instant::now();
        }
    }
}
