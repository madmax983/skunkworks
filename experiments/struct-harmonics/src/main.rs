pub mod audio;
pub mod parser;
pub mod physics;

use anyhow::Result;
use audio::{map_mass_to_freq, AudioEngine};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use glam::DVec2;
use physics::{Node, NodeKind, System};
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
    audio: Option<AudioEngine>,
    camera_pos: DVec2,
    zoom: f64,
    paused: bool,
    show_help: bool,
    node_count: usize,
    edge_count: usize,
    // Audio State
    last_played: Instant,
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

            let kind = match s.kind {
                parser::ItemType::Struct => NodeKind::Struct,
                parser::ItemType::Enum => NodeKind::Enum,
            };

            // Mass = number of fields (dependencies) + 1.0 base
            // Enums fields usually count variants, Structs fields count members
            let mass = (s.fields.len() as f64) + 2.0;

            let idx = system.add_node(Node::new(pos, s.name.clone(), kind, mass));
            name_to_idx.insert(s.name.clone(), idx);
        }

        // 2. Create Edges
        for s in &structs {
            if let Some(&source_idx) = name_to_idx.get(&s.name) {
                for field_type in &s.fields {
                    if let Some(&target_idx) = name_to_idx.get(field_type) {
                        if source_idx != target_idx {
                            system.add_edge(source_idx, target_idx);
                        }
                    }
                }
            }
        }

        let node_count = system.nodes.len();
        let edge_count = system.edges.len();

        let audio = AudioEngine::new();

        Ok(Self {
            system,
            audio,
            camera_pos: DVec2::ZERO,
            zoom: 1.0,
            paused: false,
            show_help: false,
            node_count,
            edge_count,
            last_played: Instant::now(),
        })
    }

    fn on_tick(&mut self) {
        if !self.paused {
            self.system.step(0.1);
        }

        // Audio Logic: Play frequencies of nearby nodes
        if self.last_played.elapsed() >= Duration::from_millis(150) {
            if let Some(audio) = &self.audio {
                // Find nearest node
                let mut min_dist = f64::MAX;
                let mut nearest = None;

                for node in &self.system.nodes {
                    let dist = node.pos.distance(self.camera_pos);
                    if dist < min_dist {
                        min_dist = dist;
                        nearest = Some(node);
                    }
                }

                if let Some(node) = nearest {
                    // Volume falls off with distance
                    // Max hearing distance ~200.0 (view size)
                    let max_dist = 200.0 / self.zoom;
                    if min_dist < max_dist {
                        let vol = (1.0 - (min_dist / max_dist)).clamp(0.0, 1.0) as f32;
                        let freq = map_mass_to_freq(node.mass as f32);

                        // Only play if volume is significant
                        if vol > 0.05 {
                            audio.play_freq(freq, vol * 0.5); // 0.5 master volume
                        }
                    }
                }
            }
            self.last_played = Instant::now();
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 { &args[1] } else { "." };

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

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(35)])
        .split(area);

    let canvas_area = chunks[0];
    let sidebar_area = chunks[1];

    if app.node_count == 0 {
        let p =
            Paragraph::new("No structs found in this directory.\nUsage: struct-harmonics <path>")
                .style(Style::default().fg(Color::Red))
                .block(Block::default().borders(Borders::ALL).title("Error"));
        f.render_widget(p, canvas_area);
        return;
    }

    let aspect = canvas_area.width as f64 / canvas_area.height.max(1) as f64;
    let view_height = 200.0 / app.zoom;
    let view_width = view_height * aspect * 2.0;

    let x_min = app.camera_pos.x - view_width / 2.0;
    let x_max = app.camera_pos.x + view_width / 2.0;
    let y_min = app.camera_pos.y - view_height / 2.0;
    let y_max = app.camera_pos.y + view_height / 2.0;

    // Find Nearest Node
    let mut nearest_node_idx = None;
    let mut min_dist = f64::MAX;
    for (i, node) in app.system.nodes.iter().enumerate() {
        let dist = node.pos.distance_squared(app.camera_pos);
        if dist < min_dist {
            min_dist = dist;
            nearest_node_idx = Some(i);
        }
    }

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Struct Harmonics 🎼 "),
        )
        .x_bounds([x_min, x_max])
        .y_bounds([y_min, y_max])
        .paint(move |ctx| {
            // Edges
            for edge in &app.system.edges {
                let p1 = app.system.nodes[edge.source].pos;
                let p2 = app.system.nodes[edge.target].pos;

                let is_connected =
                    nearest_node_idx.is_some_and(|idx| edge.source == idx || edge.target == idx);

                let color = if is_connected {
                    Color::Cyan
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

            // Nodes
            for (i, node) in app.system.nodes.iter().enumerate() {
                let is_nearest = nearest_node_idx == Some(i);

                let color = match (node.kind, is_nearest) {
                    (NodeKind::Struct, true) => Color::LightYellow,
                    (NodeKind::Struct, false) => Color::Yellow,
                    (NodeKind::Enum, true) => Color::LightMagenta,
                    (NodeKind::Enum, false) => Color::Magenta,
                };

                // Radius based on mass
                let radius = node.mass.sqrt() * (if is_nearest { 1.5 } else { 1.0 });

                ctx.draw(&Circle {
                    x: node.pos.x,
                    y: node.pos.y,
                    radius,
                    color,
                });

                if is_nearest || app.zoom > 1.2 {
                    ctx.print(
                        node.pos.x + radius + 1.0,
                        node.pos.y,
                        Span::raw(node.name.clone()).fg(color),
                    );
                }
            }

            // Draw "Listener" (Camera Center)
            ctx.print(
                app.camera_pos.x,
                app.camera_pos.y,
                Span::raw("👂").fg(Color::White),
            );
        });

    f.render_widget(canvas, canvas_area);

    // Sidebar
    let sidebar_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(10),
            Constraint::Length(7),
            Constraint::Min(0),
        ])
        .split(sidebar_area);

    let stats_text = vec![Line::from(vec![
        Span::styled("Nodes: ", Style::default().fg(Color::Gray)),
        Span::raw(app.node_count.to_string()),
        Span::raw(" | "),
        Span::styled("Links: ", Style::default().fg(Color::Gray)),
        Span::raw(app.edge_count.to_string()),
    ])];
    f.render_widget(
        Paragraph::new(stats_text).block(Block::default().borders(Borders::ALL).title(" Stats ")),
        sidebar_chunks[0],
    );

    if let Some(idx) = nearest_node_idx {
        let node = &app.system.nodes[idx];
        let freq = map_mass_to_freq(node.mass as f32);

        let details = vec![
            Line::from(vec![
                Span::styled("Name: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    node.name.clone(),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Mass/Fields: ", Style::default().fg(Color::Gray)),
                Span::raw(format!("{:.1}", node.mass)),
            ]),
            Line::from(vec![
                Span::styled("Freq: ", Style::default().fg(Color::Gray)),
                Span::raw(format!("{:.1} Hz", freq)),
            ]),
            Line::from(vec![
                Span::styled("Pos: ", Style::default().fg(Color::Gray)),
                Span::raw(format!("({:.1}, {:.1})", node.pos.x, node.pos.y)),
            ]),
        ];
        f.render_widget(
            Paragraph::new(details)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Auditory Focus "),
                )
                .wrap(Wrap { trim: true }),
            sidebar_chunks[1],
        );
    } else {
        f.render_widget(
            Paragraph::new("Move closer to hear...").block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Auditory Focus "),
            ),
            sidebar_chunks[1],
        );
    }

    // Legend
    let legend_items = vec![
        ListItem::new(Line::from(vec![
            Span::styled("●", Style::default().fg(Color::Yellow)),
            Span::raw(" Struct (Mass)"),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("●", Style::default().fg(Color::Magenta)),
            Span::raw(" Enum (Mass)"),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("👂", Style::default().fg(Color::White)),
            Span::raw(" Listener"),
        ])),
    ];
    let legend =
        List::new(legend_items).block(Block::default().borders(Borders::ALL).title(" Legend "));
    f.render_widget(legend, sidebar_chunks[2]);

    // Help
    let help_prompt = Paragraph::new("Press '?' or 'h'\nfor Controls")
        .style(Style::default().fg(Color::Cyan))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help_prompt, sidebar_chunks[3]);

    // Help Modal
    if app.show_help {
        let block = Block::default().title(" Controls ").borders(Borders::ALL);
        let area = centered_rect(60, 50, area);
        f.render_widget(Clear, area);
        f.render_widget(block, area);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(0)])
            .split(area);

        let text = vec![
            Line::from("Navigation:"),
            Line::from("  WASD / Arrows : Move Listener (Ear)"),
            Line::from("  + / -         : Zoom In/Out"),
            Line::from("  R             : Reset View"),
            Line::from(""),
            Line::from("Simulation:"),
            Line::from("  Space         : Pause/Resume Physics"),
            Line::from(""),
            Line::from("Audio:"),
            Line::from("  Proximity to nodes triggers their frequency."),
            Line::from("  Larger Structs (More fields) = Lower Pitch"),
            Line::from(""),
            Line::from("General:"),
            Line::from("  h / ?         : Toggle Help"),
            Line::from("  q / Esc       : Quit"),
        ];

        let p = Paragraph::new(text).wrap(Wrap { trim: true });
        f.render_widget(p, layout[0]);
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
