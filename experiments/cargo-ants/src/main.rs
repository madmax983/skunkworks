mod graph;
mod sim;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{
        canvas::{Canvas, Line, Points, Circle},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;
use crate::graph::DepGraph;
use crate::sim::World;

fn main() -> Result<()> {
    // 1. Setup World
    let width = 200.0;
    let height = 150.0;
    let graph = DepGraph::new(width, height)?;
    let mut world = World::new(graph, 50); // Start with 50 ants

    // 2. Setup TUI
    let mut tui = Tui::init()?;

    let tick_rate = Duration::from_millis(16); // ~60 FPS
    let mut last_tick = Instant::now();
    let mut paused = false;

    loop {
        // Prepare data for rendering to avoid borrowing issues in closure
        let render_data = prepare_render_data(&world);

        tui.terminal.draw(|f| ui(f, &world, &render_data, paused))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(' ') => paused = !paused,
                    KeyCode::Char('r') => {
                        // Reset layout velocities
                        for node in world.dep_graph.graph.node_weights_mut() {
                            node.vx = 0.0;
                            node.vy = 0.0;
                        }
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            if !paused {
                // Physics steps - maybe run multiple for stability?
                // Graph layout needs small DT
                world.tick(0.1);
            }
            last_tick = Instant::now();
        }
    }

    Ok(())
}

struct RenderData {
    nodes: Vec<(f64, f64, Color, String)>,
    edges: Vec<(f64, f64, f64, f64, Color)>,
    ants: Vec<(f64, f64, Color)>,
}

fn prepare_render_data(world: &World) -> RenderData {
    let graph = &world.dep_graph.graph;

    // Nodes
    let nodes: Vec<_> = graph.node_weights().map(|n| {
        let color = if n.is_root { Color::Red } else { Color::Blue };
        (n.x, n.y, color, n.name.clone())
    }).collect();

    // Edges
    let edges: Vec<_> = graph.edge_indices().map(|e_idx| {
        let (n1, n2) = graph.edge_endpoints(e_idx).unwrap();
        let node1 = &graph[n1];
        let node2 = &graph[n2];
        let edge = &graph[e_idx];

        let intensity = (edge.pheromone * 20.0).clamp(50.0, 255.0) as u8;
        let color = Color::Rgb(intensity, intensity, intensity);

        (node1.x, node1.y, node2.x, node2.y, color)
    }).collect();

    // Ants
    let ants: Vec<_> = world.ants.iter().map(|ant| {
        let x: f64;
        let y: f64;

        if let Some(edge_idx) = ant.target_edge {
            // Moving along edge
            // Edge is Source -> Target (A -> B, A depends on B)
            // Ant moves Target -> Source (B -> A)
            if let Some((source, target)) = graph.edge_endpoints(edge_idx) {
                 let s_node = &graph[source];
                 let t_node = &graph[target];
                 // Ant moves from Target (B) to Source (A)
                 // Lerp(Target, Source, progress)
                 x = t_node.x + (s_node.x - t_node.x) * ant.progress;
                 y = t_node.y + (s_node.y - t_node.y) * ant.progress;
            } else {
                let n = &graph[ant.current_node];
                x = n.x;
                y = n.y;
            }
        } else {
            let n = &graph[ant.current_node];
            x = n.x;
            y = n.y;
        }
        (x, y, Color::Yellow)
    }).collect();

    RenderData { nodes, edges, ants }
}

fn ui(f: &mut Frame, world: &World, data: &RenderData, _paused: bool) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Cargo Ants - Dependency Foraging"),
        )
        .x_bounds([0.0, world.dep_graph.width])
        .y_bounds([0.0, world.dep_graph.height])
        .paint(|ctx| {
            // Draw Edges
            for (x1, y1, x2, y2, color) in &data.edges {
                ctx.draw(&Line {
                    x1: *x1,
                    y1: *y1,
                    x2: *x2,
                    y2: *y2,
                    color: *color,
                });
            }

            // Draw Nodes
            for (x, y, color, _name) in &data.nodes {
                ctx.draw(&Circle {
                    x: *x,
                    y: *y,
                    radius: 2.0,
                    color: *color,
                });
                // Optional: Draw names if not too cluttered?
                // ctx.print(*x, *y, Span::raw(_name));
            }

            // Draw Ants
            for (x, y, color) in &data.ants {
                 ctx.draw(&Points {
                    coords: &[(*x, *y)],
                    color: *color,
                 });
            }
        });

    f.render_widget(canvas, chunks[0]);

    let status = format!(
        "Ants: {} | Nodes: {} | Edges: {} | [Space] Pause | [r] Reset Layout | [q] Quit",
        world.ants.len(),
        world.dep_graph.graph.node_count(),
        world.dep_graph.graph.edge_count()
    );
    f.render_widget(
        Paragraph::new(status).block(Block::default().borders(Borders::ALL)),
        chunks[1],
    );
}
