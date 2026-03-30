mod physics;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use physics::Universe;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let res = run_app(&mut tui);
    tui.exit()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app(tui: &mut Tui) -> Result<()> {
    // 200x100 world
    let mut universe = Universe::new(200.0, 100.0);
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(33);

    loop {
        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.area());

            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title(" Neuro Fluid "))
                .x_bounds([0.0, universe.width])
                .y_bounds([0.0, universe.height]) // 0 at bottom
                .paint(|ctx| {
                    // Draw Particles
                    for p in &universe.particles {
                        ctx.print(
                            p.pos.x,
                            p.pos.y,
                            ratatui::text::Span::styled("·", Style::default().fg(Color::Cyan)),
                        );
                    }

                    // Draw Magnets
                    for mag in &universe.magnets {
                        let is_spiking = universe.network.is_spiking(mag.neuron_id);
                        let color = if is_spiking {
                            Color::Yellow
                        } else if mag.polarity {
                            Color::Red
                        } else {
                            Color::Blue
                        };
                        let label = if is_spiking { "*" } else if mag.polarity { "N" } else { "S" };
                        ctx.print(
                            mag.pos.x,
                            mag.pos.y,
                            ratatui::text::Span::styled(label, Style::default().fg(color)),
                        );
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let stats = Paragraph::new(format!(
                "Particles: {} | Neurons/Magnets: {} | [SPACE] Inject Chaos | [R] Reset | [Q] Quit",
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
                                // Inject chaos/current
                                for mag in &universe.magnets {
                                    universe.network.neurons[mag.neuron_id].v = 40.0;
                                }
                            }
                            KeyCode::Char('r') => {
                                universe = Universe::new(200.0, 100.0);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            universe.tick();
            last_tick = Instant::now();
        }
    }
}
