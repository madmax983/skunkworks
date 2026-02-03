pub mod parser;
pub mod physics;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use glam::DVec2;
use physics::{Node, System};
use rand::Rng;
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
use std::{
    collections::HashMap,
    env,
    time::{Duration, Instant},
};
use tui_shared::Tui;

struct App {
    system: System,
    camera_pos: DVec2,
    zoom: f64,
    paused: bool,
    node_count: usize,
    edge_count: usize,
}

impl App {
    fn new(path: &str) -> Result<Self> {
        let structs = parser::scan_workspace(path)?;
        let mut system = System::new();
        let mut name_to_idx = HashMap::new();
        let mut rng = rand::thread_rng();

        // 1. Create Nodes
        for s in &structs {
            // Random start position
            let pos = DVec2::new(rng.gen_range(-100.0..100.0), rng.gen_range(-100.0..100.0));

            let idx = system.add_node(Node::new(pos, s.name.clone()));
            name_to_idx.insert(s.name.clone(), idx);
        }

        // 2. Create Edges
        for s in &structs {
            if let Some(&source_idx) = name_to_idx.get(&s.name) {
                for field_type in &s.fields {
                    if let Some(&target_idx) = name_to_idx.get(field_type) {
                        // Don't link to self
                        if source_idx != target_idx {
                            system.add_edge(source_idx, target_idx);
                        }
                    }
                }
            }
        }

        let node_count = system.nodes.len();
        let edge_count = system.edges.len();

        Ok(Self {
            system,
            camera_pos: DVec2::ZERO,
            zoom: 1.0,
            paused: false,
            node_count,
            edge_count,
        })
    }

    fn on_tick(&mut self) {
        if !self.paused {
            self.system.step(0.1);
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 { &args[1] } else { "." };

    let mut tui = Tui::init()?;
    let mut app = App::new(path)?;

    if app.node_count == 0 {
        // Just run a dummy loop to show error, or exit?
        // Let's exit gracefully with a message
        // But we are in Tui mode, so printing to stdout is hidden.
        // We should render the error.
    }

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
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char(' ') => app.paused = !app.paused,
                        // Pan
                        KeyCode::Char('w') | KeyCode::Up => app.camera_pos.y += 10.0 / app.zoom,
                        KeyCode::Char('s') | KeyCode::Down => app.camera_pos.y -= 10.0 / app.zoom,
                        KeyCode::Char('a') | KeyCode::Left => app.camera_pos.x -= 10.0 / app.zoom,
                        KeyCode::Char('d') | KeyCode::Right => app.camera_pos.x += 10.0 / app.zoom,
                        // Zoom
                        KeyCode::Char('+') | KeyCode::Char('=') => app.zoom *= 1.1,
                        KeyCode::Char('-') | KeyCode::Char('_') => app.zoom /= 1.1,
                        // Reset
                        KeyCode::Char('r') => {
                            app.camera_pos = DVec2::ZERO;
                            app.zoom = 1.0;
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
    }
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    if app.node_count == 0 {
        let p = Paragraph::new("No structs found in this directory.")
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL).title("Error"));
        f.render_widget(p, chunks[0]);
        return;
    }

    let aspect = chunks[0].width as f64 / chunks[0].height.max(1) as f64;
    let view_height = 200.0 / app.zoom;
    let view_width = view_height * aspect * 2.0; // Correct for char aspect ratio ~0.5

    let x_min = app.camera_pos.x - view_width / 2.0;
    let x_max = app.camera_pos.x + view_width / 2.0;
    let y_min = app.camera_pos.y - view_height / 2.0;
    let y_max = app.camera_pos.y + view_height / 2.0;

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Struct Soup 🍜 "),
        )
        .x_bounds([x_min, x_max])
        .y_bounds([y_min, y_max])
        .paint(|ctx| {
            // Draw Edges
            for edge in &app.system.edges {
                let p1 = app.system.nodes[edge.source].pos;
                let p2 = app.system.nodes[edge.target].pos;

                ctx.draw(&CanvasLine {
                    x1: p1.x,
                    y1: p1.y,
                    x2: p2.x,
                    y2: p2.y,
                    color: Color::DarkGray,
                });
            }

            // Draw Nodes
            for node in &app.system.nodes {
                ctx.draw(&Circle {
                    x: node.pos.x,
                    y: node.pos.y,
                    radius: 2.0, // Fixed radius for now
                    color: Color::Cyan,
                });

                // Draw Label?
                // Only if zoomed in enough or few nodes
                if app.zoom > 0.5 {
                    ctx.print(
                        node.pos.x + 3.0,
                        node.pos.y,
                        Span::raw(node.name.clone()).fg(Color::Yellow),
                    );
                }
            }
        });

    f.render_widget(canvas, chunks[0]);

    let status = format!(
        "Nodes: {} | Edges: {} | Zoom: {:.2} | Pos: ({:.0}, {:.0}) | [WASD] Pan | [+/-] Zoom | [Space] Pause | [Q] Quit",
        app.node_count, app.edge_count, app.zoom, app.camera_pos.x, app.camera_pos.y
    );
    f.render_widget(
        Paragraph::new(status).style(Style::default().bg(Color::Blue).fg(Color::White)),
        chunks[1],
    );
}
