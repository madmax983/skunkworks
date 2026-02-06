pub mod parser;
pub mod simulation;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use glam::DVec2;
use parser::NodeKind;
use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Span,
    widgets::{
        canvas::{Canvas, Circle, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
    Frame,
};
use simulation::{Node, Simulation};
use std::{
    env,
    time::{Duration, Instant},
};
use tui_shared::Tui;

struct App {
    sim: Simulation,
    camera_pos: DVec2,
    zoom: f64,
    paused: bool,
    show_help: bool,
    node_names: Vec<String>,
    node_kinds: Vec<NodeKind>,
}

impl App {
    fn new(path: &str) -> Result<Self> {
        let ast = parser::parse_file(path)?;
        let mut sim = Simulation::new();
        let mut node_names = Vec::new();
        let mut node_kinds = Vec::new();
        let mut rng = rand::thread_rng();

        // Recursively build graph
        let root_idx = sim.add_node(Node::new(DVec2::ZERO, 10.0));
        sim.nodes[root_idx].fixed = true; // Fix root? Maybe not.
        sim.nodes[root_idx].radius = 8.0;

        node_names.push(ast.name.clone());
        node_kinds.push(ast.kind.clone());

        let mut stack = vec![(root_idx, &ast)];

        while let Some((parent_idx, parent_ast)) = stack.pop() {
            for child in &parent_ast.children {
                // Random pos near parent
                let offset = DVec2::new(rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0));
                let pos = sim.nodes[parent_idx].pos + offset;

                let mass = match child.kind {
                    NodeKind::Mod => 5.0,
                    NodeKind::Fn => 3.0,
                    NodeKind::Struct => 4.0,
                    _ => 1.0,
                };

                let radius = match child.kind {
                    NodeKind::Mod => 6.0,
                    NodeKind::Fn => 4.0,
                    NodeKind::Struct => 4.0,
                    _ => 2.0,
                };

                let child_idx = sim.add_node(Node::new(pos, mass));
                sim.nodes[child_idx].radius = radius;

                node_names.push(child.name.clone());
                node_kinds.push(child.kind.clone());

                let len = match child.kind {
                    NodeKind::Mod => 80.0,
                    _ => 40.0,
                };
                sim.add_edge(parent_idx, child_idx, len);

                stack.push((child_idx, child));
            }
        }

        Ok(Self {
            sim,
            camera_pos: DVec2::ZERO,
            zoom: 1.0,
            paused: false,
            show_help: false,
            node_names,
            node_kinds,
        })
    }

    fn on_tick(&mut self) {
        if !self.paused {
            self.sim.tick(0.05); // Fixed time step
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    // Default to self if no arg
    let path = if args.len() > 1 {
        &args[1]
    } else {
        "experiments/ast-respiration/src/main.rs"
    };

    let mut tui = Tui::init()?;
    let mut app = App::new(path)?;

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
                        KeyCode::Char('?') | KeyCode::Char('h') => app.show_help = !app.show_help,
                        // Pan
                        KeyCode::Char('w') | KeyCode::Up => app.camera_pos.y += 10.0 / app.zoom,
                        KeyCode::Char('s') | KeyCode::Down => app.camera_pos.y -= 10.0 / app.zoom,
                        KeyCode::Char('a') | KeyCode::Left => app.camera_pos.x -= 10.0 / app.zoom,
                        KeyCode::Char('d') | KeyCode::Right => app.camera_pos.x += 10.0 / app.zoom,
                        // Zoom
                        KeyCode::Char('+') | KeyCode::Char('=') => app.zoom *= 1.1,
                        KeyCode::Char('-') | KeyCode::Char('_') => app.zoom /= 1.1,
                        // Breathing Control
                        KeyCode::Char(']') => app.sim.breath_speed *= 1.1,
                        KeyCode::Char('[') => app.sim.breath_speed /= 1.1,

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
    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    let canvas_area = chunks[0];
    let status_area = chunks[1];

    let aspect = canvas_area.width as f64 / canvas_area.height.max(1) as f64;
    let view_height = 200.0 / app.zoom;
    let view_width = view_height * aspect * 2.0;

    let x_min = app.camera_pos.x - view_width / 2.0;
    let x_max = app.camera_pos.x + view_width / 2.0;
    let y_min = app.camera_pos.y - view_height / 2.0;
    let y_max = app.camera_pos.y + view_height / 2.0;

    // Visualize Breath Phase
    let breath_intensity = (app.sim.breath_phase.sin() + 1.0) / 2.0; // 0..1
    let breath_color = Color::Rgb(
        (100.0 + 155.0 * breath_intensity) as u8,
        (100.0 + 100.0 * (1.0 - breath_intensity)) as u8,
        255,
    );

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" AST Respiration 🫁 "),
        )
        .x_bounds([x_min, x_max])
        .y_bounds([y_min, y_max])
        .paint(move |ctx| {
            // Edges
            for edge in &app.sim.edges {
                let p1 = app.sim.nodes[edge.source].pos;
                let p2 = app.sim.nodes[edge.target].pos;

                ctx.draw(&CanvasLine {
                    x1: p1.x,
                    y1: p1.y,
                    x2: p2.x,
                    y2: p2.y,
                    color: Color::DarkGray,
                });
            }

            // Nodes
            for (i, node) in app.sim.nodes.iter().enumerate() {
                let color = match app.node_kinds[i] {
                    NodeKind::File => Color::Red,
                    NodeKind::Mod => Color::Magenta,
                    NodeKind::Fn => Color::Cyan,
                    NodeKind::Struct => Color::Yellow,
                    NodeKind::Impl => Color::Blue,
                    _ => Color::Gray,
                };

                // Breath effect on size?
                // Or maybe just color intensity

                ctx.draw(&Circle {
                    x: node.pos.x,
                    y: node.pos.y,
                    radius: node.radius,
                    color,
                });

                if app.zoom > 0.5 {
                    ctx.print(
                        node.pos.x + node.radius,
                        node.pos.y,
                        Span::from(app.node_names[i].clone()).fg(color),
                    );
                }
            }
        });

    f.render_widget(canvas, canvas_area);

    // Status Bar
    let status_text = format!(
        "Nodes: {} | Breath Speed: {:.2} | [+/-] Zoom | [WASD] Pan | [[/]] Breath Freq",
        app.sim.nodes.len(),
        app.sim.breath_speed
    );

    let p = Paragraph::new(status_text)
        .style(Style::default().fg(breath_color))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(p, status_area);

    if app.show_help {
        let block = Block::default().title("Help").borders(Borders::ALL);
        let area = centered_rect(60, 50, area);
        f.render_widget(ratatui::widgets::Clear, area);
        f.render_widget(block, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    let horiz_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1]);

    horiz_layout[1]
}
