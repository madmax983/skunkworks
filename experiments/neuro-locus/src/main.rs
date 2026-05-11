//! 🧬 Splice: Cross neuro-sim × locus
//!
//! **Concept**: Topological Neural Morphogenesis.
//!
//! **Lineage**:
//! - Parent A (neuro-sim): Biological Izhikevich Spiking Neural Network.
//! - Parent B (locus): 2D Grid Topology and spatial wrapping mechanics.
//!
//! **Novel trait**: Spiking neurons are arranged in a 2D grid. Synapses are connected locally (e.g., adjacent neighbors) but are constrained by the `Topology` enum (Torus, Klein Bottle, etc.). This allows neural waves to wrap seamlessly around the boundary.
//!
//! **Predicted Phenotype**: An emergent "Topological Brain". Neuronal waves will propagate across the screen, wrapping around boundaries, forming continuous bio-rhythms dictated entirely by the non-Euclidean constraints of the grid.

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use locus::Topology;
use neuro_sim::Network;
use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct NeuroLocusApp {
    network: Network,
    width: usize,
    height: usize,
    topology: Topology,
    inputs: Vec<f32>,
    time: f64,
}

impl NeuroLocusApp {
    fn new(width: usize, height: usize, topology: Topology) -> Self {
        let mut network = Network::new();
        let mut rng = rand::thread_rng();

        let num_neurons = width * height;
        for _ in 0..num_neurons {
            network.add_neuron();
        }

        // Connect neurons based on topology
        for y in 0..height {
            for x in 0..width {
                let current_idx = y * width + x;

                // Connect to local neighbors in a radius
                for dy in -2i64..=2 {
                    for dx in -2i64..=2 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let target_y = y as i64 + dy;
                        let target_x = x as i64 + dx;

                        if let Some((ny, nx)) =
                            topology.normalize(target_y, target_x, width, height)
                        {
                            let neighbor_idx = ny as usize * width + nx as usize;

                            // Probabilistic synaptic connection
                            if rng.gen_bool(0.3) {
                                // Excitatory or Inhibitory
                                let weight = if rng.gen_bool(0.8) {
                                    rng.gen_range(5.0..15.0) // Excitatory
                                } else {
                                    rng.gen_range(-15.0..-5.0) // Inhibitory
                                };
                                let delay = rng.gen_range(1..5);
                                network.add_synapse_with_delay(
                                    current_idx,
                                    neighbor_idx,
                                    weight,
                                    delay,
                                );
                            }
                        }
                    }
                }
            }
        }

        Self {
            network,
            width,
            height,
            topology,
            inputs: vec![0.0; num_neurons],
            time: 0.0,
        }
    }

    fn tick(&mut self) {
        self.time += 0.05;
        let mut rng = rand::thread_rng();

        // Background noise to keep it alive
        for i in 0..self.width * self.height {
            if rng.gen_bool(0.005) {
                self.inputs[i] += rng.gen_range(10.0..20.0);
            }
        }

        // Apply a concentrated stimulus that moves in a circle
        let cx = (self.width as f64 / 2.0) + (self.time.cos() * 10.0);
        let cy = (self.height as f64 / 2.0) + (self.time.sin() * 5.0);

        if let Some((ny, nx)) = self.topology.normalize(
            cy.round() as i64,
            cx.round() as i64,
            self.width,
            self.height,
        ) {
            let center_idx = ny as usize * self.width + nx as usize;
            self.inputs[center_idx] += 30.0;
        }

        self.network.step(&self.inputs);
        self.inputs.fill(0.0);
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let res = run_app(&mut tui);
    tui.exit()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app(tui: &mut Tui) -> Result<()> {
    let width = 80;
    let height = 40;

    let topologies = [
        Topology::Plane,
        Topology::Torus,
        Topology::Klein,
        Topology::CylinderH,
    ];
    let mut topo_idx = 1; // Start with Torus

    let mut app = NeuroLocusApp::new(width, height, topologies[topo_idx]);

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(50);

    loop {
        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.area());

            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!(" Neuro-Locus | Topology: {:?} ", app.topology)),
                )
                .x_bounds([0.0, app.width as f64])
                .y_bounds([0.0, app.height as f64])
                .paint(|ctx| {
                    for y in 0..app.height {
                        for x in 0..app.width {
                            let idx = y * app.width + x;

                            // Visualization: Use membrane voltage and spiking status
                            if app.network.is_spiking(idx) {
                                ctx.print(
                                    x as f64,
                                    (app.height - 1 - y) as f64,
                                    Span::styled("█", Style::default().fg(Color::Yellow)),
                                );
                            } else {
                                let v = app.network.neurons[idx].v;
                                if v > -40.0 {
                                    ctx.print(
                                        x as f64,
                                        (app.height - 1 - y) as f64,
                                        Span::styled("▓", Style::default().fg(Color::DarkGray)),
                                    );
                                } else if v > -60.0 {
                                    ctx.print(
                                        x as f64,
                                        (app.height - 1 - y) as f64,
                                        Span::styled("▒", Style::default().fg(Color::DarkGray)),
                                    );
                                }
                            }
                        }
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let stats = Paragraph::new("Use [Space] to switch Topology | [Q] to quit")
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(stats, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char(' ') => {
                            topo_idx = (topo_idx + 1) % topologies.len();
                            // Rebuild network with new topology
                            app = NeuroLocusApp::new(width, height, topologies[topo_idx]);
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }
    }
}
