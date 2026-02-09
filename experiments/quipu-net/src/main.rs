use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{
        canvas::{Canvas, Line as CanvasLine, Points},
        Block, Borders, Paragraph,
    },
};

mod network;
mod quipu;
mod serializer;

use network::Simulation;
use quipu::{KnotType, Pendant, Quipu};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut sim = Simulation::new();
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
            // Update simulation
            sim.update();
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(frame: &mut Frame, sim: &Simulation) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(frame.size());

    // Top Pane: Network Graph
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Quipu Network (P2P via Knots)"))
        .marker(Marker::Braille)
        .x_bounds([0.0, 1.0])
        .y_bounds([0.0, 1.0])
        .paint(|ctx| {
            // Draw Edges
            for edge in &sim.edges {
                if let (Some(n1), Some(n2)) = (sim.nodes.get(edge.source), sim.nodes.get(edge.target)) {
                    ctx.draw(&CanvasLine {
                        x1: n1.x,
                        y1: 1.0 - n1.y,
                        x2: n2.x,
                        y2: 1.0 - n2.y,
                        color: Color::DarkGray,
                    });
                }
            }

            // Draw Nodes
            for node in &sim.nodes {
                ctx.draw(&Points {
                    coords: &[(node.x, 1.0 - node.y)],
                    color: Color::Cyan,
                });
            }

            // Draw Packets
            for packet in &sim.packets {
                // Interpolate position
                if let (Some(n1), Some(n2)) = (sim.nodes.get(packet.source), sim.nodes.get(packet.dest)) {
                    let x = n1.x + (n2.x - n1.x) * packet.progress;
                    let y = n1.y + (n2.y - n1.y) * packet.progress;

                    // Color based on speed (slower = red/complex, faster = green/simple)
                    let color = if packet.speed < 0.01 {
                        Color::Red
                    } else if packet.speed < 0.03 {
                        Color::Yellow
                    } else {
                        Color::Green
                    };

                    ctx.draw(&Points {
                        coords: &[(x, 1.0 - y)],
                        color,
                    });
                }
            }
        });
    frame.render_widget(canvas, chunks[0]);

    // Bottom Pane: Quipu Inspector
    // Show details of the last packet
    let packet_info = if let Some(packet) = sim.packets.last() {
        let complexity = (0.05 / packet.speed - 1.0) / 0.05; // Reverse eng speed formula
        let mut text = format!(
            "Packet ID: {} | Speed: {:.4} | Complexity: {:.2}\n",
            packet.id, packet.speed, complexity
        );
        text.push_str("Payload Structure (Quipu):\n");
        text.push_str(&format_quipu(&packet.payload));
        text
    } else {
        "Waiting for packets...".to_string()
    };

    let paragraph = Paragraph::new(packet_info)
        .block(Block::default().borders(Borders::ALL).title("Packet Inspector"))
        .wrap(ratatui::widgets::Wrap { trim: false });

    frame.render_widget(paragraph, chunks[1]);
}

fn format_quipu(quipu: &Quipu) -> String {
    let mut s = String::new();
    s.push_str(&format!("Main Cord (Color: {:?})\n", quipu.main_cord_color));
    for (i, pendant) in quipu.pendants.iter().enumerate() {
        s.push_str(&format_pendant(pendant, 1, i));
    }
    s
}

fn format_pendant(pendant: &Pendant, depth: usize, index: usize) -> String {
    let indent = "  ".repeat(depth);
    let mut s = String::new();

    // Visualize knots
    let knots_str: Vec<String> = pendant.knots.iter().map(|k| match k.knot_type {
        KnotType::Single => "o".to_string(),
        KnotType::Long(t) => format!("L({})", t),
        KnotType::FigureEight => "8".to_string(),
        KnotType::Empty => "_".to_string(),
    }).collect();

    s.push_str(&format!("{}|-- Pendant {}: [ {} ] (Color: {:?})\n", indent, index, knots_str.join("-"), pendant.color));

    for (j, sub) in pendant.subsidiaries.iter().enumerate() {
        s.push_str(&format_pendant(sub, depth + 1, j));
    }
    s
}
