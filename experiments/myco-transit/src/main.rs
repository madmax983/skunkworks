mod simulation;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders,
    },
};
use simulation::World;
use std::{
    time::{Duration, Instant},
};
use tui_shared::Tui;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    // Simulation parameters
    let width = 160;
    let height = 100;
    let num_cities = 8;
    let (mut world, mut agents) = World::with_cities_and_agents(width, height, num_cities);

    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(f.area());

            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Myco-Transit: Slime Mold Urban Planning"))
                .x_bounds([0.0, width as f64])
                .y_bounds([0.0, height as f64])
                .paint(|ctx| {
                    // Draw cities
                    for (cx, cy) in &world.cities {
                        ctx.draw(&Points {
                            coords: &[(*cx, *cy)],
                            color: Color::Red,
                        });
                        // Make cities bigger
                        ctx.draw(&Points {
                            coords: &[(*cx + 1.0, *cy), (*cx - 1.0, *cy), (*cx, *cy + 1.0), (*cx, *cy - 1.0)],
                            color: Color::Red,
                        });
                    }

                    // Draw agents (white)
                    // Collecting coords to avoid multiple draw calls
                    let agent_coords: Vec<(f64, f64)> = agents.iter().map(|a| (a.x, a.y)).collect();
                    ctx.draw(&Points {
                        coords: &agent_coords,
                        color: Color::White,
                    });

                    // Draw trails (green) - optimized
                    // Only draw points with significant trail value
                    let mut trail_coords = Vec::new();
                    for y in 0..world.height {
                        for x in 0..world.width {
                            let val = world.get_trail(x, y);
                            if val > 10.0 {
                                trail_coords.push((x as f64, y as f64));
                            }
                        }
                    }
                    ctx.draw(&Points {
                        coords: &trail_coords,
                        color: Color::Green,
                    });
                });

            f.render_widget(canvas, chunks[0]);

            let status = Line::from(vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().fg(Color::Yellow)),
                Span::raw(" to quit. Agents: "),
                Span::styled(format!("{}", agents.len()), Style::default().fg(Color::Cyan)),
            ]);
            f.render_widget(status, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            // Update simulation
            for agent in &mut agents {
                agent.update(&mut world);
            }
            world.diffuse_and_decay();
            last_tick = Instant::now();
        }
    }

    Ok(())
}
