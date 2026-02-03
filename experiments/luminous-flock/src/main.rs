use std::time::{Duration, Instant};

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

use tui_shared::Tui;

mod boid;
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

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
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

    let sync_index = app.world.synchronization_index();

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Luminous Flock (Boids + Fireflies)"),
        )
        .x_bounds([0.0, app.world.width])
        .y_bounds([0.0, app.world.height])
        .paint(|ctx| {
            for boid in &app.world.boids {
                // Determine visual style based on flash state
                let (char_str, color) = if boid.flash_timer > 0 {
                    ("★".to_string(), Color::White) // Bright flash
                } else {
                    // Dimmed based on phase (pulsing effect) or just base color
                    let base_char = boid.dna.char_representation.to_string();
                    let color = boid.dna.color;
                    (base_char, color)
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
        "Sync Index: {:.3} | Population: {} | 'r': Reset | 'q': Quit",
        sync_index,
        app.world.boids.len()
    );
    let p = Paragraph::new(status).style(Style::default().fg(Color::Black).bg(Color::Blue));
    f.render_widget(p, chunks[1]);
}
