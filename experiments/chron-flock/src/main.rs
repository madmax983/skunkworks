pub mod blame;
pub mod boid;
pub mod world;

use anyhow::Result;
use blame::{BlameAnalyzer, LineInfo};
use boid::Boid;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, canvas::Canvas},
    Terminal,
};
use std::env;
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};
use tui_shared::Tui;
use world::World;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: chron-flock <file_path>");
        return Ok(());
    }
    let file_path_str = &args[1];
    let file_path = Path::new(file_path_str);

    // Analyze Git Blame Data
    println!("Analyzing {}...", file_path_str);
    let analyzer = BlameAnalyzer::new(".");
    let blame_info = analyzer.analyze(file_path)?;
    let content = fs::read_to_string(file_path)?;

    let mut tui = Tui::init()?;
    let res = run_app(&mut tui, file_path_str, blame_info, content);
    tui.exit()?;

    if let Err(err) = res {
        eprintln!("{:?}", err);
    }
    Ok(())
}

struct App {
    world: World,
    running: bool,
    blame_info: Vec<LineInfo>,
    content: Vec<String>,
}

impl App {
    fn new(width: f64, height: f64, blame_info: Vec<LineInfo>, content: String) -> Self {
        Self {
            world: World::new(width, height, blame_info.clone()),
            running: true,
            blame_info,
            content: content.lines().map(|s| s.to_string()).collect(),
        }
    }

    fn on_tick(&mut self) {
        self.world.update();
    }
}

fn run_app(
    tui: &mut Tui,
    file_path: &str,
    blame_info: Vec<LineInfo>,
    content: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let world_width = 100.0;
    let world_height = content.lines().count().max(50) as f64;

    let mut app = App::new(world_width, world_height, blame_info, content);

    let tick_rate = Duration::from_millis(33);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui(f, &app, file_path))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => app.running = false,
                        KeyCode::Char('r') => {
                            app.world = World::new(world_width, world_height, app.blame_info.clone());
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

fn ui(f: &mut ratatui::Frame, app: &App, file_path: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    let sync_index = app.world.synchronization_index();
    let width = app.world.width;
    let height = app.world.height;

    // Create text background lines
    let mut bg_lines = vec![];
    for (i, line) in app.content.iter().enumerate() {
        let age_score = app.blame_info.get(i).map(|info| info.age_score).unwrap_or(0.0);
        let color = if age_score > 0.8 {
            Color::LightRed // Hot/new code
        } else if age_score > 0.5 {
            Color::Yellow
        } else {
            Color::DarkGray // Cold/old code
        };
        bg_lines.push(ratatui::text::Line::from(Span::styled(
            line.clone(),
            Style::default().fg(color),
        )));
    }

    // Render Canvas
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Chron-Flock: {}", file_path)),
        )
        .x_bounds([0.0, width])
        .y_bounds([0.0, height])
        .paint(|ctx| {
            // Because ratatui canvas flips Y, we invert Y rendering so text starts from top
            // Render text
            for (i, line) in app.content.iter().enumerate() {
                let age_score = app.blame_info.get(i).map(|info| info.age_score).unwrap_or(0.0);
                let color = if age_score > 0.8 {
                    Color::Rgb(255, 100, 100) // Hot/new code
                } else if age_score > 0.5 {
                    Color::Rgb(255, 200, 100)
                } else {
                    Color::DarkGray // Cold/old code
                };
                let y = height - (i as f64) - 1.0;
                ctx.print(0.0, y, Span::styled(line.clone(), Style::default().fg(color)));
            }

            // Render boids
            for boid in &app.world.boids {
                let (char_str, color) = if boid.flash_timer > 0 {
                    ("★".to_string(), Color::White)
                } else {
                    let base_char = boid.dna.char_representation.to_string();
                    let color = boid.dna.color;
                    (base_char, color)
                };

                // Invert boid y-coordinate for rendering
                let render_y = height - boid.position.y - 1.0;
                ctx.print(
                    boid.position.x,
                    render_y,
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
