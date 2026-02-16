use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Circle, Line},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::{
    time::{Duration, Instant},
};
use tui_shared::Tui;
use locus::Vec2;

mod arm;
mod flock;

use arm::Arm;
use flock::World;

struct App {
    world: World,
    arm: Arm,
    running: bool,
    target_mode: TargetMode,
    manual_target: Vec2,
}

#[derive(PartialEq)]
enum TargetMode {
    Auto,   // Chases center of flock
    Manual, // Controlled by user (WASD)
}

impl App {
    fn new(width: f64, height: f64) -> Self {
        // Arm base at bottom center
        let base = Vec2::new(width / 2.0, 0.0);
        // 4 segments of length 15
        let lengths = vec![15.0, 15.0, 15.0, 15.0, 10.0];
        let arm = Arm::new(base, lengths);

        Self {
            world: World::new(width, height),
            arm,
            running: true,
            target_mode: TargetMode::Auto,
            manual_target: Vec2::new(width / 2.0, height / 2.0),
        }
    }

    fn on_tick(&mut self) {
        // 1. Determine Target
        let target = match self.target_mode {
            TargetMode::Auto => self.world.center_of_mass(),
            TargetMode::Manual => self.manual_target,
        };

        // 2. Update Arm
        self.arm.solve(target);

        // 3. Update Flock (flee from arm tip)
        let predator_pos = self.arm.end_effector();
        // Pass Some(pos) to enable fleeing
        self.world.update(Some(predator_pos));
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    // Canvas dimensions
    let width = 200.0;
    let height = 100.0;

    let mut app = App::new(width, height);

    let tick_rate = Duration::from_millis(30);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => app.running = false,
                    KeyCode::Char('m') => {
                        app.target_mode = match app.target_mode {
                            TargetMode::Auto => {
                                app.manual_target = app.arm.end_effector();
                                TargetMode::Manual
                            }
                            TargetMode::Manual => TargetMode::Auto,
                        };
                    }
                    // Manual control
                    KeyCode::Char('w') | KeyCode::Up => app.manual_target.y += 2.0,
                    KeyCode::Char('s') | KeyCode::Down => app.manual_target.y -= 2.0,
                    KeyCode::Char('a') | KeyCode::Left => app.manual_target.x -= 2.0,
                    KeyCode::Char('d') | KeyCode::Right => app.manual_target.x += 2.0,
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if !app.running {
            break;
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Luminous Limb"))
        .x_bounds([0.0, app.world.width])
        .y_bounds([0.0, app.world.height])
        .paint(|ctx| {
            // Draw Boids
            for boid in &app.world.boids {
                let (char_string, color) = if boid.flash_timer > 0 {
                    ("★".to_string(), Color::White)
                } else {
                    (boid.dna.char_representation.to_string(), boid.dna.color)
                };

                // Ratatui Canvas expects &str for print
                // We construct a Span or just string
                ctx.print(
                    boid.physics.position.x,
                    boid.physics.position.y,
                    ratatui::text::Span::styled(char_string, Style::default().fg(color)),
                );
            }

            // Draw Arm
            for i in 0..app.arm.joints.len() - 1 {
                let p1 = app.arm.joints[i];
                let p2 = app.arm.joints[i + 1];
                ctx.draw(&Line {
                    x1: p1.x,
                    y1: p1.y,
                    x2: p2.x,
                    y2: p2.y,
                    color: Color::Green,
                });

                // Joint
                ctx.draw(&Circle {
                    x: p1.x,
                    y: p1.y,
                    radius: 1.0,
                    color: Color::Yellow,
                });
            }

            // End Effector
            let end = app.arm.end_effector();
            ctx.draw(&Circle {
                x: end.x,
                y: end.y,
                radius: 2.0,
                color: Color::Red,
            });

            // Target (if manual)
            if app.target_mode == TargetMode::Manual {
                ctx.draw(&Circle {
                    x: app.manual_target.x,
                    y: app.manual_target.y,
                    radius: 1.0,
                    color: Color::Gray,
                });
            }
        });

    f.render_widget(canvas, chunks[0]);

    let mode_str = match app.target_mode {
        TargetMode::Auto => "AUTO (Hunting Center of Mass)",
        TargetMode::Manual => "MANUAL (WASD)",
    };

    let status = format!(
        "Mode: {} | Sync: {:.3} | 'm': Toggle Mode | 'q': Quit",
        mode_str,
        app.world.synchronization_index()
    );
    f.render_widget(
        Paragraph::new(status).style(Style::default().bg(Color::Blue).fg(Color::Black)),
        chunks[1],
    );
}
