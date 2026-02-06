use std::io;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{
        canvas::{Canvas, Line},
        Block, Borders, Paragraph,
    },
    Terminal,
};

use tui_shared::Tui;

mod boid;
mod qubit;
mod world;
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()>
where
    std::io::Error: From<<B as Backend>::Error>,
{
    // Canvas dimensions (virtual units)
    let world_width = 200.0;
    let world_height = 100.0;

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
                            // Reset/Reseed
                            app.world = World::new(world_width, world_height);
                        }
                        KeyCode::Char('m') => {
                            // Measure
                            app.world.measure_all();
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

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Quantum Boids (Entanglement + Flocking)"),
        )
        .x_bounds([0.0, app.world.width])
        .y_bounds([0.0, app.world.height])
        .paint(|ctx| {
            // Draw entanglement lines first (background)
            for boid in &app.world.boids {
                if let Some(partner_idx) = boid.entangled_partner {
                    if partner_idx < app.world.boids.len() {
                        let partner = &app.world.boids[partner_idx];
                        ctx.draw(&Line {
                            x1: boid.position.0,
                            y1: boid.position.1,
                            x2: partner.position.0,
                            y2: partner.position.1,
                            color: Color::DarkGray,
                        });
                    }
                }
            }

            // Draw boids
            for boid in &app.world.boids {
                // Color based on qubit state
                // |0> (Prob 0) = Blue
                // |1> (Prob 1) = Red
                // Superposition = Purple
                let p = boid.qubit.prob_one();
                let color = if p < 0.2 {
                    Color::Blue
                } else if p > 0.8 {
                    Color::Red
                } else {
                    Color::Magenta
                };

                let char_str = if boid.entangled_partner.is_some() {
                    "∞"
                } else {
                    "•"
                };

                ctx.print(
                    boid.position.0,
                    boid.position.1,
                    Span::styled(char_str, Style::default().fg(color)),
                );
            }
        });

    f.render_widget(canvas, chunks[0]);

    let status = format!(
        "Population: {} | 'r': Reset | 'm': Measure (Collapse) | 'q': Quit",
        app.world.boids.len()
    );
    let p = Paragraph::new(status).style(Style::default().fg(Color::Black).bg(Color::White));
    f.render_widget(p, chunks[1]);
}
