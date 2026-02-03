mod fish;
mod pond;

use crate::pond::Pond;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseButton, MouseEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Line},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    // Initial size guess
    let size = tui.terminal.size()?;
    // We use a larger coordinate space for smooth movement
    let mut pond = Pond::new(size.width as f64 * 2.0, size.height as f64 * 4.0, 30);

    let tick_rate = Duration::from_millis(30);
    let mut last_tick = Instant::now();
    let mut running = true;

    while running {
        tui.terminal.draw(|f| ui(f, &mut pond))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => running = false,
                            KeyCode::Char('f') => {
                                // Add random food/fish
                                use rand::Rng;
                                let mut rng = rand::thread_rng();
                                pond.add_food(
                                    rng.gen_range(0.0..pond.params.width),
                                    rng.gen_range(0.0..pond.params.height),
                                );
                            }
                            _ => {}
                        }
                    }
                }
                Event::Mouse(mouse) => {
                    if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
                        // Map mouse coordinates to pond coordinates
                        // This is tricky because the UI layout changes.
                        // For now, let's just spawn random food or try to map roughly.
                        // We'll skip precise mouse mapping for this iteration.
                        use rand::Rng;
                        let mut rng = rand::thread_rng();
                        pond.add_food(
                            rng.gen_range(0.0..pond.params.width),
                            rng.gen_range(0.0..pond.params.height),
                        );
                    }
                }
                Event::Resize(w, h) => {
                    // Update pond boundaries
                    pond.resize(w as f64 * 2.0, h as f64 * 4.0);
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            pond.update();
            last_tick = Instant::now();
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, pond: &mut Pond) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Digital Koi"))
        .x_bounds([0.0, pond.params.width])
        .y_bounds([0.0, pond.params.height])
        .paint(|ctx| {
            for fish in &pond.fish {
                // Draw fish body (head)
                // ctx.print(fish.x, fish.y, Span::styled("●", Style::default().fg(fish.color)));

                // Draw trail
                if fish.history.len() >= 2 {
                    for i in 0..fish.history.len() - 1 {
                        let (x1, y1) = fish.history[i];
                        let (x2, y2) = fish.history[i + 1];
                        // Fade out trail? Canvas doesn't support gradient lines easily.
                        // We just draw lines.
                        ctx.draw(&Line {
                            x1,
                            y1,
                            x2,
                            y2,
                            color: fish.color,
                        });
                    }
                }
            }
        });

    f.render_widget(canvas, chunks[0]);

    // Status Bar
    let status_text = format!(
        "Fish: {} | Turb: {:.2} | [q] Quit | [f] Feed",
        pond.fish.len(),
        pond.params.turbulence,
    );
    let status =
        Paragraph::new(status_text).style(Style::default().bg(Color::Blue).fg(Color::White));
    f.render_widget(status, chunks[1]);
}
