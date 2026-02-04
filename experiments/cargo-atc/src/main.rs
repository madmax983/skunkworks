mod engine;
mod graph;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use engine::GameEngine;
use graph::DepGraph;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Circle, Line as CanvasLine},
        Block, Borders, Gauge, List, ListItem, ListState, Paragraph,
    },
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

fn main() -> Result<()> {
    // 1. Setup Game
    let width = 200.0;
    let height = 150.0;
    let graph = DepGraph::new(width, height)?;
    let mut engine = GameEngine::new(graph, 4); // 4 Runways

    // 2. Setup TUI
    let mut tui = Tui::init()?;
    let mut app_state = AppState::default();

    let tick_rate = Duration::from_millis(50); // 20 TPS
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui(f, &engine, &mut app_state))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Down => {
                            if !engine.ready_queue.is_empty() {
                                let i = match app_state.queue_selection {
                                    Some(i) => (i + 1).min(engine.ready_queue.len() - 1),
                                    None => 0,
                                };
                                app_state.queue_selection = Some(i);
                            }
                        }
                        KeyCode::Up => {
                            if !engine.ready_queue.is_empty() {
                                let i = match app_state.queue_selection {
                                    Some(i) => i.saturating_sub(1),
                                    None => 0,
                                };
                                app_state.queue_selection = Some(i);
                            }
                        }
                        KeyCode::Char(c) if c.is_ascii_digit() => {
                            if let Some(digit) = c.to_digit(10) {
                                // 1-based index input, 0-based internal
                                let runway_idx = (digit as usize).saturating_sub(1);
                                if let Some(queue_idx) = app_state.queue_selection {
                                    if engine.assign_crate(queue_idx, runway_idx) {
                                        // If successful, clamp selection
                                        if engine.ready_queue.is_empty() {
                                            app_state.queue_selection = None;
                                        } else if queue_idx >= engine.ready_queue.len() {
                                            app_state.queue_selection =
                                                Some(engine.ready_queue.len() - 1);
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            engine.tick();
            last_tick = Instant::now();
        }

        if engine.game_over {
            // Simple game over screen could go here, or just exit
            // For now, let's keep running to admire the empty list
        }
    }

    Ok(())
}

struct AppState {
    queue_selection: Option<usize>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            queue_selection: Some(0),
        }
    }
}

fn ui(f: &mut Frame, engine: &GameEngine, app_state: &mut AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(f.area());

    // Left: Radar
    draw_radar(f, chunks[0], engine);

    // Right: Controls
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(12), // Runways
            Constraint::Min(10),    // Queue
            Constraint::Length(6),  // Stats
        ])
        .split(chunks[1]);

    draw_runways(f, right_chunks[0], engine);
    draw_queue(f, right_chunks[1], engine, app_state);
    draw_stats(f, right_chunks[2], engine);
}

fn draw_radar(f: &mut Frame, area: Rect, engine: &GameEngine) {
    let graph = &engine.graph;

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Dependency Radar"),
        )
        .x_bounds([0.0, graph.width])
        .y_bounds([0.0, graph.height])
        .paint(|ctx| {
            // Draw Edges
            for edge_idx in graph.graph.edge_indices() {
                if let Some((n1, n2)) = graph.graph.edge_endpoints(edge_idx) {
                    let node1 = &graph.graph[n1];
                    let node2 = &graph.graph[n2];

                    let color = if engine.completed.contains(&n1) && engine.completed.contains(&n2)
                    {
                        Color::DarkGray
                    } else if engine.completed.contains(&n2) {
                        // Dependency done, target waiting
                        Color::Green
                    } else {
                        Color::Red
                    };

                    ctx.draw(&CanvasLine {
                        x1: node1.x,
                        y1: node1.y,
                        x2: node2.x,
                        y2: node2.y,
                        color,
                    });
                }
            }

            // Draw Nodes
            for idx in graph.graph.node_indices() {
                let node = &graph.graph[idx];
                let color = if engine.completed.contains(&idx) {
                    Color::DarkGray
                } else if engine.ready_queue.contains(&idx) {
                    Color::Green
                } else if engine.runways.iter().any(|r| r.assigned_crate == Some(idx)) {
                    Color::Yellow
                } else {
                    Color::Blue
                };

                ctx.draw(&Circle {
                    x: node.x,
                    y: node.y,
                    radius: 3.0,
                    color,
                });
            }
        });

    f.render_widget(canvas, area);
}

fn draw_runways(f: &mut Frame, area: Rect, engine: &GameEngine) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Runways (Press 1-4)");
    f.render_widget(block.clone(), area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            (0..engine.runways.len())
                .map(|_| Constraint::Length(2))
                .collect::<Vec<_>>(),
        )
        .split(area);

    for (i, runway) in engine.runways.iter().enumerate() {
        if i >= chunks.len() {
            break;
        }

        let label = format!("R{}: ", i + 1);
        let (percent, text) = if let Some(crate_idx) = runway.assigned_crate {
            let node = engine.get_node(crate_idx);
            let p = (runway.progress as f64 / runway.total_cost as f64).clamp(0.0, 1.0);
            (p, format!("Building {} ({:.0}%)", node.name, p * 100.0))
        } else {
            (0.0, "Idle".to_string())
        };

        let gauge = Gauge::default()
            .block(Block::default())
            .gauge_style(Style::default().fg(Color::Cyan))
            .label(format!("{}{}", label, text))
            .ratio(percent);

        f.render_widget(gauge, chunks[i]);
    }
}

fn draw_queue(f: &mut Frame, area: Rect, engine: &GameEngine, app_state: &mut AppState) {
    let items: Vec<ListItem> = engine
        .ready_queue
        .iter()
        .map(|&idx| {
            let node = engine.get_node(idx);
            let content = format!("{} v{} [{}]", node.name, node.version, node.build_cost);
            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Ready for Landing (Up/Down)"),
        )
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    let mut state = ListState::default();
    state.select(app_state.queue_selection);

    f.render_stateful_widget(list, area, &mut state);
}

fn draw_stats(f: &mut Frame, area: Rect, engine: &GameEngine) {
    let total = engine.graph.graph.node_count();
    let completed = engine.completed.len();

    let text = vec![
        Line::from(vec![
            Span::raw("Status: "),
            if engine.game_over {
                Span::styled("ALL SYSTEMS GO", Style::default().fg(Color::Green))
            } else {
                Span::raw("Building...")
            },
        ]),
        Line::from(format!("Progress: {}/{} Crates", completed, total)),
        Line::from(format!("Ticks: {}", engine.ticks)),
        Line::from("Controls: [Arrows] Select, [1-4] Assign, [Q] Quit"),
    ];

    let p = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Tower Control"),
    );
    f.render_widget(p, area);
}
