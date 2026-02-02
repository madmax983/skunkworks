mod game;
mod track;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Color,
    text::Line,
    widgets::{
        canvas::{Canvas, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

use game::GameState;
use track::Track;

fn main() -> Result<()> {
    // 1. Init TUI
    let mut tui = Tui::init()?;

    // 2. Load Track
    // Try to load from current directory, fallback to dummy if fail
    let track = match Track::from_git(".", 500) {
        Ok(t) => t,
        Err(_) => {
            // Fallback for tests or empty repos
            let mut t = Track::new();
            t.segments.push(track::Segment {
                curvature: 0.0,
                length: 100.0,
                description: "Fallback Track".into(),
            });
            t.compute_geometry();
            t
        }
    };

    let mut game = GameState::new(track);

    // 3. Loop
    let tick_rate = Duration::from_millis(16); // 60 FPS
    let mut last_tick = Instant::now();
    let mut running = true;

    while running {
        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(f.area());

            // Left: Game Canvas
            let center_x = game.track.get_x_at(game.car.distance);

            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Diff Drift 🚗"),
                )
                .x_bounds([center_x - 40.0, center_x + 40.0])
                .y_bounds([game.car.distance - 10.0, game.car.distance + 40.0])
                .paint(|ctx| {
                    // Draw Track Center Line
                    // Optimize: finding start index
                    // For now, iterate all points (simple)
                    // TODO: Optimize if slow

                    let mut prev_point: Option<(f64, f64, f64)> = None;

                    for point in &game.track.points {
                        let (py, px, pangle) = *point;

                        // Culling
                        if py < game.car.distance - 20.0 {
                            continue;
                        }
                        if py > game.car.distance + 60.0 {
                            break;
                        }

                        if let Some((prev_y, prev_x, prev_angle)) = prev_point {
                            // Center Line (Dashed?)
                            ctx.draw(&CanvasLine {
                                x1: prev_x,
                                y1: prev_y,
                                x2: px,
                                y2: py,
                                color: Color::DarkGray,
                            });

                            // Calculate Normals
                            let w = 4.0; // Half Width (Total width 8)

                            let nx1 = -prev_angle.sin() * w;
                            let ny1 = prev_angle.cos() * w;

                            let nx2 = -pangle.sin() * w;
                            let ny2 = pangle.cos() * w;

                            // Left Border
                            ctx.draw(&CanvasLine {
                                x1: prev_x + nx1,
                                y1: prev_y + ny1,
                                x2: px + nx2,
                                y2: py + ny2,
                                color: Color::Blue,
                            });

                            // Right Border
                            ctx.draw(&CanvasLine {
                                x1: prev_x - nx1,
                                y1: prev_y - ny1,
                                x2: px - nx2,
                                y2: py - ny2,
                                color: Color::Blue,
                            });
                        }
                        prev_point = Some((py, px, pangle));
                    }

                    // Draw Car
                    // Car X is offset from track center at current distance
                    // But our track logic already puts car.x as relative to center?
                    // Yes, car.x is -1.0 to 1.0 relative to track width?
                    // Wait, in physics I used car.x as absolute offset?
                    // "self.car.x += self.car.steer_angle ..."
                    // "if self.car.x.abs() > 1.2"
                    // And I used "track_width: 20.0" in comment, but 1.2 in logic.
                    // So car.x is "normalized" units.
                    // If track width is 8.0 (w=4.0), then visual X = center_x + car.x * 4.0.

                    let track_center_x = game.track.get_x_at(game.car.distance);
                    // Need angle to orient car
                    // For now assume car aligns with track?
                    // Or simple circle.

                    let visual_car_x = track_center_x + game.car.x * 4.0;

                    ctx.print(visual_car_x, game.car.distance, "🏎️");

                    // Draw some debug info
                    // ctx.print(-15.0, game.car.distance + 30.0, format!("Score: {}", game.score).into());
                });

            f.render_widget(canvas, chunks[0]);

            // Right: HUD
            let hud_text: Vec<String> = vec![
                format!("Score: {}", game.score),
                format!("Speed: {:.1}", game.car.speed),
                format!("Dist:  {:.1}", game.car.distance),
                format!("Offset: {:.2}", game.car.x),
                "".to_string(),
                "Controls:".to_string(),
                " [Up]    Accelerate".to_string(),
                " [Down]  Brake".to_string(),
                " [Left]  Steer Left".to_string(),
                " [Right] Steer Right".to_string(),
                " [q]     Quit".to_string(),
            ];

            let lines: Vec<Line> = hud_text.iter().map(|s| Line::from(s.as_str())).collect();
            let hud =
                Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("HUD"));
            f.render_widget(hud, chunks[1]);
        })?;

        // Input
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => running = false,
                        KeyCode::Left => game.steer(-1.0),
                        KeyCode::Right => game.steer(1.0),
                        KeyCode::Up => game.accelerate(5.0),
                        KeyCode::Down => game.accelerate(-5.0),
                        _ => {}
                    }
                } else if key.kind == KeyEventKind::Release {
                    match key.code {
                        KeyCode::Left | KeyCode::Right => game.steer(0.0),
                        _ => {}
                    }
                }
            }
        }

        // Update
        if last_tick.elapsed() >= tick_rate {
            let dt = last_tick.elapsed().as_secs_f64();
            game.update(dt);
            last_tick = Instant::now();
        }
    }

    Ok(())
}
