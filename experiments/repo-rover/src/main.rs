mod rover;
mod world;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    text::Span,
    widgets::{
        canvas::{Canvas, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
    Frame,
};
use rover::Rover;
use std::time::{Duration, Instant};
use tui_shared::{math::Vec2, Tui};
use world::{EntityType, World};

struct App {
    rover: Rover,
    world: World,
    running: bool,
    view_radius: f64,
    message: String,
    message_timer: usize,
}

impl App {
    fn new() -> Result<Self> {
        let mut world = World::new();
        // Start scanning current directory
        world.scan_path(std::env::current_dir()?.as_path())?;

        Ok(Self {
            rover: Rover::new(0.0, 0.0),
            world,
            running: true,
            view_radius: 60.0,
            message: String::from("Welcome to Repo Rover! Use Arrow Keys to move."),
            message_timer: 100,
        })
    }

    fn update(&mut self) {
        self.rover.update();
        if self.message_timer > 0 {
            self.message_timer -= 1;
        } else {
            self.message = String::new();
        }
    }

    fn set_message(&mut self, msg: String) {
        self.message = msg;
        self.message_timer = 180;
    }

    fn scan_action(&mut self) {
        let target = self.world.entities.iter()
            .map(|e| (e.name.clone(), e.kind.clone(), e.pos.distance(self.rover.pos)))
            .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));

        if let Some((name, kind, dist)) = target {
            if dist < 5.0 {
                match kind {
                    EntityType::File { size } => {
                        self.set_message(format!("File: {} | Size: {} bytes", name, size));
                    },
                    EntityType::Directory => {
                        self.set_message(format!("Directory: {} | Use ENTER to enter", name));
                    }
                }
            } else {
                self.set_message("No target in range.".to_string());
            }
        }
    }

    fn enter_action(&mut self) -> Result<()> {
        let target = self.world.entities.iter()
            .map(|e| (e.path.clone(), e.name.clone(), e.kind.clone(), e.pos.distance(self.rover.pos)))
            .min_by(|a, b| a.3.partial_cmp(&b.3).unwrap_or(std::cmp::Ordering::Equal));

        if let Some((path, name, kind, dist)) = target {
            if dist < 5.0 {
                match kind {
                    EntityType::Directory => {
                        self.world.scan_path(&path)?;
                        self.rover.pos = Vec2::zero();
                        self.rover.vel = Vec2::zero();
                        self.set_message(format!("Entered: {}", name));
                    }
                    EntityType::File { .. } => {
                        self.set_message("Cannot enter a file.".to_string());
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_input(&mut self, event: Event) -> Result<()> {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => self.running = false,
                    KeyCode::Up | KeyCode::Char('w') => self.rover.thrust(0.1),
                    KeyCode::Down | KeyCode::Char('s') => self.rover.thrust(-0.05),
                    KeyCode::Left | KeyCode::Char('a') => self.rover.rotate(0.1),
                    KeyCode::Right | KeyCode::Char('d') => self.rover.rotate(-0.1),
                    KeyCode::Char('+') | KeyCode::Char('=') => self.view_radius = (self.view_radius - 5.0).max(10.0),
                    KeyCode::Char('-') => self.view_radius = (self.view_radius + 5.0).min(200.0),
                    KeyCode::Enter => self.enter_action()?,
                    KeyCode::Char(' ') => self.scan_action(),
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new()?;

    // Game loop
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    while app.running {
        tui.terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            app.handle_input(event::read()?)?;
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(1), // HUD
        ])
        .split(f.area());

    draw_canvas(f, app, chunks[0]);
    draw_hud(f, app, chunks[1]);
}

fn draw_canvas(f: &mut Frame, app: &App, area: Rect) {
    let view_x = app.rover.pos.x;
    let view_y = app.rover.pos.y;
    let r = app.view_radius;

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Sector Map"))
        .x_bounds([view_x - r * 1.5, view_x + r * 1.5]) // Aspect ratio correction attempt
        .y_bounds([view_y - r, view_y + r])
        .marker(symbols::Marker::Braille)
        .paint(|ctx| {
            // Draw Entities
            for entity in &app.world.entities {
                let color = match entity.kind {
                    EntityType::Directory => Color::Yellow,
                    EntityType::File { .. } => Color::Blue,
                };

                // Culling for performance (simple box check)
                if (entity.pos.x - view_x).abs() > r * 2.0 || (entity.pos.y - view_y).abs() > r * 2.0 {
                    continue;
                }

                ctx.print(entity.pos.x, entity.pos.y, Span::styled(
                    match entity.kind {
                        EntityType::Directory => "📂",
                        EntityType::File { .. } => "·",
                    },
                    Style::default().fg(color)
                ));

                // If close, draw label
                if entity.pos.distance(app.rover.pos) < 5.0 {
                     ctx.print(entity.pos.x, entity.pos.y + 1.0, Span::raw(entity.name.clone()));
                }
            }

            // Draw Rover manually using lines
            let angle = app.rover.angle;
            let pos = app.rover.pos;

            // Tip
            let tip = Vec2::new(angle.cos(), angle.sin()) * 2.0;
            // Left Wing
            let left = Vec2::new((angle + 2.5).cos(), (angle + 2.5).sin()) * 1.5;
            // Right Wing
            let right = Vec2::new((angle - 2.5).cos(), (angle - 2.5).sin()) * 1.5;

            let p1 = pos + tip;
            let p2 = pos + left;
            let p3 = pos + right;

            ctx.draw(&CanvasLine {
                x1: p1.x, y1: p1.y,
                x2: p2.x, y2: p2.y,
                color: Color::Red,
            });
            ctx.draw(&CanvasLine {
                x1: p2.x, y1: p2.y,
                x2: p3.x, y2: p3.y,
                color: Color::Red,
            });
            ctx.draw(&CanvasLine {
                x1: p3.x, y1: p3.y,
                x2: p1.x, y2: p1.y,
                color: Color::Red,
            });
        });

    f.render_widget(canvas, area);
}

fn draw_hud(f: &mut Frame, app: &App, area: Rect) {
    let hud_text = format!(
        "POS: {:.1},{:.1} | VEL: {:.1} | DIR: {:?} | {}",
        app.rover.pos.x,
        app.rover.pos.y,
        app.rover.vel.magnitude(),
        app.world.current_path.file_name().unwrap_or_default().to_string_lossy(),
        app.message
    );
    f.render_widget(Paragraph::new(hud_text).style(Style::default().fg(Color::Green)), area);
}
