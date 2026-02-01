use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Color,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Line, Points},
        Block, Borders, Paragraph,
    },
    Frame,
};

use crate::physics::Graph;

pub fn ui(f: &mut Frame, graph: &Graph) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    // Calculate bounds to center the galaxy
    // Find min/max x and y
    let mut min_x = -100.0;
    let mut max_x = 100.0;
    let mut min_y = -100.0;
    let mut max_y = 100.0;

    if !graph.nodes.is_empty() {
        min_x = graph.nodes[0].x;
        max_x = graph.nodes[0].x;
        min_y = graph.nodes[0].y;
        max_y = graph.nodes[0].y;

        for node in &graph.nodes {
            if node.x < min_x {
                min_x = node.x;
            }
            if node.x > max_x {
                max_x = node.x;
            }
            if node.y < min_y {
                min_y = node.y;
            }
            if node.y > max_y {
                max_y = node.y;
            }
        }
    }

    // Add padding
    let padding = 20.0;
    min_x -= padding;
    max_x += padding;
    min_y -= padding;
    max_y += padding;

    // Canvas
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Git Galaxy"))
        .x_bounds([min_x, max_x])
        .y_bounds([min_y, max_y])
        .marker(Marker::Braille)
        .paint(|ctx| {
            // Draw Edges
            for edge in &graph.edges {
                let u = &graph.nodes[edge.source];
                let v = &graph.nodes[edge.target];
                ctx.draw(&Line {
                    x1: u.x,
                    y1: u.y,
                    x2: v.x,
                    y2: v.y,
                    color: Color::DarkGray,
                });
            }

            // Draw Nodes
            for node in &graph.nodes {
                // Determine color
                let color = Color::Rgb(node.color.0, node.color.1, node.color.2);
                ctx.draw(&Points {
                    coords: &[(node.x, node.y)],
                    color,
                });
            }
        });

    f.render_widget(canvas, chunks[0]);

    // Info
    let node_count = graph.nodes.len();
    let edge_count = graph.edges.len();
    let info_text = format!(
        "Nodes: {} | Edges: {} | Press 'q' to quit",
        node_count, edge_count
    );
    let info = Paragraph::new(info_text).block(Block::default().borders(Borders::ALL));
    f.render_widget(info, chunks[1]);
}
