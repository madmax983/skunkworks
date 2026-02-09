mod quipu;
mod serializer;
mod graph;
mod simulation;

use graph::DependencyGraph;
use simulation::Simulation;
use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    prelude::*,
    symbols::Marker,
    widgets::{
        Block, Borders,
        canvas::{Canvas, Line as CanvasLine, Points},
        Paragraph,
    },
};
use rand::prelude::*;

fn main() -> io::Result<()> {
    // Setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Init Simulation
    let mut rng = rand::thread_rng();
    let mut dep_graph = DependencyGraph::new();
    // Create a larger graph
    dep_graph.generate_layered_dag(6, 4, &mut rng);
    dep_graph.calculate_layout();
    let mut sim = Simulation::new(dep_graph);

    // Loop
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(50);

    loop {
        terminal.draw(|f| ui(f, &sim))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            sim.step(&mut rng);
            last_tick = Instant::now();
        }
    }

    // Restore
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(frame: &mut Frame, sim: &Simulation) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(frame.size());

    let area_left = chunks[0];
    let area_right = chunks[1];

    // Left: Network Map
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("QuipuNet: P2P Knot Graph"))
        .marker(Marker::Braille)
        .x_bounds([0.0, 1.0])
        .y_bounds([0.0, 1.0])
        .paint(|ctx| {
            // Draw Edges
            for edge_idx in sim.graph.graph.edge_indices() {
                if let Some((source, target)) = sim.graph.graph.edge_endpoints(edge_idx) {
                    let n1 = &sim.graph.graph[source];
                    let n2 = &sim.graph.graph[target];

                    ctx.draw(&CanvasLine {
                        x1: n1.x as f64,
                        y1: 1.0 - n1.y as f64,
                        x2: n2.x as f64,
                        y2: 1.0 - n2.y as f64,
                        color: Color::DarkGray,
                    });
                }
            }

            // Draw Nodes
            for node in sim.graph.graph.node_weights() {
                ctx.draw(&Points {
                    coords: &[(node.x as f64, 1.0 - node.y as f64)],
                    color: Color::White,
                });
            }

            // Draw Packets
            for packet in &sim.packets {
                if let Some(edge_idx) = packet.current_edge {
                    if let Some((source, target)) = sim.graph.graph.edge_endpoints(edge_idx) {
                        let n1 = &sim.graph.graph[source];
                        let n2 = &sim.graph.graph[target];

                        let px = n1.x + (n2.x - n1.x) * packet.progress;
                        let py = n1.y + (n2.y - n1.y) * packet.progress;

                        // Color based on complexity/speed
                        let color = if packet.speed < 0.02 { Color::Red } else { Color::Green };

                        ctx.draw(&Points {
                            coords: &[(px as f64, 1.0 - py as f64)],
                            color,
                        });
                    }
                }
            }
        });
    frame.render_widget(canvas, area_left);

    // Right: Quipu Inspector
    let block = Block::default().borders(Borders::ALL).title("Packet Inspector");
    let inner_area = block.inner(area_right);
    frame.render_widget(block, area_right);

    if let Some(packet) = sim.packets.last() {
        let mut lines = Vec::new();
        lines.push(Line::from(vec![Span::raw(format!("Packet ID: {}", packet.message))]));
        lines.push(Line::from(vec![Span::raw(format!("Speed: {:.4}", packet.speed))]));
        lines.push(Line::from(vec![Span::raw("Payload (Quipu):")]));
        lines.push(Line::from(vec![Span::raw("")]));

        // Render Quipu as ASCII
        // Main cord
        lines.push(Line::from(vec![Span::styled("==================================================", Style::default().fg(Color::Yellow))]));

        // Pendants
        let max_knots = packet.payload.pendants.iter().map(|p| p.knots.len()).max().unwrap_or(0);

        // For each level (knot index), print a line
        for k in 0..max_knots {
            let mut spans = Vec::new();
            for pendant in &packet.payload.pendants {
                if k < pendant.knots.len() {
                    let knot = &pendant.knots[k];
                    let symbol = match knot.knot_type {
                        quipu::KnotType::Single => "o",
                        quipu::KnotType::Long(_) => "=",
                        quipu::KnotType::FigureEight => "8",
                        quipu::KnotType::Empty => "|",
                    };
                    spans.push(Span::styled(format!("  {}  ", symbol), Style::default().fg(Color::Cyan)));
                } else {
                    spans.push(Span::raw("  |  "));
                }
            }
            lines.push(Line::from(spans));
        }

        // Draw rest of cords
        let mut spans = Vec::new();
        for _ in &packet.payload.pendants {
             spans.push(Span::raw("  |  "));
        }
        lines.push(Line::from(spans));

        frame.render_widget(Paragraph::new(lines), inner_area);
    } else {
        frame.render_widget(Paragraph::new("No packets in network."), inner_area);
    }
}
