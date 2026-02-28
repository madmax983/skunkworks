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
    widgets::{
        Block, Borders, Paragraph,
        canvas::{Canvas, Points},
    },
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

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    let world_width = 160.0;
    let world_height = 100.0;
    let mut app = App::new(world_width, world_height);

    let tick_rate = Duration::from_millis(33); // ~30 FPS
    let mut last_tick = Instant::now();

    let mut trails_low = Vec::with_capacity(2048);
    let mut trails_med = Vec::with_capacity(2048);
    let mut trails_high = Vec::with_capacity(2048);

    loop {
        // Pre-process render data
        trails_low.clear();
        trails_med.clear();
        trails_high.clear();

        for y in 0..(app.world.height as usize) {
            for x in 0..(app.world.width as usize) {
                let val = app.world.get_trail(x, y);
                if val > 50.0 {
                    trails_high.push((x as f64, y as f64));
                } else if val > 20.0 {
                    trails_med.push((x as f64, y as f64));
                } else if val > 5.0 {
                    trails_low.push((x as f64, y as f64));
                }
            }
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
                        .title("Myco-Flock: Pheromone-Guided Flocking"),
                )
                .x_bounds([0.0, world_width])
                .y_bounds([0.0, world_height])
                .paint(|ctx| {
                    // Draw Trails
                    ctx.draw(&Points {
                        coords: &trails_low,
                        color: Color::DarkGray,
                    });
                    ctx.draw(&Points {
                        coords: &trails_med,
                        color: Color::Gray,
                    });
                    ctx.draw(&Points {
                        coords: &trails_high,
                        color: Color::White,
                    });

                    // Draw Boids
                    for boid in &app.world.boids {
                        let base_char = boid.dna.char_representation.to_string();
                        let color = boid.dna.color;
                        ctx.print(
                            boid.position.x,
                            boid.position.y,
                            Span::styled(base_char, Style::default().fg(color)),
                        );
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let status = format!(
                "Population: {} | 'r': Reset | 'q': Quit",
                app.world.boids.len()
            );
            let p = Paragraph::new(status).style(Style::default().fg(Color::Black).bg(Color::Blue));
            f.render_widget(p, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => app.running = false,
                        KeyCode::Char('r') => {
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
