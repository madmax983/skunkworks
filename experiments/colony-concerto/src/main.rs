mod audio;
mod graph;
mod ant;

use std::sync::Arc;
use std::time::{Duration, Instant};
use crossbeam::channel::unbounded;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{
        canvas::{Canvas, Line as CanvasLine, Points},
        Block, Borders,
    },
    symbols::Marker,
};
use rand::prelude::*;

use crate::audio::AudioEngine;
use crate::graph::{DepGraph, generate_layered_dag};
use crate::ant::Ant;

fn main() -> anyhow::Result<()> {
    // Setup TUI
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Init Logic
    let mut rng = rand::thread_rng();
    let graph = generate_layered_dag(8, 6, &mut rng); // 8 layers, 6 nodes each
    let graph = Arc::new(graph);

    let (audio_tx, audio_rx) = unbounded();
    let _audio_engine = AudioEngine::new(audio_rx)?;

    // Spawn Ants
    let num_ants = 10;
    // Start ants at random root nodes
    let roots: Vec<_> = graph.node_indices()
        .filter(|&i| graph[i].layer == 0)
        .collect();

    for i in 0..num_ants {
        let start_node = roots[rng.gen_range(0..roots.len())];
        let ant = Ant::new(i, graph.clone(), audio_tx.clone(), start_node);
        ant.spawn(); // Consumes ant and starts thread
    }

    // Main Loop
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &graph))?;

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
            last_tick = Instant::now();
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(frame: &mut Frame, graph: &Arc<DepGraph>) {
    let area = frame.area();

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Colony Concerto - 'q' to quit"))
        .marker(Marker::Braille)
        .x_bounds([0.0, 1.0])
        .y_bounds([0.0, 1.0])
        .paint(|ctx| {
            // Draw Edges first
            for edge_idx in graph.edge_indices() {
                if let Some((source, target)) = graph.edge_endpoints(edge_idx) {
                    // Access node positions
                    // We don't lock nodes just to read position (x, y are public fields)
                    // But we access via graph indexing which is safe
                    let n1 = &graph[source];
                    let n2 = &graph[target];

                    ctx.draw(&CanvasLine {
                        x1: n1.x as f64,
                        y1: 1.0 - n1.y as f64, // Invert Y
                        x2: n2.x as f64,
                        y2: 1.0 - n2.y as f64,
                        color: Color::DarkGray,
                    });
                }
            }

            // Draw Nodes
            for node_idx in graph.node_indices() {
                let node = &graph[node_idx];

                // Try to read state
                let color = if let Ok(state) = node.state.try_lock() {
                    if state.builder_id.is_some() {
                        // Being built
                        if state.progress > 0.5 {
                            Color::Green
                        } else {
                            Color::Yellow
                        }
                    } else {
                        // Idle
                        Color::White
                    }
                } else {
                    // Locked (Contention!)
                    Color::Red
                };

                ctx.draw(&Points {
                    coords: &[(node.x as f64, 1.0 - node.y as f64)],
                    color,
                });
            }
        });

    frame.render_widget(canvas, area);
}
