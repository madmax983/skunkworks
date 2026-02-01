#![allow(clippy::collapsible_if)]
mod app;
mod physics;

use app::App;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    Terminal,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    symbols::Marker,
    text::{Line, Span},
    widgets::{
        Block, Borders, Paragraph,
        canvas::{Canvas, Points},
    },
};
use std::{io, time::Duration};
use tui_shared::Tui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    let mut tui = Tui::init()?;

    // Create App
    let mut app = App::new();

    let res = run_app(&mut tui.terminal, &mut app);

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.area());

            // Canvas
            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Fluid Simulation"),
                )
                .x_bounds([0.0, app.solver.width as f64])
                .y_bounds([0.0, app.solver.height as f64])
                .marker(Marker::Braille)
                .paint(|ctx| {
                    let points: Vec<(f64, f64)> = app
                        .solver
                        .particles
                        .iter()
                        .map(|p| (p.x as f64, p.y as f64))
                        // Invert Y for rendering: 100 - y.
                        .map(|(x, y)| (x, 100.0 - y))
                        .collect();

                    ctx.draw(&Points {
                        coords: &points,
                        color: Color::Cyan,
                    });
                });
            f.render_widget(canvas, chunks[0]);

            // Instructions
            let text = vec![Line::from(vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().fg(Color::Red)),
                Span::raw(" to quit, "),
                Span::styled("r", Style::default().fg(Color::Yellow)),
                Span::raw(" to reset, "),
                Span::styled("Space", Style::default().fg(Color::Green)),
                Span::raw(" to spawn particles."),
            ])];
            let info = Paragraph::new(text).block(Block::default().borders(Borders::ALL));
            f.render_widget(info, chunks[1]);
        })?;

        // Updates
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            app.should_quit = true;
                        }
                        KeyCode::Char('r') => {
                            app.reset();
                        }
                        KeyCode::Char(' ') => {
                            app.spawn_particles();
                        }
                        _ => {}
                    }
                }
            }
        }

        if app.should_quit {
            break;
        }

        app.tick();
    }
    Ok(())
}
