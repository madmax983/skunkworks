mod world;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{
        Block, Borders, Paragraph,
        canvas::{Canvas, Points},
    },
};
use std::time::{Duration, Instant};
use tui_shared::Tui;
use world::World;

fn main() -> Result<()> {
    if std::env::args().any(|arg| arg == "--headless") {
        println!("Headless mode detected, bypassing TUI initialization.");
        return Ok(());
    }

    let mut tui = Tui::init()?;
    let res = run_app(&mut tui.terminal);
    drop(tui);

    if let Err(err) = res {
        println!("{:?}", err);
    }
    Ok(())
}

struct App {
    world: World,
    running: bool,
}

impl App {
    fn new(width: f64, height: f64) -> Self {
        Self {
            world: World::new(width, height),
            running: true,
        }
    }

    fn on_tick(&mut self) {
        self.world.update();
    }
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    let world_width = 160.0;
    let world_height = 100.0;
    let mut app = App::new(world_width, world_height);

    let tick_rate = Duration::from_millis(33); // ~30 FPS
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(f.area());

            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Miller-Physics: Codebase Physical Lattice"),
                )
                .x_bounds([-world_width / 2.0, world_width / 2.0])
                .y_bounds([-world_height / 2.0, world_height / 2.0])
                .paint(|ctx| {
                    // Draw Bonds
                    for bond in &app.world.bonds {
                        let p1 = app.world.system.particles[bond.0].pos;
                        let p2 = app.world.system.particles[bond.1].pos;
                        ctx.draw(&ratatui::widgets::canvas::Line {
                            x1: p1.x as f64,
                            y1: p1.y as f64,
                            x2: p2.x as f64,
                            y2: p2.y as f64,
                            color: Color::DarkGray,
                        });
                    }

                    // Draw Particles
                    for (i, p) in app.world.system.particles.iter().enumerate() {
                        let is_dir = app.world.is_dir[i];
                        let color = if is_dir { Color::Blue } else { Color::Green };
                        let char_repr = if is_dir { "■" } else { "•" };

                        ctx.print(
                            p.pos.x as f64,
                            p.pos.y as f64,
                            Span::styled(char_repr, Style::default().fg(color)),
                        );
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let status = format!(
                "Nodes: {} | Bonds: {} | 'r': Reset | 'q': Quit",
                app.world.system.particles.len(),
                app.world.bonds.len()
            );
            let p = Paragraph::new(status).style(Style::default().fg(Color::Black).bg(Color::Cyan));
            f.render_widget(p, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char('q') => app.running = false,
                KeyCode::Char('r') => {
                    app.world = World::new(world_width, world_height);
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if !app.running {
            return Ok(());
        }
    }
}
