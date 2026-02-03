use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Line as CanvasLine, Points},
        Block, Borders, Paragraph,
    },
    Frame,
};

use crate::model::Network;

pub fn draw_ui(f: &mut Frame, network: &Network) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(30)])
        .split(f.area());

    draw_network(f, chunks[0], network);
    draw_sidebar(f, chunks[1], network);
}

fn draw_network(f: &mut Frame, area: Rect, network: &Network) {
    // Determine bounds (naive auto-fit)
    let mut min_x = 0.0;
    let mut max_x = 100.0;
    let mut min_y = 0.0;
    let mut max_y = 100.0;

    if network.graph.node_count() > 0 {
        let positions: Vec<(f64, f64)> = network
            .graph
            .node_weights()
            .map(|n| n.pos)
            .collect();

        min_x = positions.iter().map(|p| p.0).fold(f64::INFINITY, f64::min);
        max_x = positions.iter().map(|p| p.0).fold(f64::NEG_INFINITY, f64::max);
        min_y = positions.iter().map(|p| p.1).fold(f64::INFINITY, f64::min);
        max_y = positions.iter().map(|p| p.1).fold(f64::NEG_INFINITY, f64::max);
    }

    // Padding
    min_x -= 10.0;
    max_x += 10.0;
    min_y -= 10.0;
    max_y += 10.0;

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(" Network Topology "))
        .marker(symbols::Marker::Braille)
        .x_bounds([min_x, max_x])
        .y_bounds([min_y, max_y])
        .paint(|ctx| {
            // Draw Links
            for edge_idx in network.graph.edge_indices() {
                if let Some((source, target)) = network.graph.edge_endpoints(edge_idx) {
                    let p1 = network.graph[source].pos;
                    let p2 = network.graph[target].pos;
                    let link = &network.graph[edge_idx];

                    // Color based on traffic
                    let color = if link.packets.is_empty() {
                        Color::DarkGray
                    } else if link.packets.len() < 5 {
                        Color::Cyan
                    } else {
                        Color::LightCyan
                    };

                    ctx.draw(&CanvasLine {
                        x1: p1.0,
                        y1: p1.1,
                        x2: p2.0,
                        y2: p2.1,
                        color,
                    });

                    // Draw Packets on Link
                    for (_, progress) in &link.packets {
                        // Lerp
                        let px = p1.0 + (p2.0 - p1.0) * progress;
                        let py = p1.1 + (p2.1 - p1.1) * progress;

                        ctx.draw(&Points {
                            coords: &[(px, py)],
                            color: Color::White, // Could be packet color
                        });
                    }
                }
            }

            // Draw Nodes
            for node in network.graph.node_weights() {
                let q_len = node.queue.len();
                let color = if q_len == 0 {
                    Color::Green
                } else if q_len < 5 {
                    Color::Yellow
                } else if q_len < 20 {
                    Color::Red
                } else {
                    Color::Magenta
                };

                // Draw as a point (or small circle logic if we had it)
                // Just use 'O' or similar
                ctx.print(node.pos.0, node.pos.1, Span::styled("O", Style::default().fg(color)));

                // Draw Price label if high
                let price = node.current_price();
                if price > 5.0 {
                    ctx.print(node.pos.0 + 1.0, node.pos.1 + 1.0,
                        Span::styled(format!("{:.1}", price), Style::default().fg(Color::Gray).add_modifier(Modifier::DIM)));
                }
            }
        });

    f.render_widget(canvas, area);
}

fn draw_sidebar(f: &mut Frame, area: Rect, network: &Network) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Min(0)])
        .split(area);

    // Stats
    let total_q: usize = network.graph.node_weights().map(|n| n.queue.len()).sum();
    let max_price: f64 = network.graph.node_weights().map(|n| n.current_price()).fold(0.0, f64::max);

    let stats_text = vec![
        Line::from(vec![Span::raw("Ticks: "), Span::styled(format!("{}", network.tick_count), Style::default().fg(Color::Cyan))]),
        Line::from(vec![Span::raw("Total Packets: "), Span::styled(format!("{}", network.total_packets), Style::default().fg(Color::Green))]),
        Line::from(vec![Span::raw("Dropped: "), Span::styled(format!("{}", network.dropped_packets), Style::default().fg(Color::Red))]),
        Line::from(""),
        Line::from(vec![Span::raw("Queued: "), Span::styled(format!("{}", total_q), Style::default().fg(Color::Yellow))]),
        Line::from(vec![Span::raw("Max Price: "), Span::styled(format!("{:.2}", max_price), Style::default().fg(Color::Magenta))]),
    ];

    f.render_widget(
        Paragraph::new(stats_text)
            .block(Block::default().borders(Borders::ALL).title(" Market Data ")),
        chunks[0],
    );

    // Help
    let help_text = vec![
        Line::from("Controls:"),
        Line::from(" [Q] Quit"),
        Line::from(" [B] Burst (DDOS)"),
        Line::from(" [R] Reset"),
    ];

    f.render_widget(
        Paragraph::new(help_text)
            .block(Block::default().borders(Borders::ALL).title(" Help ")),
        chunks[1],
    );
}
