pub mod sexagesimal;
pub mod forecaster;

#[cfg(test)]
mod tests;

use std::io::{self, stdout, Stdout};
use std::time::Duration;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, symbols::Marker, widgets::*};

use crate::sexagesimal::Sexagesimal;
use crate::forecaster::TimeSeries;
use rand::Rng;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    // Explicitly use CrosstermBackend<Stdout> to avoid generic error issues with anyhow
    let res = run_app(Terminal::new(CrosstermBackend::new(stdout))?);

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    // Generate data
    // Random walk starting at 60 (1, 0)
    let mut data = Vec::new();
    let mut current: i64 = 60;
    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        current += rng.gen_range(-5..=5);
        if current < 1 { current = 1; }
        data.push(Sexagesimal::from_u64(current as u64));
    }

    let ts = TimeSeries::new(data);
    let ma = ts.moving_average(5);

    // Convert for plotting
    let raw_data_points: Vec<(f64, f64)> = ts.data.iter().enumerate()
        .map(|(i, v)| (i as f64, v.to_f64()))
        .collect();

    let window_size = 5;
    let ma_points: Vec<(f64, f64)> = ma.data.iter().enumerate()
        .map(|(i, v)| ((i + window_size - 1) as f64, v.to_f64()))
        .collect();

    loop {
        terminal.draw(|f| {
            let area = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0)].as_ref())
                .split(area);

            // Chart
            // Generate labels
            // X Axis: 0 to 100
            // We want labels every 10 steps.
            // 0, 10, 20... 100
            let x_labels: Vec<Span> = (0..11).map(|i| {
                let val = i * 10;
                let s = if val == 0 { Sexagesimal::zero() } else { Sexagesimal::from_u64(val as u64) };
                Span::styled(format!("{}", s), Style::default().add_modifier(Modifier::BOLD))
            }).collect();

            // Y Axis: 0 to 100 (assuming data stays in range)
            let y_labels: Vec<Span> = (0..11).map(|i| {
                let val = i * 10;
                let s = if val == 0 { Sexagesimal::zero() } else { Sexagesimal::from_u64(val as u64) };
                Span::styled(format!("{}", s), Style::default().add_modifier(Modifier::BOLD))
            }).collect();

            let datasets = vec![
                Dataset::default()
                    .name("Raw Data")
                    .marker(Marker::Dot)
                    .style(Style::default().fg(Color::Cyan))
                    .data(&raw_data_points),
                Dataset::default()
                    .name("Moving Average (5)")
                    .marker(Marker::Braille)
                    .style(Style::default().fg(Color::Yellow))
                    .data(&ma_points),
            ];

            let chart = Chart::new(datasets)
                .block(Block::default().title("Babylonian Forecast (base-60)").borders(Borders::ALL))
                .x_axis(Axis::default()
                    .title("Time")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 100.0])
                    .labels(x_labels))
                .y_axis(Axis::default()
                    .title("Value")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 150.0]) // Increased bound slightly
                    .labels(y_labels));

            f.render_widget(chart, chunks[0]);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
    }
}
