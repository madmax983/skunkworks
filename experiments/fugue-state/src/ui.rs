use crate::app::App;
use crate::parser::Timbre;
use ratatui::widgets::canvas::{Canvas, Line as CanvasLine, Rectangle};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    let title_text = format!("Fugue State: Playing {} events", app.events.len());
    let title = Paragraph::new(title_text)
        .block(Block::default().borders(Borders::ALL).title("Information"))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(title, chunks[0]);

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Score"))
        .x_bounds([0.0, app.total_duration as f64])
        .y_bounds([0.0, 1000.0])
        .paint(|ctx| {
            for (i, event) in app.events.iter().enumerate() {
                let start_time = app.event_start_times[i] as f64;
                let duration = event.duration as f64;
                let freq = event.frequency as f64;

                let color = if i == app.current_event_index {
                    Color::Yellow
                } else {
                    match event.timbre {
                        Timbre::Sine => Color::Green,
                        Timbre::Triangle => Color::Magenta,
                        Timbre::Square => Color::Red,
                        Timbre::Sawtooth => Color::Blue,
                    }
                };

                ctx.draw(&Rectangle {
                    x: start_time,
                    y: freq,
                    width: duration,
                    height: 10.0,
                    color,
                });
            }

            ctx.draw(&CanvasLine {
                x1: app.elapsed as f64,
                y1: 0.0,
                x2: app.elapsed as f64,
                y2: 1000.0,
                color: Color::White,
            });
        });

    f.render_widget(canvas, chunks[1]);

    let current_desc = if app.current_event_index < app.events.len() {
        &app.events[app.current_event_index].description
    } else {
        "Finished"
    };

    let status = Paragraph::new(format!(
        "Event: {} | Time: {:.2}s / {:.2}s",
        current_desc, app.elapsed, app.total_duration
    ))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[2]);
}
