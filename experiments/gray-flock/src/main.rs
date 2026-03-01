mod boid;
mod world;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, canvas::Canvas},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;
use world::World;

fn main() -> Result<()> {
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

#[allow(clippy::collapsible_if)]
fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    // Canvas dimensions
    let world_width = 160.0;
    let world_height = 80.0;

    let mut app = App::new(world_width, world_height);

    let tick_rate = Duration::from_millis(33); // ~30 FPS
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => app.running = false,
                        KeyCode::Char('r') => {
                            // Reset
                            app.world = World::new(world_width, world_height);
                        }
                        _ => {}
                    }
                }
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

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    let gs_v = app.world.gray_scott.v();
    let w = app.world.gray_scott.width();

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Gray-Flock (Reaction-Diffusion Swarming)"),
        )
        .x_bounds([0.0, app.world.width])
        .y_bounds([0.0, app.world.height])
        .paint(|ctx| {
            // Render chemical background
            // V ranges from 0.0 to ~1.0. Draw dots for different intensities
            for i in 0..gs_v.len() {
                let v = gs_v[i];
                if v > 0.05 {
                    let x = (i % w) as f64;
                    let y = (i / w) as f64;

                    let color = if v > 0.4 {
                        Color::Cyan
                    } else if v > 0.2 {
                        Color::DarkGray
                    } else {
                        Color::Black
                    };

                    if color != Color::Black {
                        ctx.print(x, y, Span::styled(".", Style::default().fg(color)));
                    }
                }
            }

            // Render boids
            for boid in &app.world.boids {
                ctx.print(
                    boid.position.x,
                    boid.position.y,
                    Span::styled(">", Style::default().fg(Color::Yellow)),
                );
            }
        });

    f.render_widget(canvas, chunks[0]);

    let status = format!(
        "Population: {} | 'r': Reset | 'q': Quit",
        app.world.boids.len()
    );
    let p = Paragraph::new(status).style(Style::default().fg(Color::Black).bg(Color::Cyan));
    f.render_widget(p, chunks[1]);
}
