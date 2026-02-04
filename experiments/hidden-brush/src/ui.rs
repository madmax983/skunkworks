use crate::vm::Turtle;
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color},
    symbols,
    widgets::{canvas::{Canvas, Line}, Block, Borders, Paragraph},
    Terminal,
};
use std::{io, time::{Duration, Instant}};

pub fn run(turtle: Turtle) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, turtle);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, turtle: Turtle) -> Result<()> {
    let path = turtle.path;
    let mut visible_count = 0;
    let total_segments = path.len();
    let tick_rate = Duration::from_millis(16); // ~60 FPS
    let mut last_tick = Instant::now();

    // Calculate bounds
    let mut min_x = 0.0;
    let mut max_x = 0.0;
    let mut min_y = 0.0;
    let mut max_y = 0.0;

    if !path.is_empty() {
        // Initialize with first point
        min_x = path[0].start.0;
        max_x = path[0].start.0;
        min_y = path[0].start.1;
        max_y = path[0].start.1;
    }

    for seg in &path {
        min_x = min_x.min(seg.start.0).min(seg.end.0);
        max_x = max_x.max(seg.start.0).max(seg.end.0);
        min_y = min_y.min(seg.start.1).min(seg.end.1);
        max_y = max_y.max(seg.start.1).max(seg.end.1);
    }

    // Add margin
    let margin = 20.0;
    min_x -= margin;
    max_x += margin;
    min_y -= margin;
    max_y += margin;

    // Determine speed
    let speed = if total_segments > 0 {
        (total_segments / 300).max(1) // Finish in ~5 seconds (300 frames)
    } else {
        1
    };

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(f.area());

            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Hidden Brush Viewer"))
                .marker(symbols::Marker::Braille)
                .x_bounds([min_x, max_x])
                .y_bounds([min_y, max_y])
                .paint(|ctx| {
                    for i in 0..visible_count {
                        if i >= path.len() { break; }
                        let seg = &path[i];
                        ctx.draw(&Line {
                            x1: seg.start.0,
                            y1: seg.start.1,
                            x2: seg.end.0,
                            y2: seg.end.1,
                            color: Color::Rgb(seg.color.0, seg.color.1, seg.color.2),
                        });
                    }
                    // Draw turtle head
                    if visible_count > 0 && visible_count <= path.len() {
                         let head = &path[visible_count - 1].end;
                         ctx.print(head.0, head.1, "O");
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let status = format!("Progress: {}/{} segments | Bounds: [{:.1}, {:.1}] x [{:.1}, {:.1}] | Speed: {} seg/frame", visible_count, total_segments, min_x, max_x, min_y, max_y, speed);
            f.render_widget(
                Paragraph::new(status).block(Block::default().borders(Borders::ALL).title("Info")),
                chunks[1]
            );

        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            if visible_count < total_segments {
                visible_count += speed;
                if visible_count > total_segments { visible_count = total_segments; }
            }
            last_tick = Instant::now();
        }
    }
}
