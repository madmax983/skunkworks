mod laban;
mod rover;
mod world;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use laban::{Director, LabanEffort};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    text::Span,
    widgets::{
        canvas::{Canvas, Line as CanvasLine},
        Block, Borders, Sparkline,
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
    director: Director,
    running: bool,
    view_radius: f64,
    message: String,
    message_timer: usize,
    history_weight: Vec<u64>,
    history_time: Vec<u64>,
    history_space: Vec<u64>,
    history_flow: Vec<u64>,
}

impl App {
    fn new() -> Result<Self> {
        let mut world = World::new();
        // Start scanning current directory
        world.scan_path(std::env::current_dir()?.as_path())?;

        Ok(Self {
            rover: Rover::new(0.0, 0.0),
            world,
            director: Director::default(),
            running: true,
            view_radius: 60.0,
            message: String::from("Laban Rover Online. Environment affects physics."),
            message_timer: 100,
            history_weight: vec![0; 50],
            history_time: vec![0; 50],
            history_space: vec![0; 50],
            history_flow: vec![0; 50],
        })
    }

    fn update(&mut self, dt_secs: f32) {
        self.update_environment_perception();
        self.director.update(dt_secs);
        self.rover.update(&self.director.current_effort);

        // Update history for sparklines
        let e = self.director.current_effort;
        self.push_history(&e);

        if self.message_timer > 0 {
            self.message_timer -= 1;
        } else {
            self.message = String::new();
        }
    }

    fn push_history(&mut self, e: &LabanEffort) {
        if self.history_weight.len() >= 100 {
            self.history_weight.remove(0);
        }
        self.history_weight.push((e.weight * 10.0) as u64);

        if self.history_time.len() >= 100 {
            self.history_time.remove(0);
        }
        self.history_time.push((e.time * 10.0) as u64);

        if self.history_space.len() >= 100 {
            self.history_space.remove(0);
        }
        self.history_space.push((e.space * 10.0) as u64);

        if self.history_flow.len() >= 100 {
            self.history_flow.remove(0);
        }
        self.history_flow.push((e.flow * 10.0) as u64);
    }

    fn update_environment_perception(&mut self) {
        let mut total_size = 0u64;
        let mut total_age_sec = 0u64;
        let mut count = 0;
        let now = std::time::SystemTime::now();

        for e in &self.world.entities {
            // Check entities within "sensor range" (30 units)
            if e.pos.distance(self.rover.pos) < 30.0 {
                match e.kind {
                    EntityType::File { size } => total_size += size,
                    _ => total_size += 4096, // Directory weight assumption
                }
                let age = now.duration_since(e.modified).unwrap_or_default().as_secs();
                total_age_sec += age;
                count += 1;
            }
        }

        if count > 0 {
            let avg_size = total_size as f64 / count as f64;
            let avg_age = total_age_sec as f64 / count as f64;

            // Normalize
            // Size: 1KB = Light, 10MB = Heavy.
            // ln(1) = 0. ln(10000000) ~ 16.
            let size_factor = (avg_size.ln() / 16.0).clamp(0.0, 1.0) as f32;

            // Age: 0 = New, 1 Year = Old.
            let year_sec = 31536000.0;
            let age_factor = (avg_age as f64 / year_sec).clamp(0.0, 1.0) as f32;

            // Depth: Deeper = More Indirect/Complex Space?
            let depth = self.world.current_path.components().count();
            let depth_factor = (depth as f32 / 8.0).clamp(0.0, 1.0);

            self.director
                .set_target_from_environment(size_factor, age_factor, depth_factor);
        } else {
            // In the void, return to neutral
            self.director.set_target_from_environment(0.5, 0.5, 0.5);
        }
    }

    fn set_message(&mut self, msg: String) {
        self.message = msg;
        self.message_timer = 180;
    }

    fn scan_action(&mut self) {
        let target = self.get_nearest_entity();

        if let Some((name, kind, dist)) = target {
            if dist < 5.0 {
                match kind {
                    EntityType::File { size } => {
                        self.set_message(format!("File: {} | Size: {} bytes", name, size));
                    }
                    EntityType::Directory => {
                        self.set_message(format!("Directory: {} | ENTER to dive", name));
                    }
                }
            } else {
                self.set_message("Void. No targets.".to_string());
            }
        }
    }

