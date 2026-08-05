//! # Hyperbolic Neural Networks (neuro-poincare) 🧠🍩
//!
//! **Parents:** `neuro-sim` + `poincare-disk`
//!
//! ## Concept
//! A Spiking Neural Network (SNN) where the Izhikevich neurons are mapped onto a Poincaré disk (a 2D projection of hyperbolic space).
//!
//! ## Novel Trait
//! Biological firing sequences experience synaptic delays warped by hyperbolic distance constraints. Synapses that span across the disk or towards the boundary experience massive transmission delays, mimicking relativistic spatial warping.
//!
//! ## Phenotype
//! An emergent bio-neural entity embedded in hyperbolic space. As signals propagate outwards toward the boundary, the hyperbolic distance increases exponentially. This causes massive synaptic transmission delays near the edge while the center fires rapidly, creating a temporal warping effect on biological brain waves.
//!
use ::rand::Rng;
use neuro_sim::Network;
use poincare_disk::{hyperbolic_dist, Point};
use std::time::{Duration, Instant};
use tui_shared::{
    crossterm::event::{self, Event, KeyCode},
    ratatui::{
        backend::CrosstermBackend,
        style::Color,
        widgets::{
            canvas::{Canvas, Rectangle},
            Block, Borders,
        },
        Terminal,
    },
    Tui,
};

const NUM_NEURONS: usize = 50;

struct Node {
    pos: Point,
    neuron_id: usize,
}

fn main() -> std::io::Result<()> {
    let mut tui = Tui::init()?;

    run_app(&mut tui.terminal)
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> std::io::Result<()> {
    let mut rng = ::rand::thread_rng();
    let mut network = Network::new();
    let mut nodes = Vec::new();

    // Create neurons and assign random points within the unit disk
    for _ in 0..NUM_NEURONS {
        let r = rng.gen_range(0.0..0.8); // Keep slightly away from edge for stability
        let theta = rng.gen_range(0.0..std::f64::consts::TAU);
        let pos = Point::new(r * theta.cos(), r * theta.sin());
        let neuron_id = network.add_neuron();

        nodes.push(Node { pos, neuron_id });
    }

    // Connect them with delays proportional to hyperbolic distance
    for i in 0..NUM_NEURONS {
        for j in 0..NUM_NEURONS {
            if i != j {
                // Sparsity: Only connect 10% of pairs
                if rng.gen_bool(0.1) {
                    let dist = hyperbolic_dist(nodes[i].pos, nodes[j].pos);
                    // Map distance (0 to ~infinity) to a delay (0 to 50 steps)
                    let delay = (dist * 10.0).clamp(0.0, 50.0) as usize;

                    // Excitatory weight
                    let weight = rng.gen_range(5.0..15.0);
                    network.add_synapse_with_delay(
                        nodes[i].neuron_id,
                        nodes[j].neuron_id,
                        weight,
                        delay,
                    );
                }
            }
        }
    }

    let mut inputs = vec![0.0; NUM_NEURONS];
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            let area = f.area();

            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .title("Hyperbolic Neural Network")
                        .borders(Borders::ALL),
                )
                .x_bounds([-1.1, 1.1])
                .y_bounds([-1.1, 1.1])
                .paint(|ctx| {
                    // Draw disk boundary
                    for i in 0..100 {
                        let t1 = (i as f64) * std::f64::consts::TAU / 100.0;
                        let t2 = ((i + 1) as f64) * std::f64::consts::TAU / 100.0;
                        ctx.draw(&tui_shared::ratatui::widgets::canvas::Line {
                            x1: t1.cos(),
                            y1: t1.sin(),
                            x2: t2.cos(),
                            y2: t2.sin(),
                            color: Color::DarkGray,
                        });
                    }

                    // Draw synapses (just visually connecting them)
                    for syn in &network.synapses {
                        let n1 = &nodes[syn.from];
                        let n2 = &nodes[syn.to];

                        ctx.draw(&tui_shared::ratatui::widgets::canvas::Line {
                            x1: n1.pos.re,
                            y1: n1.pos.im,
                            x2: n2.pos.re,
                            y2: n2.pos.im,
                            color: Color::DarkGray,
                        });
                    }

                    // Draw neurons
                    for node in &nodes {
                        let is_spiking = network.is_spiking(node.neuron_id);
                        let color = if is_spiking {
                            Color::Yellow
                        } else {
                            Color::Blue
                        };

                        ctx.draw(&Rectangle {
                            x: node.pos.re - 0.02,
                            y: node.pos.im - 0.02,
                            width: 0.04,
                            height: 0.04,
                            color,
                        });
                    }
                });

            f.render_widget(canvas, area);
        })?;

        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
                if let KeyCode::Char(' ') = key.code {
                    // Inject random stimulus
                    for i in 0..NUM_NEURONS {
                        if rng.gen_bool(0.1) {
                            inputs[i] += 50.0;
                        }
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            // Apply slight random noise
            for i in 0..NUM_NEURONS {
                if rng.gen_bool(0.01) {
                    inputs[i] += rng.gen_range(10.0..30.0);
                }
            }

            network.step(&inputs);
            // Reset inputs
            for input in inputs.iter_mut() {
                *input = 0.0;
            }
            last_tick = Instant::now();
        }
    }
}
