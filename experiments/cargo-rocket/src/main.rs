use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Circle, Context, Line as CanvasLine},
        Block, Borders, Gauge, Paragraph,
    },
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

pub mod physics;
pub mod world;

use physics::{System, Vec2};

struct App {
    system: System,
    running: bool,
    zoom: f64,
    camera_pos: Vec2,
    tick_count: u64,
}

impl App {
    fn new(system: System) -> Self {
        Self {
            system,
            running: true,
            zoom: 0.5,
            camera_pos: Vec2::zero(),
            tick_count: 0,
        }
    }

    fn update(&mut self, dt: f64) {
        // Handle input state (thrust/rotate) is done in main loop via key events modifying ship state?
        // Actually, cleaner to handle "is key down" for continuous thrust.
        // But crossterm only gives KeyPress.
        // For continuous thrust, we toggle a flag on Press/Release?
        // Crossterm doesn't reliably give KeyRelease on all terms.
        // So we might use "Press Space to Toggle Thrust" or "Press to Thrust for X frames".
        // Let's use "Pulse Thrust": Pressing Space adds velocity instantly? No, physics.
        // Let's use: Pressing Space sets `thrusting = true`. It auto-resets to false after update?
        // Or we use `poll` very fast and if no key, we stop.
        // Simplest: Press Space = Toggle Engines.

        self.system.update(dt);

        // Camera follows ship
        let target = self.system.ship.pos;
        // Smooth follow
        self.camera_pos = self.camera_pos + (target - self.camera_pos) * 5.0 * dt;

        // Collision Check - Collect Cargo
        for body in &mut self.system.bodies {
            if body.is_fixed {
                continue;
            }
            // Simple circle collision
            let dist = (body.pos - self.system.ship.pos).magnitude();
            if dist < body.radius + 2.0 {
                // Mark as "Visited"
                // If it wasn't green, give fuel bonus?
                if body.color != Color::Green {
                    body.color = Color::Green; // Visited
                    self.system.ship.fuel =
                        (self.system.ship.fuel + 100.0).min(self.system.ship.max_fuel);
                }
            }
        }

        self.tick_count += 1;
    }
}

