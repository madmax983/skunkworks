use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use glam::Vec3;
use neuro_sim::Network;
use physics_pbd::{Constraint, PbdSystem};
use ratatui::{
    layout::{Constraint as LayoutConstraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::{Canvas, Line}, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct NeuroPhysicsApp {
    network: Network,
    physics: PbdSystem,
    motor_neurons: Vec<usize>,
    muscle_constraints: Vec<usize>, // indices into physics.constraints
}

impl NeuroPhysicsApp {
    fn new() -> Self {
        let mut network = Network::new();
        let mut physics = PbdSystem::new();
        let mut motor_neurons = Vec::new();
        let mut muscle_constraints = Vec::new();

        // 1. Setup Spiking Neural Network (CPG - Central Pattern Generator)
        let n1 = network.add_neuron(); // Oscillator 1
        let n2 = network.add_neuron(); // Oscillator 2
        let m1 = network.add_neuron(); // Motor Neuron 1
        let m2 = network.add_neuron(); // Motor Neuron 2

        // Mutually inhibitory oscillators for alternating rhythm
        network.add_synapse_with_delay(n1, n2, -20.0, 5);
        network.add_synapse_with_delay(n2, n1, -20.0, 5);

        // Connect oscillators to motor neurons
        network.add_synapse_with_delay(n1, m1, 30.0, 1);
        network.add_synapse_with_delay(n2, m2, 30.0, 1);

        motor_neurons.push(m1);
        motor_neurons.push(m2);

        // 2. Setup Physical Body (A simple articulated worm/limb)
        let p0 = physics.add_particle(Vec3::new(-4.0, 0.0, 0.0), 1.0);
        let p1 = physics.add_particle(Vec3::new(0.0, 0.0, 0.0), 1.0);
        let p2 = physics.add_particle(Vec3::new(4.0, 0.0, 0.0), 1.0);

        // Top structural nodes
        let t1 = physics.add_particle(Vec3::new(-2.0, 2.0, 0.0), 1.0);
        let t2 = physics.add_particle(Vec3::new(2.0, 2.0, 0.0), 1.0);

        // Structural integrity (rigid distance constraints)
        let _ = physics.add_distance_constraint(p0, p1, 4.0);
        let _ = physics.add_distance_constraint(p1, p2, 4.0);
        let _ = physics.add_distance_constraint(p0, t1, 2.82); // diagonal
        let _ = physics.add_distance_constraint(p1, t1, 2.82);
        let _ = physics.add_distance_constraint(p1, t2, 2.82);
        let _ = physics.add_distance_constraint(p2, t2, 2.82);
        let _ = physics.add_distance_constraint(t1, t2, 4.0);

        // Muscles (Actuator constraints)
        // Muscle 1: p0 to p1 bottom
        physics.constraints.push(Constraint::Actuator {
            p1: p0,
            p2: p1,
            min_len: 2.0,
            max_len: 4.0,
            factor: 1.0,
            stiffness: 0.5,
        });
        muscle_constraints.push(physics.constraints.len() - 1);

        // Muscle 2: p1 to p2 bottom
        physics.constraints.push(Constraint::Actuator {
            p1: p1,
            p2: p2,
            min_len: 2.0,
            max_len: 4.0,
            factor: 1.0,
            stiffness: 0.5,
        });
        muscle_constraints.push(physics.constraints.len() - 1);

        // Pin the tail so it doesn't just float away
        let _ = physics.add_pin_constraint(p0, Vec3::new(-4.0, 0.0, 0.0));

        Self {
            network,
            physics,
            motor_neurons,
            muscle_constraints,
        }
    }

    fn tick(&mut self) {
        // Provide continuous background current to the oscillators to keep them firing
        let mut external_inputs = vec![0.0; 4];
        external_inputs[0] = 15.0; // Input to n1
        external_inputs[1] = 14.5; // Input to n2 (slightly different to prevent perfect lockstep)

        // Step neural network
        self.network.step(&external_inputs);

        // Map neural spikes to physical muscle contraction
        for (i, &motor_idx) in self.motor_neurons.iter().enumerate() {
            let spiking = self.network.is_spiking(motor_idx);
            let constraint_idx = self.muscle_constraints[i];

            if let Constraint::Actuator { factor, .. } = &mut self.physics.constraints[constraint_idx] {
                if spiking {
                    // Contract muscle
                    *factor = (*factor - 0.2).max(0.0);
                } else {
                    // Relax muscle back to resting state
                    *factor = (*factor + 0.05).min(1.0);
                }
            }
        }

        // Add some gravity and ground collision
        for particle in &mut self.physics.particles {
            particle.vel.y -= 0.01; // Gravity
            if particle.pos.y < -5.0 {
                particle.pos.y = -5.0;
                particle.vel.y *= -0.5; // Bounce
                particle.vel.x *= 0.9; // Friction
            }
        }

        // Step physics simulation
        self.physics.step(0.1, 5);
    }

    fn draw(&self, frame: &mut ratatui::Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([LayoutConstraint::Min(0), LayoutConstraint::Length(3)].as_ref())
            .split(frame.area());

        let title = " 🧬 Splice: neuro-sim × physics-pbd | Neural Musculoskeletal Physics ";

        let canvas = Canvas::default()
            .block(Block::default().title(title).borders(Borders::ALL))
            .paint(|ctx| {
                // Draw constraints (bones and muscles)
                for constraint in &self.physics.constraints {
                    match constraint {
                        Constraint::Distance { p1, p2, .. } => {
                            let pos1 = self.physics.particles[*p1].pos;
                            let pos2 = self.physics.particles[*p2].pos;
                            ctx.draw(&Line {
                                x1: pos1.x as f64,
                                y1: pos1.y as f64,
                                x2: pos2.x as f64,
                                y2: pos2.y as f64,
                                color: Color::DarkGray,
                            });
                        }
                        Constraint::Actuator { p1, p2, factor, .. } => {
                            let pos1 = self.physics.particles[*p1].pos;
                            let pos2 = self.physics.particles[*p2].pos;

                            // Color red when contracted, white when relaxed
                            let color = if *factor < 0.5 { Color::Red } else { Color::White };

                            ctx.draw(&Line {
                                x1: pos1.x as f64,
                                y1: pos1.y as f64,
                                x2: pos2.x as f64,
                                y2: pos2.y as f64,
                                color,
                            });
                        }
                        _ => {}
                    }
                }

                // Draw particles
                for particle in &self.physics.particles {
                    ctx.print(
                        particle.pos.x as f64,
                        particle.pos.y as f64,
                        ratatui::text::Span::styled("O", Style::default().fg(Color::Cyan)),
                    );
                }
            })
            .x_bounds([-10.0, 10.0])
            .y_bounds([-10.0, 10.0]);

        frame.render_widget(canvas, chunks[0]);

        let m1_spike = self.network.is_spiking(self.motor_neurons[0]);
        let m2_spike = self.network.is_spiking(self.motor_neurons[1]);

        let stats = Paragraph::new(format!(
            "Motor 1 Firing: {} | Motor 2 Firing: {} | [Q] Quit",
            if m1_spike { "██" } else { "--" },
            if m2_spike { "██" } else { "--" },
        ))
        .block(Block::default().borders(Borders::ALL));
        frame.render_widget(stats, chunks[1]);
    }
}

fn main() -> Result<()> {
    // Implement standard headless guard for workspace tests
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Headless mode: exiting early to prevent X11 panics or execution timeouts.");
        return Ok(());
    }

    let mut tui = Tui::init()?;
    let mut app = NeuroPhysicsApp::new();

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(50);

    loop {
        tui.terminal.draw(|f| app.draw(f))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
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

    tui.exit()?;
    Ok(())
}
