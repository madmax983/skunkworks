mod harvester;
mod physics;
mod platter;

use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
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

use harvester::harvest_functions;
use physics::{Body, Universe, Vec2};

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
    // 1. Harvest
    // Try to scan parent directory to get more interesting data
    let root = if std::path::Path::new("../..").exists() {
        "../.."
    } else {
        "."
    };

    // Scan and limit
    let all_signatures = harvest_functions(root);
    let limit = 150;
    let signatures = if all_signatures.len() > limit {
        // Take a random sample or just first N? First N is fine for stability.
        // Actually, maybe random is better to get variety across the repo.
        // But for now, let's take every Kth element to spread it out.
        let step = all_signatures.len() / limit;
        all_signatures
            .iter()
            .step_by(step)
            .take(limit)
            .cloned()
            .collect()
    } else {
        all_signatures
    };

    // 2. Universe
    let mut universe = Universe::new();

    for (i, sig) in signatures.iter().enumerate() {
        let angle = (i as f32) * 0.1;
        let dist = 10.0 + (i as f32).sqrt() * 5.0; // Spiral out
        let pos_x = angle.cos() * dist;
        let pos_y = angle.sin() * dist;

        let mass = 10.0 + (sig.inputs.len() as f32) * 2.0;
        let radius = mass.sqrt() * 0.5;

        // Color based on return type
        let color = match sig.output.as_str() {
            "Result" => Color::Yellow,
            "Option" => Color::Blue,
            "String" => Color::Green,
            "Vec" => Color::Cyan,
            "bool" => Color::Red,
            "" | "()" => Color::DarkGray,
            _ => Color::White,
        };

        let body = Body::new(pos_x, pos_y, mass, radius, color).with_signature(sig.clone());

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
                    "Ferrous Cladistics | Functions: {} | Magnetic Taxonomy Active",
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

                        // Point
                        let symbol = if body.radius > 4.0 { "O" } else { "o" };
                        // If signature is present, maybe show first letter?
                        let s = if let Some(sig) = &body.signature {
                             // Use first char of name if mass is high enough
                             if body.mass > 15.0 {
                                 sig.name.chars().next().map(|c| c.to_string()).unwrap_or_else(|| "?".to_string())
                             } else {
                                 symbol.to_string()
                             }
                        } else {
                             symbol.to_string()
                        };

                        ctx.print(
                            body.pos.x as f64,
                            body.pos.y as f64,
                            Span::styled(s, Style::default().fg(body.color)),
                        );
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let controls =
                Paragraph::new("Controls: [Q] Quit | [+/-] Zoom | [Arrows] Pan | [R] Reset | Colors: Yel=Result Blu=Opt Grn=Str Red=Bool")
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
