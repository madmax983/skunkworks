use std::env;
use std::time::{Duration, Instant};

use glam::Vec3;
use neuro_sim::Network;
use physics_pbd::{PbdSystem, Constraint};
use rand::{thread_rng, Rng};

use tui_shared::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    ratatui::{
        layout::{Constraint as TuiConstraint, Direction, Layout},
        style::{Color, Style},
        widgets::{canvas::Canvas, Block, Borders, Paragraph},
    },
    Tui,
};

struct NeuroPhysicsApp {
    brain: Network,
    body: PbdSystem,
    node_to_neuron: Vec<usize>,
}

impl NeuroPhysicsApp {
    fn new() -> Self {
        let mut brain = Network::new();
        let mut body = PbdSystem::new();
        let mut node_to_neuron = Vec::new();
        let mut rng = thread_rng();

        let width = 5;
        let height = 5;
        let mut p_indices = vec![];
        let spacing = 20.0;
        let start_x = 30.0;
        let start_y = 30.0;

        for y in 0..height {
            for x in 0..width {
                let is_pinned = y == 0;
                let mass = if is_pinned { 0.0 } else { 1.0 };
                let pos = Vec3::new(start_x + (x as f32) * spacing, start_y + (y as f32) * spacing, 0.0);
                let p_idx = body.add_particle(pos, mass);
                if is_pinned {
                    let _ = body.add_pin_constraint(p_idx, pos);
                }
                p_indices.push(p_idx);

                let n_idx = brain.add_neuron();
                brain.neurons[n_idx].a = 0.02 + rng.gen_range(0.0..0.02);
                brain.neurons[n_idx].b = 0.2;
                brain.neurons[n_idx].c = -65.0;
                brain.neurons[n_idx].d = 8.0;
                node_to_neuron.push(n_idx);
            }
        }

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;

                if x < width - 1 {
                    let right_idx = y * width + (x + 1);
                    let _ = body.add_actuator_constraint(p_indices[idx], p_indices[right_idx], spacing * 0.5, spacing, 1.0);
                    brain.add_synapse(node_to_neuron[idx], node_to_neuron[right_idx], rng.gen_range(5.0..10.0));
                    brain.add_synapse(node_to_neuron[right_idx], node_to_neuron[idx], rng.gen_range(5.0..10.0));
                }
                if y < height - 1 {
                    let down_idx = (y + 1) * width + x;
                    let _ = body.add_actuator_constraint(p_indices[idx], p_indices[down_idx], spacing * 0.5, spacing, 1.0);
                    brain.add_synapse(node_to_neuron[idx], node_to_neuron[down_idx], rng.gen_range(5.0..10.0));
                    brain.add_synapse(node_to_neuron[down_idx], node_to_neuron[idx], rng.gen_range(5.0..10.0));
                }
            }
        }

        Self {
            brain,
            body,
            node_to_neuron,
        }
    }

    fn tick(&mut self) {
        let mut rng = thread_rng();

        let mut inputs = vec![0.0; self.brain.neurons.len()];

        for i in 0..self.brain.neurons.len() {
            if rng.gen_bool(0.02) {
                inputs[i] = 20.0;
            } else {
                let p_idx = i;
                let vel = self.body.particles[p_idx].vel;
                inputs[i] = vel.length() * 0.5;
            }
        }

        self.brain.step(&inputs);

        for c in self.body.constraints.iter_mut() {
            if let Constraint::Actuator { p1, p2, factor, .. } = c {
                let n1 = self.node_to_neuron[*p1];
                let n2 = self.node_to_neuron[*p2];

                if self.brain.spikes[n1] || self.brain.spikes[n2] {
                    *factor = 0.0;
                } else {
                    *factor = (*factor + 0.1).min(1.0);
                }
            }
        }

        for p in &mut self.body.particles {
            if p.inv_mass > 0.0 {
                p.vel.y += 0.5;
            }
        }
        self.body.step(0.016, 5);
    }
}

fn main() -> anyhow::Result<()> {
    if env::args().any(|arg| arg == "--headless") {
        println!("Headless mode: exiting early to prevent X11 panics or execution timeouts.");
        let mut app = NeuroPhysicsApp::new();
        for _ in 0..10 {
            app.tick();
        }
        return Ok(());
    }

    let mut tui = Tui::init()?;
    let res = run_app(&mut tui);
    tui.exit()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app(tui: &mut Tui) -> anyhow::Result<()> {
    let mut app = NeuroPhysicsApp::new();
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(50);

    loop {
        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([TuiConstraint::Min(0), TuiConstraint::Length(3)])
                .split(f.area());

            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Neural Muscle Contraction (neuro-physics) "),
                )
                .x_bounds([0.0, 150.0])
                .y_bounds([0.0, 150.0])
                .paint(|ctx| {
                    for c in &app.body.constraints {
                        if let Constraint::Actuator { p1, p2, factor, .. } = c {
                            let pos1 = app.body.particles[*p1].pos;
                            let pos2 = app.body.particles[*p2].pos;

                            let color = if *factor < 0.5 { Color::Red } else { Color::Blue };

                            ctx.draw(&tui_shared::ratatui::widgets::canvas::Line {
                                x1: pos1.x as f64,
                                y1: (150.0 - pos1.y) as f64,
                                x2: pos2.x as f64,
                                y2: (150.0 - pos2.y) as f64,
                                color,
                            });
                        }
                    }

                    for (i, p) in app.body.particles.iter().enumerate() {
                        let is_spiking = app.brain.spikes[app.node_to_neuron[i]];
                        let color = if is_spiking { Color::Yellow } else { Color::White };
                        let symbol = if is_spiking { "O" } else { "o" };

                        ctx.print(
                            p.pos.x as f64,
                            (150.0 - p.pos.y) as f64,
                            tui_shared::ratatui::text::Span::styled(symbol, Style::default().fg(color))
                        );
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let mut active_spikes = 0;
            for &s in &app.brain.spikes {
                if s { active_spikes += 1; }
            }

            let stats = Paragraph::new(format!(
                "Neurons: {} | Spiking: {} | Muscles (Actuators): {} | [Q] Quit",
                app.brain.neurons.len(), active_spikes, app.body.constraints.len()
            ))
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
