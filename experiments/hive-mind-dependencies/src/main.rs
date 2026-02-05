mod graph;
mod ant;
mod simulation;

use graph::DependencyGraph;
use simulation::Simulation;
use rand::prelude::*;
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
        Block, Borders,
    },
    symbols::Marker,
};

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
    // Create a larger graph: 8 layers, 6 nodes per layer
    dep_graph.generate_layered_dag(8, 6, &mut rng);
    dep_graph.calculate_layout();
    let mut sim = Simulation::new(dep_graph, 200, &mut rng);

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
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(frame: &mut Frame, sim: &Simulation) {
    let area = frame.area();

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Hive Mind Dependencies - 'q' to quit"))
        .marker(Marker::Braille)
        .x_bounds([0.0, 1.0])
        .y_bounds([0.0, 1.0])
        .paint(|ctx| {
            // Draw Edges
            for edge_idx in sim.graph.graph.edge_indices() {
                if let Some((source, target)) = sim.graph.graph.edge_endpoints(edge_idx) {
                     let n1 = &sim.graph.graph[source];
                     let n2 = &sim.graph.graph[target];

                     let pheromone = *sim.pheromones.get(&edge_idx).unwrap_or(&0.0);
                     let color = if pheromone > 50.0 {
                         Color::Red
                     } else if pheromone > 10.0 {
                         Color::Yellow
                     } else if pheromone > 1.0 {
                         Color::Blue
                     } else {
                         Color::DarkGray
                     };

                     ctx.draw(&CanvasLine {
                         x1: n1.x as f64,
                         y1: 1.0 - n1.y as f64, // Invert Y for canvas
                         x2: n2.x as f64,
                         y2: 1.0 - n2.y as f64,
                         color,
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

            // Draw Ants
            for ant in &sim.ants {
                 let node = &sim.graph.graph[ant.current_node];
                 let color = if ant.carrying_artifact { Color::Green } else { Color::Magenta };
                 ctx.draw(&Points {
                     coords: &[(node.x as f64, 1.0 - node.y as f64)],
                     color,
                 });
            }
        });

    frame.render_widget(canvas, area);
}
