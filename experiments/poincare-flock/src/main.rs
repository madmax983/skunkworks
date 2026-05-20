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
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Hyperbolic Swarm "),
                )
                .x_bounds([0.0, universe.width])
                .y_bounds([0.0, universe.height]) // 0 at bottom
                .paint(|ctx| {
                    let center_x = universe.width / 2.0;
                    let center_y = universe.height / 2.0;
                    let r = (universe.width.min(universe.height) / 2.0) - 2.0;

                    // Draw disk boundary rough approximation
                    for i in 0..60 {
                        let angle = (i as f64 / 60.0) * std::f64::consts::PI * 2.0;
                        ctx.print(
                            center_x + r * angle.cos(),
                            center_y + r * angle.sin(),
                            ratatui::text::Span::styled(".", Style::default().fg(Color::DarkGray)),
                        );
                    }

                    // Draw Particles
                    for p in &universe.particles {
                        let color = if p.vel.magnitude() > universe.params.max_speed * 0.8 {
                            Color::Red
                        } else if p.vel.magnitude() > universe.params.max_speed * 0.4 {
                            Color::Yellow
                        } else {
                            Color::Cyan
                        };
                        ctx.print(
                            p.pos.x,
                            p.pos.y,
                            ratatui::text::Span::styled("v", Style::default().fg(color)),
                        );
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let stats = Paragraph::new(format!("Boids: {} | [Q] Quit", universe.particles.len(),))
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
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            universe.update(0.05);
            last_tick = Instant::now();
        }
    }
}
