use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    symbols::Marker,
    text::{Line as TextLine, Span},
    widgets::{
        canvas::{Canvas, Line, Points},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::collections::HashMap;

use crate::physics::Graph;

#[cfg(feature = "nova")]
use crate::constellations::Constellation;

fn get_top_authors(graph: &Graph) -> Vec<(String, usize, (u8, u8, u8))> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut colors: HashMap<String, (u8, u8, u8)> = HashMap::new();

    for node in &graph.nodes {
        *counts.entry(node.author.clone()).or_insert(0) += 1;
        colors.entry(node.author.clone()).or_insert(node.color);
    }

    let mut result: Vec<_> = counts
        .into_iter()
        .map(|(author, count)| {
            let color = colors[&author];
            (author, count, color)
        })
        .collect();

    result.sort_by(|a, b| b.1.cmp(&a.1));
    result.truncate(10); // Top 10
    result
}

pub struct ViewState {
    pub zoom: f64,
    pub pan_x: f64,
    pub pan_y: f64,
}

impl Default for ViewState {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            pan_x: 0.0,
            pan_y: 0.0,
        }
    }
}

pub fn ui(
    f: &mut Frame,
    graph: &Graph,
    view_state: &ViewState,
    #[cfg(feature = "nova")] constellations: &[Constellation],
) {
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(f.area());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(main_chunks[0]);

    // Calculate bounds to center the galaxy
    // Find min/max x and y
    let mut min_x = -100.0;
    let mut max_x = 100.0;
    let mut min_y = -100.0;
    let mut max_y = 100.0;

    if !graph.nodes.is_empty() {
        min_x = graph.nodes[0].pos.x;
        max_x = graph.nodes[0].pos.x;
        min_y = graph.nodes[0].pos.y;
        max_y = graph.nodes[0].pos.y;

        for node in &graph.nodes {
            if node.pos.x < min_x {
                min_x = node.pos.x;
            }
            if node.pos.x > max_x {
                max_x = node.pos.x;
            }
            if node.pos.y < min_y {
                min_y = node.pos.y;
            }
            if node.pos.y > max_y {
                max_y = node.pos.y;
            }
        }
    }

    // Add padding
    let padding = 20.0;
    min_x -= padding;
    max_x += padding;
    min_y -= padding;
    max_y += padding;

    // Apply ViewState (Zoom and Pan)
    let center_x = (min_x + max_x) / 2.0;
    let center_y = (min_y + max_y) / 2.0;
    let width = max_x - min_x;
    let height = max_y - min_y;

    let view_width = width / view_state.zoom;
    let view_height = height / view_state.zoom;

    let final_min_x = center_x + view_state.pan_x - view_width / 2.0;
    let final_max_x = center_x + view_state.pan_x + view_width / 2.0;
    let final_min_y = center_y + view_state.pan_y - view_height / 2.0;
    let final_max_y = center_y + view_state.pan_y + view_height / 2.0;

    // Canvas
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Git Galaxy"))
        .x_bounds([final_min_x, final_max_x])
        .y_bounds([final_min_y, final_max_y])
        .marker(Marker::Braille)
        .paint(move |ctx| {
            // Draw Edges
            for edge in &graph.edges {
                let u = &graph.nodes[edge.source];
                let v = &graph.nodes[edge.target];
                ctx.draw(&Line {
                    x1: u.pos.x,
                    y1: u.pos.y,
                    x2: v.pos.x,
                    y2: v.pos.y,
                    color: Color::DarkGray,
                });
            }

            // Draw Constellations (Nova)
            #[cfg(feature = "nova")]
            {
                for constellation in constellations {
                    if constellation.nodes.len() > 1 {
                        for i in 0..constellation.nodes.len() - 1 {
                            let idx1 = constellation.nodes[i];
                            let idx2 = constellation.nodes[i + 1];
                            let u = &graph.nodes[idx1];
                            let v = &graph.nodes[idx2];
                            ctx.draw(&Line {
                                x1: u.pos.x,
                                y1: u.pos.y,
                                x2: v.pos.x,
                                y2: v.pos.y,
                                color: constellation.color,
                            });
                        }
                    } else if constellation.nodes.len() == 1 {
                        let u = &graph.nodes[constellation.nodes[0]];
                        // Cross
                        ctx.draw(&Line {
                            x1: u.pos.x - 2.0,
                            y1: u.pos.y,
                            x2: u.pos.x + 2.0,
                            y2: u.pos.y,
                            color: constellation.color,
                        });
                        ctx.draw(&Line {
                            x1: u.pos.x,
                            y1: u.pos.y - 2.0,
                            x2: u.pos.x,
                            y2: u.pos.y + 2.0,
                            color: constellation.color,
                        });
                    }
                }
            }

            // Draw Nodes
            for node in &graph.nodes {
                // Determine color
                let color = Color::Rgb(node.color.0, node.color.1, node.color.2);
                ctx.draw(&Points {
                    coords: &[(node.pos.x, node.pos.y)],
                    color,
                });
            }
        });

    f.render_widget(canvas, left_chunks[0]);

    // Info
    let node_count = graph.nodes.len();
    let edge_count = graph.edges.len();
    let info_text = format!(
        "Nodes: {} | Edges: {} | Press 'q' to quit | +/- Zoom | Arrows Pan",
        node_count, edge_count
    );
    let info = Paragraph::new(info_text).block(Block::default().borders(Borders::ALL));
    f.render_widget(info, left_chunks[1]);

    // Sidebar
    #[cfg(feature = "nova")]
    let sidebar_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10),
            Constraint::Length(15),
            Constraint::Min(0),
        ])
        .split(main_chunks[1]);

    #[cfg(not(feature = "nova"))]
    let sidebar_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Min(0)])
        .split(main_chunks[1]);

    // Stats
    let stats_text = vec![
        TextLine::from(vec![Span::raw("Galaxy Stats")]),
        TextLine::from(vec![Span::raw(format!("Nodes: {}", graph.nodes.len()))]),
        TextLine::from(vec![Span::raw(format!("Edges: {}", graph.edges.len()))]),
        TextLine::from(vec![Span::raw(format!("Zoom: {:.1}x", view_state.zoom))]),
        TextLine::from(vec![Span::raw(format!(
            "Pan: {:.0}, {:.0}",
            view_state.pan_x, view_state.pan_y
        ))]),
    ];
    let stats =
        Paragraph::new(stats_text).block(Block::default().borders(Borders::ALL).title("Stats"));
    f.render_widget(stats, sidebar_chunks[0]);

    // Constellations (Nova)
    #[cfg(feature = "nova")]
    {
        let const_text: Vec<TextLine> = constellations
            .iter()
            .map(|c| {
                TextLine::from(vec![
                    Span::styled("★ ", Style::default().fg(c.color)),
                    Span::raw(format!("{}", c.name)),
                ])
            })
            .collect();

        let const_block = Paragraph::new(const_text).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Constellations"),
        );
        f.render_widget(const_block, sidebar_chunks[1]);
    }

    // Legend
    let top_authors = get_top_authors(graph);
    let mut legend_lines = Vec::new();
    for (author, count, color) in top_authors {
        legend_lines.push(TextLine::from(vec![
            Span::styled(
                "■ ",
                Style::default().fg(Color::Rgb(color.0, color.1, color.2)),
            ),
            Span::raw(format!(" {} ({})", author, count)),
        ]));
    }
    let legend = Paragraph::new(legend_lines)
        .block(Block::default().borders(Borders::ALL).title("Top Authors"));

    // Render legend at correct index
    #[cfg(feature = "nova")]
    f.render_widget(legend, sidebar_chunks[2]);

    #[cfg(not(feature = "nova"))]
    f.render_widget(legend, sidebar_chunks[1]);
}
