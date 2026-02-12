use std::io::stdout;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use glam::Vec3;
use rand::Rng;

mod physics;
use physics::BioString;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    // Initialize BioString
    // Start (-40, 0, 0) to (40, 0, 0)
    let start = Vec3::new(-40.0, 0.0, 0.0);
    let end = Vec3::new(40.0, 0.0, 0.0);
    let mut string = BioString::new(start, end, 80, 20.0, 0.5); // 80 segments

    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(f.area());

            // Render Canvas
            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Biomorphic Strings"))
                .x_bounds([-50.0, 50.0])
                .y_bounds([-25.0, 25.0])
                .paint(|ctx| {
                    for i in 0..string.nodes.len() - 1 {
                        let n1 = &string.nodes[i];
                        let n2 = &string.nodes[i + 1];

                        // Project to 2D (just x, y for now, maybe rotate later)
                        let p1 = (n1.pos.x as f64, n1.pos.y as f64);
                        let p2 = (n2.pos.x as f64, n2.pos.y as f64);

                        // Color based on 'v' concentration
                        // v ranges 0.0 to 1.0 (approx)
                        // Low v = Green, High v = Blue/Cyan
                        let v_avg = (n1.v + n2.v) * 0.5;
                        let color = if v_avg > 0.5 {
                            Color::Cyan
                        } else if v_avg > 0.2 {
                            Color::Blue
                        } else {
                            Color::Green
                        };

                        // Thickness/Character based on mass?
                        // Canvas lines are single pixel width usually.

                        ctx.draw(&CanvasLine {
                            x1: p1.0,
                            y1: p1.1,
                            x2: p2.0,
                            y2: p2.1,
                            color,
                        });
                    }
                });
            f.render_widget(canvas, chunks[0]);

            // Status Bar
            let status = Paragraph::new(vec![
                Line::from(vec![
                    Span::raw("Controls: "),
                    Span::styled("Space", Style::default().fg(Color::Yellow)),
                    Span::raw(" to Pluck | "),
                    Span::styled("R", Style::default().fg(Color::Yellow)),
                    Span::raw(" to Reset | "),
                    Span::styled("Q", Style::default().fg(Color::Yellow)),
                    Span::raw(" to Quit"),
                ]),
                Line::from(vec![
                    Span::raw(format!("Nodes: {} | Tension: {:.1} | Feed: {:.3} | Kill: {:.3}",
                        string.nodes.len(), string.tension, string.feed, string.kill)),
                ]),
            ])
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(status, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('r') => {
                         string = BioString::new(start, end, 80, 20.0, 0.5);
                    }
                    KeyCode::Char(' ') => {
                        // Pluck a random node
                        let mut rng = rand::thread_rng();
                        let idx = rng.gen_range(1..string.nodes.len()-1);
                        let force = Vec3::new(0.0, rng.gen_range(10.0..50.0), 0.0);
                        string.pluck(idx, force);
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            // Physics Update (sub-step for stability)
            let dt = 0.016; // 60 FPS
            // Run physics 5 times per frame for stability
            for _ in 0..5 {
                string.update(dt / 5.0);
            }
            last_tick = Instant::now();
        }
    }
}
