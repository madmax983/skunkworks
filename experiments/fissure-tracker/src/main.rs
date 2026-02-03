pub mod scanner;
pub mod simulation;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Span,
    widgets::{
        canvas::{Canvas, Circle, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
    Frame,
};
use simulation::{Vec2, World};
use std::{env, time::{Duration, Instant}};
use tui_shared::Tui;

struct App {
    world: World,
    camera_pos: Vec2,
    zoom: f64,
    quit: bool,
}

impl App {
    fn new(path: &str) -> Self {
        let scans = scanner::scan_workspace(std::path::PathBuf::from(path));
        let world = World::new(scans);
        Self {
            world,
            camera_pos: Vec2::new(0.0, 0.0),
            zoom: 1.0,
            quit: false,
        }
    }

    fn on_tick(&mut self) {
        self.world.step();
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 { &args[1] } else { "." };

    let mut tui = Tui::init()?;
    let mut app = App::new(path);

    run_app(&mut tui, &mut app)?;

    Ok(())
}

fn run_app(tui: &mut Tui, app: &mut App) -> Result<()> {
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.quit = true,
                        KeyCode::Char('w') | KeyCode::Up => app.camera_pos.y += 10.0 / app.zoom,
                        KeyCode::Char('s') | KeyCode::Down => app.camera_pos.y -= 10.0 / app.zoom,
                        KeyCode::Char('a') | KeyCode::Left => app.camera_pos.x -= 10.0 / app.zoom,
                        KeyCode::Char('d') | KeyCode::Right => app.camera_pos.x += 10.0 / app.zoom,
                        KeyCode::Char('+') | KeyCode::Char('=') => app.zoom *= 1.1,
                        KeyCode::Char('-') | KeyCode::Char('_') => app.zoom /= 1.1,
                        KeyCode::Char('r') => {
                            app.camera_pos = Vec2::new(0.0, 0.0);
                            app.zoom = 1.0;
                        }
                        _ => {}
                    }
                }
            }
        }

        if app.quit {
            break;
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let area = f.area();

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    let canvas_area = main_layout[0];
    let status_area = main_layout[1];

    // Viewport calculation
    let aspect = canvas_area.width as f64 / canvas_area.height.max(1) as f64;
    let view_height = 200.0 / app.zoom;
    let view_width = view_height * aspect * 2.0;

    let x_min = app.camera_pos.x - view_width / 2.0;
    let x_max = app.camera_pos.x + view_width / 2.0;
    let y_min = app.camera_pos.y - view_height / 2.0;
    let y_max = app.camera_pos.y + view_height / 2.0;

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(" Fissure Tracker 🌋 "))
        .x_bounds([x_min, x_max])
        .y_bounds([y_min, y_max])
        .paint(move |ctx| {

            // Draw Fissures (Jagged Lines)
            for fissure in &app.world.fissures {
                // Color gets redder with age? Or just White?
                // Let's use Red for danger.
                let color = Color::LightRed;

                for i in 0..fissure.points.len().saturating_sub(1) {
                    let p1 = fissure.points[i];
                    let p2 = fissure.points[i+1];

                    ctx.draw(&CanvasLine {
                        x1: p1.x,
                        y1: p1.y,
                        x2: p2.x,
                        y2: p2.y,
                        color,
                    });
                }
            }

            // Draw Nodes (Files)
            for node in &app.world.nodes {
                let color = if node.stress == 0.0 {
                    Color::Green
                } else if node.stress < 5.0 {
                    Color::Yellow
                } else {
                    Color::Red
                };

                let radius = if node.stress > 0.0 { 3.0 } else { 1.5 };

                ctx.draw(&Circle {
                    x: node.pos.x,
                    y: node.pos.y,
                    radius,
                    color,
                });

                // Draw Label if zoomed in
                if app.zoom > 1.5 || node.stress > 10.0 {
                    let name = node.data.path.file_name().unwrap().to_str().unwrap().to_string();
                    ctx.print(
                        node.pos.x + radius,
                        node.pos.y + radius,
                        Span::raw(name).fg(Color::Gray),
                    );
                }
            }
        });

    f.render_widget(canvas, canvas_area);

    // Status Bar
    let status_text = format!(
        "Nodes: {} | Fissures: {} | Zoom: {:.1}x | WASD: Pan | +/-: Zoom | q: Quit",
        app.world.nodes.len(),
        app.world.fissures.len(),
        app.zoom
    );
    f.render_widget(Paragraph::new(status_text).style(Style::default().fg(Color::Black).bg(Color::White)), status_area);
}
