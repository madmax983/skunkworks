mod brain;
mod physics;

use anyhow::Result;
use brain::Network;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use physics::FluidSolver;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Line, Points},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct App {
    network: Network,
    fluid: FluidSolver,
    width: f32,
    height: f32,
    particles_poured: usize,
    spikes_routed: usize,
}

impl App {
    fn new() -> Self {
        Self {
            network: Network::new(vec![3, 4, 3, 2]),
            fluid: FluidSolver::new(),
            width: 100.0,
            height: 100.0,
            particles_poured: 0,
            spikes_routed: 0,
        }
    }

    fn update(&mut self, width: f32, height: f32) {
        // Update Layout if resized
        if (self.width - width).abs() > 1.0 || (self.height - height).abs() > 1.0 {
            self.width = width;
            self.height = height;

            // Recalculate network layout
            // Note: Physics coordinate system matches screen characters?
            // Or a virtual coordinate system?
            // FluidSolver uses float coordinates.
            // Canvas uses user defined bounds.
            // Let's use 0..width, 0..height matching the TUI area.
            self.network.layout(width, height);

            // Sync containers to physics engine
            self.fluid.containers.clear();
            for n in &self.network.neurons {
                self.fluid.add_container(n.id, n.x, n.y, n.w, n.h);
            }
        }

        // Safety check: if containers are empty (first run), sync them
        if self.fluid.containers.is_empty() && !self.network.neurons.is_empty() {
             self.network.layout(width, height);
             self.fluid.containers.clear();
             for n in &self.network.neurons {
                self.fluid.add_container(n.id, n.x, n.y, n.w, n.h);
            }
        }

        let dt = 0.2; // Time step

        // 1. Update Physics
        let drained = self.fluid.update(dt);

        // 2. Route Drained Particles (Spikes)
        for (neuron_id, _x) in drained {
            if let Some(target_id) = self.network.get_target_for_spike(neuron_id) {
                // Route to next layer
                // We need to mutate network to spawn spike
                // But we can't mutate network while holding reference to it?
                // get_target returns value (usize), not ref. Safe.
                self.network.spawn_spike(neuron_id, target_id);
                self.spikes_routed += 1;
            } else {
                // End of line (Output layer).
                // Maybe count it or recycle it?
            }
        }

        // 3. Update Traveling Spikes
        let arrivals = self.network.update(dt);

        // 4. Spawn arriving particles
        for (_target_id, x, y) in arrivals {
            // Find container ID for target? It's just target_id (neurons are 1:1 with containers)
            self.fluid.add_particle(x, y, _target_id);
        }
    }

    fn pour(&mut self, input_idx: usize) {
        // Pour into the Nth neuron of the first layer
        if let Some(neuron) = self.network.neurons.iter().find(|n| n.layer == 0 && n.index_in_layer == input_idx) {
            // Spawn a cluster of particles
            let center_x = neuron.x + neuron.w / 2.0;
            let center_y = neuron.y + 1.0;

            for i in 0..5 {
                let offset_x = (i as f32 - 2.0) * 0.5;
                self.fluid.add_particle(center_x + offset_x, center_y, neuron.id);
            }
            self.particles_poured += 5;
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    // Update dimensions
    // We use Canvas coordinates, so let's set them to match the Rect size roughly
    app.width = chunks[0].width as f32;
    app.height = chunks[0].height as f32 * 2.0; // *2 for vertical resolution (half-blocks usually, or just scaling)

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(" Hydro-Brain 🧠💧 "))
        .x_bounds([0.0, app.width as f64])
        .y_bounds([app.height as f64, 0.0]) // Inverted Y
        .paint(|ctx| {
            // Draw Synapses (Lines)
            // We draw these first so they are background
            for s in &app.network.synapses {
                let src = &app.network.neurons[s.source];
                let tgt = &app.network.neurons[s.target];

                let x1 = (src.x + src.w / 2.0) as f64;
                let y1 = (src.y + src.h) as f64; // Bottom of src
                let x2 = (tgt.x + tgt.w / 2.0) as f64;
                let y2 = tgt.y as f64; // Top of tgt

                ctx.draw(&Line {
                    x1, y1, x2, y2,
                    color: Color::DarkGray,
                });
            }

            // Draw Traveling Spikes
            for spike in &app.network.traveling_spikes {
                 ctx.print(spike.x as f64, spike.y as f64, ratatui::text::Span::styled("•", Style::default().fg(Color::Yellow)));
            }

            // Draw Neurons (Containers)
            for n in &app.network.neurons {
                // Draw Box
                let color = if n.layer == 0 { Color::Green }
                           else if n.layer == app.network.layers.len() - 1 { Color::Red }
                           else { Color::White };

                // Top line
                /*
                ctx.draw(&Line {
                    x1: n.x as f64, y1: n.y as f64,
                    x2: (n.x + n.w) as f64, y2: n.y as f64,
                    color,
                });
                */
                // Bottom line
                ctx.draw(&Line {
                    x1: n.x as f64, y1: (n.y + n.h) as f64,
                    x2: (n.x + n.w) as f64, y2: (n.y + n.h) as f64,
                    color,
                });
                // Left line
                ctx.draw(&Line {
                    x1: n.x as f64, y1: n.y as f64,
                    x2: n.x as f64, y2: (n.y + n.h) as f64,
                    color,
                });
                // Right line
                ctx.draw(&Line {
                    x1: (n.x + n.w) as f64, y1: n.y as f64,
                    x2: (n.x + n.w) as f64, y2: (n.y + n.h) as f64,
                    color,
                });
            }

            // Draw Fluid
             let fluid_points: Vec<(f64, f64)> = app
                .fluid
                .particles
                .iter()
                .map(|p| (p.x as f64, p.y as f64))
                .collect();

            ctx.draw(&Points {
                coords: &fluid_points,
                color: Color::Cyan,
            });
        });

    f.render_widget(canvas, chunks[0]);

    let stats = Paragraph::new(format!(
        "Particles: {} | Spikes: {} | Controls: [1-3] Pour | [Q] Quit",
        app.fluid.particles.len(),
        app.spikes_routed
    ))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(stats, chunks[1]);
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();
    let tick_rate = Duration::from_millis(33);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char('1') => app.pour(0),
                        KeyCode::Char('2') => app.pour(1),
                        KeyCode::Char('3') => app.pour(2),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            // We update inside the draw loop or here?
            // App::update needs width/height, which we only really know from Frame.
            // But we can store it in App.
            // For now, update is called in UI.
            // Better practice: update physics here using stored width/height.
            // But width/height are initialized to 100.0, so fine.
             app.update(app.width, app.height);
            last_tick = Instant::now();
        }
    }

    Ok(())
}
