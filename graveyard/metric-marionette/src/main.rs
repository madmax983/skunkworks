mod monitor;
mod skeleton;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::Color,
    text::Span,
    widgets::{
        Block, Borders, Paragraph,
        canvas::{Canvas, Circle, Line},
    },
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

use monitor::Monitor;
use skeleton::Skeleton;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    let mut monitor = Monitor::new();
    let mut skeleton = Skeleton::new_humanoid();

    let mut last_tick = Instant::now();

    loop {
        let now = Instant::now();
        let dt = now.duration_since(last_tick).as_secs_f64();
        // Cap dt to avoid explosion if paused
        let dt = dt.min(0.1);
        last_tick = now;

        // Poll events
        #[allow(clippy::collapsible_if)]
        if event::poll(Duration::from_millis(30))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    _ => {}
                }
            }
        }

        // Update
        monitor.refresh();
        skeleton.animate(&monitor, dt);

        // Draw
        tui.terminal.draw(|f| {
            ui(f, &skeleton, &monitor);
        })?;
    }

    Ok(())
}

fn ui(f: &mut Frame, skeleton: &Skeleton, monitor: &Monitor) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Metric Marionette ⚛️💃"),
        )
        .x_bounds([-20.0, 20.0])
        .y_bounds([-20.0, 20.0])
        .paint(|ctx| {
            // Draw skeleton
            let bones = skeleton.solve_fk();
            for (start, end) in bones {
                ctx.draw(&Line {
                    x1: start.x,
                    y1: start.y,
                    x2: end.x,
                    y2: end.y,
                    color: Color::White,
                });

                // Draw joints
                ctx.draw(&Circle {
                    x: end.x,
                    y: end.y,
                    radius: 0.3,
                    color: Color::Cyan,
                });
            }
        });

    f.render_widget(canvas, chunks[0]);

    let status = format!(
        "CPU: {:.1}% | RAM: {:.1}% | Press 'q' to quit",
        monitor.cpu_usage * 100.0,
        monitor.ram_usage * 100.0
    );

    let p = Paragraph::new(Span::raw(status)).block(Block::default().borders(Borders::ALL));
    f.render_widget(p, chunks[1]);
}