fn main() -> Result<()> {
    let system = world::load_system()?;
    let mut app = App::new(system);
    let mut tui = Tui::init()?;

    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    while app.running {
        tui.terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.running = false,
                        KeyCode::Left | KeyCode::Char('a') => app.system.ship.angle += 0.2,
                        KeyCode::Right | KeyCode::Char('d') => app.system.ship.angle -= 0.2,
                        KeyCode::Char(' ') => {
                            app.system.ship.thrusting = !app.system.ship.thrusting
                        }
                        KeyCode::Char('+') | KeyCode::Char('=') => app.zoom *= 1.1,
                        KeyCode::Char('-') | KeyCode::Char('_') => app.zoom /= 1.1,
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            let dt = 0.05; // Fixed physics step for stability
            app.update(dt);
            last_tick = Instant::now();
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(7)])
        .split(f.area());

    // Main Canvas
    let (cx, cy) = (app.camera_pos.x, app.camera_pos.y);
    let view_w = 200.0 / app.zoom;
    let view_h = view_w * (chunks[0].height as f64 / chunks[0].width as f64) * 2.0;

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Cargo Rocket 🚀 "),
        )
        .x_bounds([cx - view_w / 2.0, cx + view_w / 2.0])
        .y_bounds([cy - view_h / 2.0, cy + view_h / 2.0])
        .paint(|ctx| {
            draw_system(ctx, app);
        });

    f.render_widget(canvas, chunks[0]);

    // HUD Calculation
    let ship = &app.system.ship;
    let vel_mag = ship.vel.magnitude();

    let visited_count = app
        .system
        .bodies
        .iter()
        .filter(|b| b.color == Color::Green)
        .count();
    let total_count = app.system.bodies.iter().filter(|b| !b.is_fixed).count();
    let progress_ratio = if total_count > 0 {
        visited_count as f64 / total_count as f64
    } else {
        0.0
    };

    let fuel_ratio = (ship.fuel / ship.max_fuel).clamp(0.0, 1.0);

    // HUD Layout
    let hud_chunks = Layout::horizontal([
        Constraint::Percentage(30),
        Constraint::Percentage(40),
        Constraint::Percentage(30),
    ])
    .split(chunks[1]);

    // 1. Fuel Gauge
    let fuel_color = if fuel_ratio > 0.5 {
        Color::Green
    } else if fuel_ratio > 0.2 {
        Color::Yellow
    } else {
        Color::Red
    };

    let fuel_gauge = Gauge::default()
        .block(Block::bordered().title(" Fuel "))
        .gauge_style(Style::default().fg(fuel_color))
        .ratio(fuel_ratio)
        .label(format!("{:.1}/{:.1}", ship.fuel, ship.max_fuel));

    f.render_widget(fuel_gauge, hud_chunks[0]);

    // 2. Telemetry & Controls
    let telemetry_text = vec![
        Line::from(vec![
            Span::raw("VEL: "),
            Span::styled(
                format!("{:.2}", vel_mag),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(" | POS: "),
            Span::styled(
                format!("{:.0},{:.0}", ship.pos.x, ship.pos.y),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(if ship.thrusting {
            Span::styled(
                "🔥 ENGINE ON",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )
        } else {
            Span::raw("   ENGINE OFF")
        }),
        Line::from(""),
        Line::from(Span::styled(
            "Controls: ←/→ Rotate | Space Thrust | +/- Zoom | Q Quit",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let telemetry = Paragraph::new(telemetry_text)
        .block(Block::bordered().title(" Navigation "))
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(telemetry, hud_chunks[1]);

    // 3. Mission Progress
    let progress_gauge = Gauge::default()
        .block(Block::bordered().title(" Mission Progress "))
        .gauge_style(Style::default().fg(Color::Blue))
        .ratio(progress_ratio)
        .label(format!("{} / {}", visited_count, total_count));

    f.render_widget(progress_gauge, hud_chunks[2]);
}

fn draw_system(ctx: &mut Context, app: &App) {
    // Draw Bodies
    for body in &app.system.bodies {
        // Culling? nah, canvas handles it.
        ctx.draw(&Circle {
            x: body.pos.x,
            y: body.pos.y,
            radius: body.radius,
            color: body.color,
        });

        if body.radius > 2.0 || app.zoom > 1.0 {
            ctx.print(
                body.pos.x + body.radius,
                body.pos.y,
                Span::raw(body.name.clone()),
            );
        }
    }

    // Draw Ship
    let ship = &app.system.ship;
    let tip = ship.pos + Vec2::new(ship.angle.cos(), ship.angle.sin()) * 5.0;
    let left = ship.pos + Vec2::new((ship.angle + 2.5).cos(), (ship.angle + 2.5).sin()) * 3.0;
    let right = ship.pos + Vec2::new((ship.angle - 2.5).cos(), (ship.angle - 2.5).sin()) * 3.0;

    ctx.draw(&CanvasLine {
        x1: tip.x,
        y1: tip.y,
        x2: left.x,
        y2: left.y,
        color: Color::White,
    });
    ctx.draw(&CanvasLine {
        x1: left.x,
        y1: left.y,
        x2: right.x,
        y2: right.y,
        color: Color::White,
    });
    ctx.draw(&CanvasLine {
        x1: right.x,
        y1: right.y,
        x2: tip.x,
        y2: tip.y,
        color: Color::White,
    });

    // Exhaust
    if ship.thrusting {
        let back = ship.pos - Vec2::new(ship.angle.cos(), ship.angle.sin()) * 4.0;
        ctx.draw(&CanvasLine {
            x1: (left.x + right.x) / 2.0,
            y1: (left.y + right.y) / 2.0,
            x2: back.x,
            y2: back.y,
            color: Color::Red,
        });
    }
}
