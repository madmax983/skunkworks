mod physics;
mod rain;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use physics::FluidSolver;
use rain::RainManager;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct App {
    fluid: FluidSolver,
    rain: RainManager,
    width: f64,
    height: f64,
}

impl App {
    fn new() -> Self {
        // Initial dimensions, will update on first render
        Self {
            fluid: FluidSolver::new(100.0, 100.0),
            rain: RainManager::new(),
            width: 100.0,
            height: 100.0,
        }
    }

    fn update(&mut self) {
        // Update dimensions if needed (fluid solver handles boundaries)
        self.fluid.width = self.width as f32;
        self.fluid.height = self.height as f32;

        // Update rain
        let splashes = self.rain.update(self.width, self.height);

        // Convert splashes to fluid particles
        for (x, y) in splashes {
            self.fluid.add_particle(x as f32, y as f32);
        }

        // Update fluid
        // SPH requires small time steps for stability.
        // We can sub-step.
        let dt = 0.5; // Tunable
        self.fluid.update(dt);
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    // Update app dimensions based on render area
    // Note: Canvas coordinates are arbitrary, but matching them to terminal cells (roughly)
    // or just a fixed coordinate system is easier.
    // term-fluids used 100x100 fixed? No, it used App::new() default, but maybe Canvas size?
    // Let's use the layout size.
    // But physics simulation stability depends on density, so resizing world changes physics behavior if particle count stays same.
    // Let's keep physics world fixed at 100x100 (or slightly larger 200x150) and map to screen.
    // Or just update physics boundaries to match screen chars?
    // Screen chars: 100-200 width, 30-60 height.
    // Physics 200x100 is reasonable.

    app.width = chunks[0].width as f64;
    app.height = chunks[0].height as f64 * 2.0; // *2 because characters are tall, or just use raw cells?
                                                // Let's use raw width/height for now, maybe *2 for block characters if we used half-blocks.
                                                // But Canvas usually maps X/Y to resolution.
                                                // Let's stick to 1:1 mapping for simplicity.
    app.width = 100.0;
    app.height = 100.0;
    // Actually, let's keep it fixed so resizing window doesn't reset physics boundaries wildly.

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(" Fluid Rain "))
        .x_bounds([0.0, app.width])
        .y_bounds([app.height, 0.0]) // Invert Y so 0 is top
        .paint(|ctx| {
            // Draw Fluid
            let fluid_points: Vec<(f64, f64)> = app
                .fluid
                .particles
                .iter()
                .map(|p| (p.x as f64, p.y as f64))
                .collect();

            ctx.draw(&Points {
                coords: &fluid_points,
                color: Color::Cyan,
            });

            // Draw Rain
            for drop in &app.rain.drops {
                for (i, &ch) in drop.stream.iter().enumerate() {
                    let char_y = drop.y - i as f64;
                    // Cull invisible
                    if char_y < 0.0 || char_y > app.height {
                        continue;
                    }

                    let color = if i == 0 {
                        Color::White
                    } else if i < 3 {
                        Color::Green
                    } else {
                        Color::DarkGray
                    };

                    ctx.print(
                        drop.x,
                        char_y,
                        ratatui::text::Span::styled(ch.to_string(), Style::default().fg(color)),
                    );
                }
            }
        });

    f.render_widget(canvas, chunks[0]);

    let stats = Paragraph::new(format!(
        "Particles: {} | Rain Drops: {} | [Q] Quit",
        app.fluid.particles.len(),
        app.rain.drops.len()
    ))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(stats, chunks[1]);
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();
    let tick_rate = Duration::from_millis(33);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }
    }

    Ok(())
}
