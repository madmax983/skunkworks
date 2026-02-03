mod graph;
mod render;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use petgraph::Direction;
use petgraph::graph::NodeIndex;
use ratatui::{
    Frame,
    layout::{Constraint, Direction as LayoutDirection, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, canvas::Canvas},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

use graph::WorkspaceGraph;
use render::{build_scene, draw_scene};

struct App {
    graph_data: WorkspaceGraph,
    current_node: NodeIndex,
    scene_depth: usize,
    last_tick: Instant,
    selected_idx: usize,
}

impl App {
    fn new() -> Result<Self> {
        let graph_data = graph::build_graph()?;

        // Find a good starting node
        let start_node = graph_data.root_index.unwrap_or_else(|| {
            // Fallback to first node
            graph_data.graph.node_indices().next().unwrap() // Safe if graph not empty
        });

        Ok(Self {
            graph_data,
            current_node: start_node,
            scene_depth: 6,
            last_tick: Instant::now(),
            selected_idx: 0,
        })
    }

    fn on_tick(&mut self) {
        // Animation logic could go here
    }

    fn move_selection(&mut self, delta: isize) {
        self.selected_idx = (self.selected_idx as isize + delta).max(0) as usize;
    }

    fn climb(&mut self, direction: Direction) {
        let neighbors: Vec<_> = self
            .graph_data
            .graph
            .neighbors_directed(self.current_node, direction)
            .collect();

        if !neighbors.is_empty() {
            self.current_node = neighbors[self.selected_idx % neighbors.len()];
            self.selected_idx = 0; // Reset selection on move
        }
    }
}

fn main() -> Result<()> {
    // If graph build fails, print error and exit
    // But TUI init handles panic/error gracefully if we use Tui struct properly?
    // Let's try to build graph before TUI init to show error in console if any.
    // Actually App::new builds graph.

    // Check if graph can be built
    if let Err(e) = graph::build_graph() {
        eprintln!(
            "Failed to build dependency graph: {}. Make sure you are in a cargo workspace.",
            e
        );
        return Ok(());
    }

    let mut tui = Tui::init()?;
    let mut app = App::new()?;

    loop {
        tui.terminal.draw(|f| ui(f, &app))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Up | KeyCode::Char('w') => app.climb(Direction::Outgoing),
                    KeyCode::Down | KeyCode::Char('s') => app.climb(Direction::Incoming),
                    KeyCode::Left | KeyCode::Char('a') => app.move_selection(-1),
                    KeyCode::Right | KeyCode::Char('d') => app.move_selection(1),
                    _ => {}
                }
            }
        }

        if app.last_tick.elapsed() >= Duration::from_millis(16) {
            app.on_tick();
            app.last_tick = Instant::now();
        }
    }

    tui.exit()?;
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(LayoutDirection::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let canvas_area = chunks[0];
    let info_area = chunks[1];

    let current_label = &app.graph_data.graph[app.current_node];

    // Info
    let deps_count = app
        .graph_data
        .graph
        .neighbors_directed(app.current_node, Direction::Outgoing)
        .count();
    let rev_deps_count = app
        .graph_data
        .graph
        .neighbors_directed(app.current_node, Direction::Incoming)
        .count();

    let info = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Penrose Build", Style::default().fg(Color::Cyan)),
            Span::raw(" | "),
            Span::styled(
                format!("Current: {}", current_label),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![Span::raw(format!(
            "Dependencies (Up): {} | Dependents (Down): {}",
            deps_count, rev_deps_count
        ))]),
        Line::from(vec![Span::raw(format!(
            "Selection Index: {}",
            app.selected_idx
        ))]),
        Line::from(vec![Span::raw(
            "WASD / Arrows: Up/Down to climb, Left/Right to select branch. 'q' to quit.",
        )]),
    ])
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(info, info_area);

    // Canvas
    // We build the scene relative to current_node
    let scene = build_scene(&app.graph_data.graph, app.current_node, app.scene_depth);

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Isometric Dependency Tower"),
        )
        .x_bounds([-40.0, 40.0])
        .y_bounds([-30.0, 30.0])
        .paint(move |ctx| {
            draw_scene(ctx, &scene, &app.graph_data.graph);
        });

    f.render_widget(canvas, canvas_area);
}
