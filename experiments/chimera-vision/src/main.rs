use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, MouseEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::canvas::{Canvas, Context, Rectangle},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};
use tui_shared::Tui;

mod agent;
mod app;
mod eye;
mod physics;

use app::{App, Mode};

fn main() -> Result<()> {
    // Setup terminal
    let mut tui = Tui::init()?;

    // Create app
    let size = tui.terminal.size()?;
    let mut app = App::new(size.width as f32, size.height as f32);

    let res = run_app(&mut tui.terminal, &mut app);

    // Restore terminal
    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('m') => app.toggle_mode(),
                    _ => {}
                },
                Event::Mouse(mouse) => match mouse.kind {
                    MouseEventKind::Moved | MouseEventKind::Drag(_) | MouseEventKind::Down(_) => {
                        // TUI y=0 is top. Physics y=0 is top.
                        // ratatui Canvas y=0 is bottom.
                        // We need to map Mouse Y (Top=0) to Canvas Y (Bottom=0).
                        // BUT app logic stores Eye in physics coords (Top=0).
                        // So we pass raw mouse coords to app.
                        app.on_mouse(mouse.column, mouse.row);
                    }
                    _ => {}
                },
                Event::Resize(w, h) => {
                    app.resize(w as f32, h as f32);
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(f.area());

    // Physics Y grows DOWN (0 is top).
    // Canvas Y grows UP (0 is bottom).
    // So we flip Y when rendering: draw_y = height - phys_y.
    let height = app.height as f64;

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Chimera Vision 🧬👁️"),
        )
        .x_bounds([0.0, app.width as f64])
        .y_bounds([0.0, height]) // 0 at bottom, height at top
        .paint(|ctx: &mut Context| {
            // Draw Chaos Field (Physics Nodes)
            for node in &app.system.nodes {
                if node.fixed { continue; }

                let chaos = app.system.get_chaos_level(node.pos);
                let color = if chaos > 10.0 {
                    Color::Red
                } else if chaos > 2.0 {
                    Color::Yellow
                } else {
                    Color::DarkGray
                };

                ctx.print(
                    node.pos.x as f64,
                    height - node.pos.y as f64,
                    Span::styled(".", Style::default().fg(color)),
                );
            }

            // Draw Agents
            for agent in &app.agents {
                let (r, g, b) = agent.dna_color;
                // Approximate RGB to TUI Color
                // Since ratatui Color::Rgb exists, use it.
                let color = Color::Rgb(r, g, b);

                let char = if agent.vm.chaos_mode { '!' } else { '@' };

                ctx.print(
                    agent.pos.x as f64,
                    height - agent.pos.y as f64,
                    Span::styled(char.to_string(), Style::default().fg(color)),
                );
            }

            // Draw Gaze Box
            // Eye coords are Top-Down (0 is top).
            let fx = app.eye.x as f64;
            let fy = height - app.eye.y as f64; // Flip Y for canvas
            let fw = app.eye.fovea_width as f64;
            let fh = app.eye.fovea_height as f64;

            let gaze_color = match app.mode {
                Mode::Stabilize => Color::Cyan,
                Mode::Excite => Color::Magenta,
            };

            ctx.draw(&Rectangle {
                x: fx - fw / 2.0,
                y: fy - fh / 2.0,
                width: fw,
                height: fh,
                color: gaze_color,
            });

            // Draw Reticle
            ctx.print(fx, fy, Span::styled("+", Style::default().fg(Color::White)));
        });

    f.render_widget(canvas, chunks[0]);

    let mode_str = match app.mode {
        Mode::Stabilize => "STABILIZE (Cyan - Calm)",
        Mode::Excite => "EXCITE (Magenta - Chaos)",
    };

    let info = Paragraph::new(format!(
        "Mode: {} | Agents: {} | Eye: ({:.1}, {:.1}) | 'm': Toggle Mode | Mouse: Move Gaze | 'q': Quit",
        mode_str,
        app.agents.len(),
        app.eye.x,
        app.eye.y
    ))
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(info, chunks[1]);
}
