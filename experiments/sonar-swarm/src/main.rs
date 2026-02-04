use anyhow::Result;
use crossbeam_channel::bounded;
use crossterm::event::{self, Event, KeyCode, MouseButton, MouseEventKind};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{
        canvas::{Canvas, Rectangle},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

use tui_shared::Tui;

mod audio;
mod boid;
mod physics;
mod world;

use audio::{run_audio, AudioCommand};
use world::World;

fn main() -> Result<()> {
    // Setup Audio
    let width = 60;
    let height = 30;
    let (cmd_tx, cmd_rx) = bounded(100);
    let (snap_tx, snap_rx) = bounded(1);

    let _stream = run_audio(width, height, cmd_rx, snap_tx)?;

    // Setup TUI
    let mut tui = Tui::init()?;
    let res = run_app(&mut tui.terminal, width, height, cmd_tx, snap_rx);
    drop(tui);

    if let Err(err) = res {
        eprintln!("{:?}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    width: usize,
    height: usize,
    cmd_tx: crossbeam_channel::Sender<AudioCommand>,
    snap_rx: crossbeam_channel::Receiver<Vec<f32>>,
) -> Result<()> {
    let mut world = World::new(width, height, cmd_tx.clone());

    // Add box walls
    for x in 0..width {
        world.walls[0 * width + x] = true;
        let _ = cmd_tx.send(AudioCommand::AddWall(x, 0));
        world.walls[(height - 1) * width + x] = true;
        let _ = cmd_tx.send(AudioCommand::AddWall(x, height - 1));
    }
    for y in 0..height {
        world.walls[y * width + 0] = true;
        let _ = cmd_tx.send(AudioCommand::AddWall(0, y));
        world.walls[y * width + width - 1] = true;
        let _ = cmd_tx.send(AudioCommand::AddWall(width - 1, y));
    }

    let tick_rate = Duration::from_millis(33);
    let mut last_tick = Instant::now();
    let mut running = true;

    while running {
        if let Ok(snap) = snap_rx.try_recv() {
            world.pressure_map = snap;
        }

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(f.area());

            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Sonar Swarm (Echo Chamber x Luminous Flock)"),
                )
                .x_bounds([0.0, width as f64])
                .y_bounds([0.0, height as f64])
                .paint(|ctx| {
                    for y in 0..height {
                        for x in 0..width {
                            let idx = y * width + x;
                            if world.walls[idx] {
                                ctx.draw(&Rectangle {
                                    x: x as f64,
                                    y: y as f64,
                                    width: 1.0,
                                    height: 1.0,
                                    color: Color::Gray,
                                });
                            } else {
                                let p = world.pressure_map[idx];
                                let intensity = p.abs();
                                if intensity > 0.1 {
                                    let color = if p > 0.0 { Color::Cyan } else { Color::Blue };
                                    ctx.layer();
                                    ctx.print(
                                        x as f64,
                                        y as f64,
                                        Span::styled(".", Style::default().fg(color)),
                                    );
                                }
                            }
                        }
                    }

                    for boid in &world.boids {
                        let (char_str, color) = if boid.ping_timer > 0 {
                            ("O".to_string(), Color::White)
                        } else {
                            (boid.dna.char_representation.to_string(), boid.dna.color)
                        };
                        ctx.print(
                            boid.position.0,
                            boid.position.1,
                            Span::styled(char_str, Style::default().fg(color)),
                        );
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let info = Paragraph::new("Q: Quit | R: Reset Walls")
                .style(Style::default().fg(Color::White).bg(Color::Black));
            f.render_widget(info, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => running = false,
                    KeyCode::Char('r') => {
                        world.walls.fill(false);
                        let _ = cmd_tx.send(AudioCommand::ClearWalls);
                    }
                    _ => {}
                },
                Event::Mouse(mouse) => {
                    match mouse.kind {
                        MouseEventKind::Down(MouseButton::Left) => {
                            let x = mouse.column as usize;
                            let y = mouse.row as usize;
                            if x > 0 && x <= width && y > 0 && y <= height {
                                let wx = x - 1;
                                let wy = height - (y - 1); // Flip Y
                                if wx < width && wy < height {
                                    let idx = wy * width + wx;
                                    world.walls[idx] = !world.walls[idx];
                                    if world.walls[idx] {
                                        let _ = cmd_tx.send(AudioCommand::AddWall(wx, wy));
                                    } else {
                                        let _ = cmd_tx.send(AudioCommand::RemoveWall(wx, wy));
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            world.update();
            last_tick = Instant::now();
        }
    }

    Ok(())
}
