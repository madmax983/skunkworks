pub mod parser;
pub mod physics;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use glam::DVec2;
use physics::{Node, System};
use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Circle, Line as CanvasLine},
        Block, Borders, Clear, List, ListItem, Paragraph, Wrap,
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
    show_help: bool,
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

            let kind_idx = match s.kind {
                parser::ItemType::Struct => 0,
                parser::ItemType::Enum => 1,
            };

            let idx = system.add_node(Node::new(pos, s.name.clone(), kind_idx));
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
            show_help: false,
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
                        KeyCode::Char('q') | KeyCode::Esc => {
                            if app.show_help {
                                app.show_help = false;
                            } else {
                                break;
                            }
                        }
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
    let area = f.area();

    // Split into Main Canvas and Sidebar
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(35)])
        .split(area);

    let canvas_area = chunks[0];
    let sidebar_area = chunks[1];

    if app.node_count == 0 {
        let p = Paragraph::new("No structs found in this directory.")
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL).title("Error"));
        f.render_widget(p, canvas_area);
        return;
    }

    // 1. Calculate View
    let aspect = canvas_area.width as f64 / canvas_area.height.max(1) as f64;
    let view_height = 200.0 / app.zoom;
    let view_width = view_height * aspect * 2.0; // Correct for char aspect ratio ~0.5

    let x_min = app.camera_pos.x - view_width / 2.0;
    let x_max = app.camera_pos.x + view_width / 2.0;
    let y_min = app.camera_pos.y - view_height / 2.0;
    let y_max = app.camera_pos.y + view_height / 2.0;

    // 2. Find Nearest Node (for Sidebar & Highlighting)
    let mut nearest_node_idx = None;
    let mut min_dist = f64::MAX;
    // Heuristic: only check nodes within view? For now check all, N is small.
    for (i, node) in app.system.nodes.iter().enumerate() {
        let dist = node.pos.distance_squared(app.camera_pos);
        if dist < min_dist {
            min_dist = dist;
            nearest_node_idx = Some(i);
        }
    }

    // 3. Render Canvas
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Struct Soup 🍜 "),
        )
        .x_bounds([x_min, x_max])
        .y_bounds([y_min, y_max])
        .paint(move |ctx| {
            // Draw Edges
            for edge in &app.system.edges {
                let p1 = app.system.nodes[edge.source].pos;
                let p2 = app.system.nodes[edge.target].pos;

                // Highlight edge if connected to nearest node
                let is_connected = nearest_node_idx.map_or(false, |idx| {
                    edge.source == idx || edge.target == idx
                });

                let color = if is_connected {
                    Color::White
                } else {
                    Color::DarkGray
                };

                ctx.draw(&CanvasLine {
                    x1: p1.x,
                    y1: p1.y,
                    x2: p2.x,
                    y2: p2.y,
                    color,
                });
            }

            // Draw Nodes
            for (i, node) in app.system.nodes.iter().enumerate() {
                let is_nearest = nearest_node_idx == Some(i);

                let color = match (node.kind, is_nearest) {
                    (0, true) => Color::LightCyan,
                    (0, false) => Color::Cyan,
                    (1, true) => Color::LightMagenta,
                    (1, false) => Color::Magenta,
                    (_, true) => Color::White,
                    (_, false) => Color::Gray,
                };

                let radius = if is_nearest { 4.0 } else { 2.0 };

                ctx.draw(&Circle {
                    x: node.pos.x,
                    y: node.pos.y,
                    radius,
                    color,
                });

                // Draw Label
                if is_nearest || app.zoom > 0.8 {
                    let label_color = if is_nearest { Color::Yellow } else { Color::DarkGray };
                    ctx.print(
                        node.pos.x + radius + 1.0,
                        node.pos.y,
                        Span::raw(node.name.clone()).fg(label_color),
                    );
                }
            }
        });

    f.render_widget(canvas, canvas_area);

    // 4. Render Sidebar
    let sidebar_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title/Stats
            Constraint::Length(10), // Selection Details
            Constraint::Length(7),  // Legend
            Constraint::Min(0),     // Controls
        ])
        .split(sidebar_area);

    // Stats
    let stats_text = vec![
        Line::from(vec![
            Span::styled("Nodes: ", Style::default().fg(Color::Gray)),
            Span::raw(app.node_count.to_string()),
            Span::raw(" | "),
            Span::styled("Edges: ", Style::default().fg(Color::Gray)),
            Span::raw(app.edge_count.to_string()),
        ]),
    ];
    let stats = Paragraph::new(stats_text)
        .block(Block::default().borders(Borders::ALL).title(" Stats "));
    f.render_widget(stats, sidebar_chunks[0]);

    // Selection Details
    if let Some(idx) = nearest_node_idx {
        let node = &app.system.nodes[idx];
        let kind_str = match node.kind {
            0 => "Struct",
            1 => "Enum",
            _ => "Unknown",
        };

        let connections = app.system.edges.iter().filter(|e| e.source == idx || e.target == idx).count();

        let details = vec![
            Line::from(vec![Span::styled("Name: ", Style::default().fg(Color::Gray)), Span::styled(node.name.clone(), Style::default().add_modifier(Modifier::BOLD))]),
            Line::from(vec![Span::styled("Type: ", Style::default().fg(Color::Gray)), Span::raw(kind_str)]),
            Line::from(vec![Span::styled("Links: ", Style::default().fg(Color::Gray)), Span::raw(connections.to_string())]),
            Line::from(vec![Span::styled("Pos: ", Style::default().fg(Color::Gray)), Span::raw(format!("({:.1}, {:.1})", node.pos.x, node.pos.y))]),
        ];
        let p = Paragraph::new(details)
            .block(Block::default().borders(Borders::ALL).title(" Selection "))
            .wrap(Wrap { trim: true });
        f.render_widget(p, sidebar_chunks[1]);
    } else {
        let p = Paragraph::new("No selection")
            .block(Block::default().borders(Borders::ALL).title(" Selection "));
        f.render_widget(p, sidebar_chunks[1]);
    }

    // Legend
    let legend_items = vec![
        ListItem::new(Line::from(vec![Span::styled("●", Style::default().fg(Color::Cyan)), Span::raw(" Struct")])),
        ListItem::new(Line::from(vec![Span::styled("●", Style::default().fg(Color::Magenta)), Span::raw(" Enum")])),
        ListItem::new(Line::from(vec![Span::styled("─", Style::default().fg(Color::White)), Span::raw(" Connected")])),
    ];
    let legend = List::new(legend_items)
        .block(Block::default().borders(Borders::ALL).title(" Legend "));
    f.render_widget(legend, sidebar_chunks[2]);

    // Controls Help Prompt
    let help_prompt = Paragraph::new("Press '?' or 'h'\nfor Help")
        .style(Style::default().fg(Color::Yellow))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help_prompt, sidebar_chunks[3]);

    // 5. Render Help Modal
    if app.show_help {
        let block = Block::default().title(" Controls ").borders(Borders::ALL);
        let area = centered_rect(60, 50, area);
        f.render_widget(Clear, area); // Clear background
        f.render_widget(block, area);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(0)])
            .split(area);

        let text = vec![
            Line::from("Navigation:"),
            Line::from("  WASD / Arrows : Pan Camera"),
            Line::from("  + / -         : Zoom In/Out"),
            Line::from("  R             : Reset View"),
            Line::from(""),
            Line::from("Simulation:"),
            Line::from("  Space         : Pause/Resume"),
            Line::from(""),
            Line::from("General:"),
            Line::from("  h / ?         : Toggle Help"),
            Line::from("  q / Esc       : Quit"),
        ];

        let p = Paragraph::new(text).wrap(Wrap { trim: true });
        f.render_widget(p, layout[0]);
    }
}

/// Helper function to center a rect
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