    fn get_nearest_entity(&self) -> Option<(String, EntityType, f64)> {
        self.world
            .entities
            .iter()
            .map(|e| {
                (
                    e.name.clone(),
                    e.kind.clone(),
                    e.pos.distance(self.rover.pos),
                )
            })
            .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal))
    }

    fn enter_action(&mut self) -> Result<()> {
        let target = self
            .world
            .entities
            .iter()
            .map(|e| {
                (
                    e.path.clone(),
                    e.name.clone(),
                    e.kind.clone(),
                    e.pos.distance(self.rover.pos),
                )
            })
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
                        self.set_message("Cannot enter file.".to_string());
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_input(&mut self, event: Event) -> Result<()> {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                let e = &self.director.current_effort;
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => self.running = false,
                    KeyCode::Up | KeyCode::Char('w') => self.rover.thrust(0.1, e),
                    KeyCode::Down | KeyCode::Char('s') => self.rover.thrust(-0.05, e),
                    KeyCode::Left | KeyCode::Char('a') => self.rover.rotate(0.1, e),
                    KeyCode::Right | KeyCode::Char('d') => self.rover.rotate(-0.1, e),
                    KeyCode::Char('+') | KeyCode::Char('=') => {
                        self.view_radius = (self.view_radius - 5.0).max(10.0)
                    }
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
            let dt = last_tick.elapsed().as_secs_f32();
            app.update(dt);
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
            Constraint::Length(6), // HUD
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
        .x_bounds([view_x - r * 1.5, view_x + r * 1.5])
        .y_bounds([view_y - r, view_y + r])
        .marker(symbols::Marker::Braille)
        .paint(|ctx| {
            // Draw Entities
            for entity in &app.world.entities {
                let color = match entity.kind {
                    EntityType::Directory => Color::Yellow,
                    EntityType::File { .. } => Color::Blue,
                };

                // Culling
                if (entity.pos.x - view_x).abs() > r * 2.0
                    || (entity.pos.y - view_y).abs() > r * 2.0
                {
                    continue;
                }

                ctx.print(
                    entity.pos.x,
                    entity.pos.y,
                    Span::styled(
                        match entity.kind {
                            EntityType::Directory => "📂",
                            EntityType::File { .. } => "·",
                        },
                        Style::default().fg(color),
                    ),
                );

                // If close, draw label
                if entity.pos.distance(app.rover.pos) < 5.0 {
                    ctx.print(
                        entity.pos.x,
                        entity.pos.y + 1.0,
                        Span::raw(entity.name.clone()),
                    );
                }
            }

            // Draw Rover
            let angle = app.rover.angle;
            let pos = app.rover.pos;

            // Rover Color changes based on Flow (Free = Cyan, Bound = Magenta)
            let flow_color = if app.director.current_effort.flow > 0.5 {
                Color::Cyan
            } else {
                Color::Magenta
            };

            let tip = Vec2::new(angle.cos(), angle.sin()) * 2.0;
            let left = Vec2::new((angle + 2.5).cos(), (angle + 2.5).sin()) * 1.5;
            let right = Vec2::new((angle - 2.5).cos(), (angle - 2.5).sin()) * 1.5;

            let p1 = pos + tip;
            let p2 = pos + left;
            let p3 = pos + right;

            ctx.draw(&CanvasLine {
                x1: p1.x,
                y1: p1.y,
                x2: p2.x,
                y2: p2.y,
                color: flow_color,
            });
            ctx.draw(&CanvasLine {
                x1: p2.x,
                y1: p2.y,
                x2: p3.x,
                y2: p3.y,
                color: flow_color,
            });
            ctx.draw(&CanvasLine {
                x1: p3.x,
                y1: p3.y,
                x2: p1.x,
                y2: p1.y,
                color: flow_color,
            });
        });

    f.render_widget(canvas, area);
}

fn draw_hud(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(area);

    let e = app.director.current_effort;

    let draw_metric = |f: &mut Frame, title: &str, val: f32, hist: &[u64], rect: Rect| {
        let label = match title {
            "Weight" => {
                if val < 0.5 {
                    "STRONG"
                } else {
                    "LIGHT"
                }
            }
            "Time" => {
                if val < 0.5 {
                    "SUDDEN"
                } else {
                    "SUSTAINED"
                }
            }
            "Space" => {
                if val < 0.5 {
                    "DIRECT"
                } else {
                    "INDIRECT"
                }
            }
            "Flow" => {
                if val < 0.5 {
                    "BOUND"
                } else {
                    "FREE"
                }
            }
            _ => "",
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!("{} {:.2} {}", title, val, label));

        let sparkline = Sparkline::default()
            .block(block)
            .data(hist)
            .style(Style::default().fg(Color::Green));

        f.render_widget(sparkline, rect);
    };

    draw_metric(f, "Weight", e.weight, &app.history_weight, chunks[0]);
    draw_metric(f, "Time", e.time, &app.history_time, chunks[1]);
    draw_metric(f, "Space", e.space, &app.history_space, chunks[2]);
    draw_metric(f, "Flow", e.flow, &app.history_flow, chunks[3]);
}
