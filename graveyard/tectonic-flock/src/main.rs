use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{
        canvas::{Canvas, Context, Line},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

mod boid;
mod fissure;
mod git;
mod strata;
mod world;

use world::World;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let res = run_app(&mut tui.terminal);
    drop(tui); // Restore terminal
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
    let world_width = 200.0;
    let world_height = 100.0;

    let mut app = App::new(world_width, world_height);
    let tick_rate = Duration::from_millis(33);
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
                        KeyCode::Char('r') => app.world = World::new(world_width, world_height),
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
                .title("Tectonic Flock"),
        )
        .x_bounds([0.0, app.world.width])
        .y_bounds([0.0, app.world.height]) // Invert Y? Canvas is usually bottom-left 0,0.
        // If Strata 0 is top, we might need to invert or just accept it grows up.
        // Let's assume standard Canvas: 0,0 is bottom-left.
        // Git history: usually new on top. But strata accumulate.
        // Let's render assuming 0 is bottom.
        .paint(|ctx| {
            draw_world(ctx, &app.world);
        });

    f.render_widget(canvas, chunks[0]);

    let info = format!(
        "Boids: {} | Strata: {} | Fissures: {} | Scroll Y: {:.1} | 'r': Reset | 'q': Quit",
        app.world.boids.len(),
        app.world.strata.len(),
        app.world.fissures.len(),
        app.world.scroll_y
    );
    f.render_widget(
        Paragraph::new(info).style(Style::default().bg(Color::Blue)),
        chunks[1],
    );
}

fn draw_world(ctx: &mut Context, world: &World) {
    // 1. Draw Strata
    for strata in &world.strata {
        let screen_y = strata.y_pos - world.scroll_y;
        if screen_y >= 0.0 && screen_y <= world.height {
            let color = match strata.color_idx % 6 {
                0 => Color::DarkGray,
                1 => Color::Gray,
                2 => Color::White,
                3 => Color::Yellow, // Stressed?
                4 => Color::Red,
                _ => Color::Blue,
            };

            // Draw horizontal line with offset shift
            // Strata is "shifted". Let's draw it as a jagged line or just a shifted segment.
            // Let's draw a full width line but offset points.
            // Actually, Strata is usually a layer. Let's just draw a line.

            // To visualize the "Shift", maybe we break the line at the fissure or just shift the whole line?
            // "offset_x" suggests the whole layer is shifted.
            // Let's draw the line from 0 to width, but shifted by offset_x (wrapping?)
            // Or just draw it.

            ctx.draw(&Line {
                x1: 0.0 + strata.offset_x,
                y1: screen_y,
                x2: world.width + strata.offset_x,
                y2: screen_y,
                color,
            });

            // Label with hash?
            if screen_y > 1.0 && screen_y < world.height - 1.0 {
                ctx.print(
                    1.0,
                    screen_y,
                    Span::styled(
                        format!("{}", &strata.commit.hash[..7]),
                        Style::default().fg(color),
                    ),
                );
            }
        }
    }

    // 2. Draw Fissures
    for fissure in &world.fissures {
        // Draw lines between points
        if fissure.points.len() < 2 {
            continue;
        }

        for i in 0..fissure.points.len() - 1 {
            let p1 = fissure.points[i];
            let p2 = fissure.points[i + 1];

            let s_y1 = p1.y - world.scroll_y;
            let s_y2 = p2.y - world.scroll_y;

            // Simple clip check
            if (s_y1 >= 0.0 && s_y1 <= world.height) || (s_y2 >= 0.0 && s_y2 <= world.height) {
                ctx.draw(&Line {
                    x1: p1.x,
                    y1: s_y1,
                    x2: p2.x,
                    y2: s_y2,
                    color: Color::Red,
                });
            }
        }
    }

    // 3. Draw Boids
    for boid in &world.boids {
        let (char_str, color) = if boid.flash_timer > 0 {
            ("★".to_string(), Color::Yellow)
        } else {
            (boid.dna.char_representation.to_string(), boid.dna.color)
        };

        ctx.print(
            boid.position.x,
            boid.position.y,
            Span::styled(char_str, Style::default().fg(color)),
        );
    }
}
