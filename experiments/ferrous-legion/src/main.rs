mod crypto;
mod math;
mod physics;
mod roman;

use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{
        canvas::{Canvas, Line},
        Block, Borders, Paragraph,
    },
    Terminal,
};

use physics::{Body, Universe, Vec2};
use roman::Roman;

fn main() -> anyhow::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Logic
    let res = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> anyhow::Result<()>
where
    <B as Backend>::Error: Send + Sync + 'static,
{
    // 2. Universe
    let mut universe = Universe::new();
    let center = Vec2::ZERO;

    // Spawn random bodies
    let mut rng = rand::thread_rng();
    for _ in 0..50 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let dist = rng.gen_range(10.0..80.0);
        let pos = Vec2::new(angle.cos() * dist, angle.sin() * dist);

        // Random small Roman value
        let val_u64 = rng.gen_range(1..=12); // I to XII
        let value = Roman::from_u64(val_u64);

        let color = if val_u64 >= 10 {
            Color::Green
        } else if val_u64 >= 5 {
            Color::Blue
        } else {
            Color::Gray
        };

        // Tangential kick
        let to_center = (center - pos).normalize_or_zero();
        let tangent = Vec2::new(-to_center.y, to_center.x);
        let orbital_speed = 20.0 + rng.gen_range(-5.0..5.0);

        let body = Body::new(pos.x, pos.y, value, color)
            .with_velocity(tangent.x * orbital_speed, tangent.y * orbital_speed);

        universe.add_body(body);
    }

    // 3. Loop
    let mut zoom = 1.0;
    let mut pan = Vec2::ZERO;
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            let size = f.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(size);

            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title(format!(
                    "Ferrous Legion | Bodies: {} | Magnetic Arithmetic Active",
                    universe.bodies.len()
                )))
                .x_bounds([
                    pan.x as f64 - 100.0 * zoom as f64,
                    pan.x as f64 + 100.0 * zoom as f64,
                ])
                .y_bounds([
                    pan.y as f64 - 100.0 * zoom as f64,
                    pan.y as f64 + 100.0 * zoom as f64,
                ])
                .paint(|ctx| {
                    // Draw Platter (Magnetism)
                    let step = 2;
                    for y in (0..universe.platter.height).step_by(step) {
                        for x in (0..universe.platter.width).step_by(step) {
                            let mag = universe.platter.get_magnetism(x, y);
                            if mag > 0.1 {
                                let px = x as f64 - 100.0;
                                let py = y as f64 - 100.0;

                                let color = match mag {
                                    m if m > 0.8 => Color::Red,
                                    m if m > 0.5 => Color::Magenta,
                                    m if m > 0.2 => Color::Blue,
                                    _ => Color::DarkGray,
                                };
                                ctx.print(px, py, Span::styled("·", Style::default().fg(color)));
                            }
                        }
                    }

                    // Bodies
                    for body in &universe.bodies {
                        // Trail
                        for i in 0..body.trail.len().saturating_sub(1) {
                            ctx.draw(&Line {
                                x1: body.trail[i].x as f64,
                                y1: body.trail[i].y as f64,
                                x2: body.trail[i + 1].x as f64,
                                y2: body.trail[i + 1].y as f64,
                                color: Color::Gray,
                            });
                        }

                        // Point (Roman String)
                        let s = body.value.to_string();
                        ctx.print(
                            body.pos.x as f64,
                            body.pos.y as f64,
                            Span::styled(s, Style::default().fg(body.color)),
                        );
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let controls =
                Paragraph::new("Controls: [Q] Quit | [+/-] Zoom | [Arrows] Pan | [R] Reset")
                    .style(Style::default().fg(Color::White).bg(Color::DarkGray));
            f.render_widget(controls, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('r') => {
                        zoom = 1.0;
                        pan = Vec2::ZERO;
                    }
                    KeyCode::Char('+') => zoom *= 0.9,
                    KeyCode::Char('-') => zoom *= 1.1,
                    KeyCode::Up => pan.y += 10.0 * zoom,
                    KeyCode::Down => pan.y -= 10.0 * zoom,
                    KeyCode::Left => pan.x -= 10.0 * zoom,
                    KeyCode::Right => pan.x += 10.0 * zoom,
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            universe.step(0.05);
            last_tick = Instant::now();
        }
    }
}
