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
                .block(Block::default().borders(Borders::ALL).title(" Neuro-Fluid Symbiosis "))
                .x_bounds([0.0, universe.width])
                .y_bounds([0.0, universe.height]) // 0 at bottom
                .paint(|ctx| {
                    // Draw Fluid Particles
                    for p in &universe.particles {
                        ctx.print(
                            p.pos.x,
                            p.pos.y,
                            ratatui::text::Span::styled("·", Style::default().fg(Color::Cyan)),
                        );
                    }

                    // Draw Neural Poles
                    for pole in &universe.neuron_poles {
                        let color = if pole.is_spiking { Color::Red } else { Color::DarkGray };
                        let label = if pole.is_spiking { "N" } else { "n" };
                        ctx.print(
                            pole.pos.x,
                            pole.pos.y,
                            ratatui::text::Span::styled(label, Style::default().fg(color).bg(Color::Black)),
                        );
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let active_neurons = universe.neuron_poles.iter().filter(|p| p.is_spiking).count();
            let stats = Paragraph::new(format!(
                "Fluid Particles: {} | Total Neurons: {} | Spiking: {} | [Q/Esc] Quit | [Enter] Kick Neuron",
                universe.particles.len(),
                universe.neuron_poles.len(),
                active_neurons
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
                            KeyCode::Enter => {
                                // Give the network a new kick
                                universe.network.neurons[0].v = 40.0;
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
