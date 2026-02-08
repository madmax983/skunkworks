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
use std::{
    io,
    time::{Duration, Instant},
};

mod physics;
mod simulation;

use simulation::Simulation;

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create simulation
    let mut simulation = Simulation::new();
    simulation.spawn_solar_system();

    let res = run_app(&mut terminal, &mut simulation);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    simulation: &mut Simulation,
) -> anyhow::Result<()>
where
    <B as ratatui::backend::Backend>::Error: Send + Sync + 'static,
{
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(16);

    loop {
        let now = Instant::now();
        let delta = now.duration_since(last_tick).as_secs_f32();
        last_tick = now;

        // Step simulation
        // Limit delta to avoid huge jumps
        let dt = delta.min(0.1);
        simulation.step(dt);

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(f.area());

            // Left: Simulation Canvas
            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Astral Chimera System"),
                )
                .paint(|ctx| {
                    // Draw Radar
                    let radar_end_x = simulation.radar.angle.cos() * simulation.radar.length;
                    let radar_end_y = simulation.radar.angle.sin() * simulation.radar.length;
                    ctx.draw(&CanvasLine {
                        x1: 0.0,
                        y1: 0.0,
                        x2: radar_end_x as f64,
                        y2: radar_end_y as f64,
                        color: Color::Yellow,
                    });

                    // Draw Bodies
                    for body in &simulation.universe.bodies {
                        // Draw orbit trail
                        for point in &body.trail {
                            ctx.print(
                                point.x as f64,
                                point.y as f64,
                                Span::styled(".", Style::default().fg(Color::DarkGray)),
                            );
                        }

                        // Draw Body
                        let char = if body.mass > 100.0 { "O" } else { "o" };
                        ctx.print(
                            body.pos.x as f64,
                            body.pos.y as f64,
                            Span::styled(char, Style::default().fg(body.color)),
                        );
                    }
                })
                .x_bounds([-150.0, 150.0])
                .y_bounds([-100.0, 100.0]);

            f.render_widget(canvas, chunks[0]);

            // Right: VM Status
            let mut lines = vec![
                Line::from(Span::styled(
                    "Astral Chimera",
                    Style::default().fg(Color::Yellow),
                )),
                Line::from("Orbital Code Execution"),
                Line::from(""),
            ];

            if let Some(idx) = simulation.last_executed_vm_index {
                if let Some(body) = simulation.universe.bodies.get(idx) {
                    lines.push(Line::from(format!("Executing: {}", body.name)));
                    if let Some(vm) = &body.vm {
                        lines.push(Line::from(format!("Energy: {}", vm.energy)));
                        // Try to peek stack
                        if let Some(val) = vm.stack.last() {
                            lines.push(Line::from(format!("Stack Top: {:?}", val)));
                        } else {
                            lines.push(Line::from("Stack: [Empty]"));
                        }
                    }
                }
            } else {
                lines.push(Line::from("Scanning..."));
            }

            lines.push(Line::from(""));
            lines.push(Line::from("Press 'q' to quit"));

            let paragraph = Paragraph::new(lines)
                .block(Block::default().borders(Borders::ALL).title("VM State"));
            f.render_widget(paragraph, chunks[1]);
        })?;

        // Input
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }

        // Sleep to maintain framerate
        let elapsed = now.elapsed();
        if elapsed < tick_rate {
            std::thread::sleep(tick_rate - elapsed);
        }
    }
}
