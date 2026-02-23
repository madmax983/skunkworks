#![allow(dead_code, unused_imports)]

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{
        canvas::{Canvas, Line, Rectangle},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::{
    collections::{HashMap, VecDeque},
    time::{Duration, Instant},
};
use tui_shared::Tui;

use neuro_sim::Network;
use quipu::{Cord, Knot};
use rand::Rng;

const CORD_LENGTH: usize = 20; // Visual height in grid units

struct AppState {
    net: Network,
    cords: Vec<Cord>,
    beads: Vec<Bead>,
    synapse_map: HashMap<(usize, usize), usize>, // (CordIdx, ClusterIdx) -> SynapseIdx
    neuron_positions: Vec<(f64, f64)>,           // For rendering layout
    paused: bool,
    tick_rate: Duration,
    last_tick: Instant,
    iteration: u64,
}

#[derive(Clone)]
struct Bead {
    cord_idx: usize,
    y: f64, // 0.0 (Top) to CORD_LENGTH (Bottom)
    color: Color,
}

impl AppState {
    fn new() -> Self {
        let mut net = Network::new();
        let mut cords = Vec::new();
        let mut synapse_map = HashMap::new();
        let mut rng = rand::thread_rng();

        let num_neurons = 10;

        // 1. Create Neurons & Cords
        for _ in 0..num_neurons {
            net.add_neuron();
            // Initialize Cord with empty clusters
            let mut cord = Cord::new();
            // We hijack the clusters vector to represent vertical slots
            // Cluster i corresponds to delay i.
            // We need slots up to CORD_LENGTH
            for _ in 0..CORD_LENGTH {
                cord.clusters.push(Vec::new());
            }
            cords.push(cord);
        }

        // 2. Create Synapses (Wiring)
        // Each neuron connects to 2-3 others
        for i in 0..num_neurons {
            let num_targets = rng.gen_range(2..4);
            for _ in 0..num_targets {
                let target = rng.gen_range(0..num_neurons);
                if i == target {
                    continue;
                }

                // Delay determines Knot Position (Y)
                // We want knots distributed along the cord
                // Delay 1 to CORD_LENGTH-1
                let delay = rng.gen_range(2..CORD_LENGTH);

                // Weight determines Knot Type
                let weight = rng.gen_range(5.0..15.0);
                let knot = if weight < 8.0 {
                    Knot::Simple
                } else if weight < 12.0 {
                    Knot::Long((weight - 5.0) as u8)
                } else {
                    Knot::FigureEight
                };

                // Add to Network (Adjust delay to match visual travel time)
                // Visual Travel: `delay` ticks.
                // Network Delay: `delay` ticks (so it fires on T + delay + 1)
                net.add_synapse_with_delay(i, target, weight, delay);
                let syn_idx = net.synapses.len() - 1;

                // Add Knot to Visual Cord
                // We place it at index `delay`.
                if delay < cords[i].clusters.len() {
                    cords[i].clusters[delay].push(knot);
                    synapse_map.insert((i, delay), syn_idx);
                }
            }
        }

        Self {
            net,
            cords,
            beads: Vec::new(),
            synapse_map,
            neuron_positions: Vec::new(), // Calculated in UI
            paused: false,
            tick_rate: Duration::from_millis(100),
            last_tick: Instant::now(),
            iteration: 0,
        }
    }

    fn update(&mut self) {
        if self.paused {
            return;
        }

        self.iteration += 1;

        // 1. Step Physics
        // Random input to keep it alive if silent
        if self.iteration % 20 == 0 {
            let mut rng = rand::thread_rng();
            let inputs: Vec<f32> = (0..self.net.neurons.len())
                .map(|_| if rng.gen_bool(0.1) { 20.0 } else { 0.0 })
                .collect();
            self.net.step(&inputs);
        } else {
            self.net.step(&[]);
        }

        // 2. Spawn Beads for Spikes
        for (i, &spiked) in self.net.spikes.iter().enumerate() {
            if spiked {
                self.beads.push(Bead {
                    cord_idx: i,
                    y: 0.0,
                    color: Color::Yellow,
                });
            }
        }

        // 3. Move Beads
        let speed = 1.0; // 1 unit per tick
        for bead in &mut self.beads {
            bead.y += speed;
        }

        // 4. Flash Knots
        // If a bead passes a knot position, we can flash it?
        // Logic handled by rendering.

        // 5. Remove Beads that fall off
        self.beads.retain(|b| b.y < CORD_LENGTH as f64 + 2.0);
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut state = AppState::new();

    loop {
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(' ') => state.paused = !state.paused,
                    KeyCode::Char('r') => state = AppState::new(),
                    _ => {}
                }
            }
        }

        let now = Instant::now();
        if now.duration_since(state.last_tick) >= state.tick_rate {
            state.update();
            state.last_tick = now;
        }

        tui.terminal.draw(|f| {
            ui(f, &state);
        })?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_initialization() {
        let state = AppState::new();
        assert_eq!(state.cords.len(), 10);
        assert!(!state.net.neurons.is_empty());
    }
}

fn ui(f: &mut Frame, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    let title = Paragraph::new("🧬 NEURO-QUIPU: A Knotted Mind")
        .style(Style::default().fg(Color::Cyan).bold())
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Cords (Axons)"),
        )
        .x_bounds([0.0, state.cords.len() as f64])
        .y_bounds([0.0, CORD_LENGTH as f64])
        .paint(|ctx| {
            // Invert Y: 0.0 is Top in logic, but Canvas Y=0 is Bottom.
            // So Visual Y = CORD_LENGTH - logic_y.
            let height = CORD_LENGTH as f64;

            for (i, cord) in state.cords.iter().enumerate() {
                let x = i as f64 + 0.5;

                // Draw Cord Line
                ctx.draw(&Line {
                    x1: x,
                    y1: 0.0,
                    x2: x,
                    y2: height,
                    color: Color::DarkGray,
                });

                // Draw Knots
                for (cluster_idx, cluster) in cord.clusters.iter().enumerate() {
                    if cluster.is_empty() {
                        continue;
                    }

                    let y_logic = cluster_idx as f64;
                    let y_visual = height - y_logic;

                    // Draw Knot Blob
                    let color = if state.net.get_synapse_activity(
                        *state.synapse_map.get(&(i, cluster_idx)).unwrap_or(&999),
                    ) {
                        Color::Red // Firing!
                    } else {
                        Color::Gray
                    };

                    let mut radius = 0.1;
                    for k in cluster {
                        match k {
                            Knot::Simple => radius += 0.05,
                            Knot::Long(_) => radius += 0.1,
                            Knot::FigureEight => radius += 0.15,
                        }
                    }

                    ctx.draw(&Rectangle {
                        x: x - radius / 2.0,
                        y: y_visual - 0.1,
                        width: radius,
                        height: 0.2,
                        color,
                    });
                }
            }

            // Draw Beads
            for bead in &state.beads {
                let x = bead.cord_idx as f64 + 0.5;
                let y_visual = height - bead.y;

                if y_visual >= 0.0 && y_visual <= height {
                    ctx.draw(&Rectangle {
                        x: x - 0.1,
                        y: y_visual - 0.1,
                        width: 0.2,
                        height: 0.2,
                        color: bead.color,
                    });
                }
            }
        });
    f.render_widget(canvas, chunks[1]);

    let info = format!(
        "Neurons: {} | Synapses: {} | Spikes: {} | Tick: {}",
        state.net.neurons.len(),
        state.net.synapses.len(),
        state.net.spikes.iter().filter(|&&s| s).count(),
        state.iteration
    );
    let footer = Paragraph::new(info)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}
