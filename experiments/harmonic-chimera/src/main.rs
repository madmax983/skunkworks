use std::{error::Error, io, time::Duration};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use nalgebra::Vector2;
use harmonic_engine::physics::PhysicsWorld;
use chimera_lang::prelude::*;
use rand::prelude::*;

const POPULATION_SIZE: usize = 20;
const SIMULATION_STEPS: usize = 300;
const MAX_INTEGRATORS: usize = 5;

struct Agent {
    dna: Dna,
    vm: ChimeraVM,
    world: PhysicsWorld,
    fitness: f32,
    output_trace: Vec<f32>,
}

impl Agent {
    fn new(dna: Dna) -> Self {
        Self {
            dna: dna.clone(),
            vm: ChimeraVM::new(dna),
            world: PhysicsWorld::new(),
            fitness: 0.0,
            output_trace: Vec::new(),
        }
    }

    fn reset(&mut self) {
        self.vm = ChimeraVM::new(self.dna.clone());
        self.world = PhysicsWorld::new();
        self.fitness = 0.0;
        self.output_trace.clear();
    }
}

fn generate_random_dna() -> Dna {
    let mut rng = rand::thread_rng();
    let mut genes = Vec::new();
    for _ in 0..30 {
        let op = match rng.gen_range(0..10) {
            0..=2 => OpCode::Push, // High chance of push
            3 => OpCode::Add,
            4 => OpCode::Sub,
            5 => OpCode::Mul,
            6 => OpCode::GWrite, // Replaces Store
            7 => OpCode::GRead,  // Replaces Load
            8 => OpCode::Jump,
            9 => OpCode::Brz,   // Replaces IfZero
            _ => OpCode::Nop,
        };

        // Push args: Value
        // Jump/Brz args: Strand Index (0)
        let arg = if op == OpCode::Push {
            Nucleotide::Number(rng.gen_range(0..100))
        } else {
            Nucleotide::Number(0)
        };

        genes.push(Gene { op, args: vec![arg] });
    }
    Dna { helix: Helix { strands: vec![Strand { genes }] } }
}

fn build_machine(agent: &mut Agent) {
    // Interpret Grid to build machine
    // Row 0: Integrator Existence & Initial Position X (Col = Index)
    // Row 1: Integrator Initial Velocity (Disk Speed)
    // Row 2: Coupling Target (Value = Source Index)
    // Row 3: Coupling Gain (Value = Gain * 0.1)

    let mut integrators = Vec::new();

    for i in 0..MAX_INTEGRATORS {
        // Check if integrator exists
        let exists_val = agent.vm.grid.get(0).and_then(|r| r.get(i)).unwrap_or(&Value::Int(0));
        let exists = match exists_val {
            Value::Int(n) => *n > 0,
            _ => false,
        };

        if exists {
            let x_val = match agent.vm.grid.get(1).and_then(|r| r.get(i)).unwrap_or(&Value::Int(0)) {
                Value::Int(n) => *n as f32 * 5.0 - 20.0, // Map 0..16 -> Spread
                _ => 0.0,
            };
            let y_pos = 0.0;

            let idx = agent.world.add_integrator(Vector2::new(x_val, y_pos));
            integrators.push(idx);
        }
    }

    // Couplings
    // We iterate through potential targets (integrators we created)
    for (i, &target_idx) in integrators.iter().enumerate() {
        // Read coupling source from Row 2, Col i
        let source_val = agent.vm.grid.get(2).and_then(|r| r.get(i)).unwrap_or(&Value::Int(-1));
        let source_idx_local = match source_val {
            Value::Int(n) => *n as usize,
            _ => 999,
        };

        if source_idx_local < integrators.len() {
             let source_idx = integrators[source_idx_local];
             // Read Gain from Row 3, Col i
             let gain_val = agent.vm.grid.get(3).and_then(|r| r.get(i)).unwrap_or(&Value::Int(5));
             let gain = match gain_val {
                 Value::Int(n) => *n as f32 * 0.1,
                 _ => 0.5,
             };

             agent.world.add_coupling(source_idx, target_idx, gain);
        }
    }
}

fn run_simulation(agent: &mut Agent) {
    // Run VM to build machine
    // Pre-populate stack with some constants to help
    // e.g. [0, 1, 2, 3] to help with addressing
    // But ChimeraVM starts empty.

    for _ in 0..100 {
        agent.vm.step();
    }
    build_machine(agent);

    // Run Physics
    if agent.world.integrators.is_empty() {
        agent.fitness = -100.0;
        return;
    }

    let mut total_movement = 0.0;
    let mut total_velocity = 0.0;
    let mut steps_alive = 0;
    let mut last_angle = 0.0;
    let mut _direction_changes = 0;

    for _ in 0..SIMULATION_STEPS {
        agent.world.step();
        steps_alive += 1;

        // Trace the first integrator's output angle
        if let Some(first_int) = agent.world.integrators.first() {
            if let Some(cyl) = agent.world.rigid_body_set.get(first_int.output_handle) {
                let angle = cyl.rotation().angle();
                let velocity = cyl.angvel();
                agent.output_trace.push(angle);
                total_movement += velocity.abs();
                total_velocity += velocity;

                if (velocity > 0.0 && last_angle < angle) || (velocity < 0.0 && last_angle > angle) {
                     // Moving consistently
                } else {
                     // Direction change (maybe)
                }

                // Penalize explosions
                if velocity.abs() > 50.0 {
                    break;
                }
                last_angle = angle;
            }
        }
    }

    // Fitness:
    // We want oscillation.
    // Oscillation means moving a lot (total_movement high) but staying near 0 average velocity (total_velocity low).

    if steps_alive < SIMULATION_STEPS {
        agent.fitness = steps_alive as f32; // Died early
    } else {
        // High movement, low net displacement (oscillation)
        agent.fitness = total_movement - total_velocity.abs() * 2.0;
        if agent.fitness < 0.0 { agent.fitness = 0.0; }
    }
}

fn mutate(dna: &Dna) -> Dna {
    let mut new_dna = dna.clone();
    let mut rng = rand::thread_rng();

    // Mutate existing genes
    if let Some(strand) = new_dna.helix.strands.get_mut(0) {
        if rng.gen_bool(0.3) {
            let idx = rng.gen_range(0..strand.genes.len());
            strand.genes[idx].args[0] = Nucleotide::Number(rng.gen_range(0..16));
        }
        if rng.gen_bool(0.1) {
            let idx = rng.gen_range(0..strand.genes.len());
            // Change OpCode logic roughly
             let op = match rng.gen_range(0..10) {
                0..=2 => OpCode::Push,
                3 => OpCode::Add,
                4 => OpCode::Sub,
                5 => OpCode::Mul,
                6 => OpCode::GWrite,
                7 => OpCode::GRead,
                8 => OpCode::Jump,
                9 => OpCode::Brz,
                _ => OpCode::Nop,
            };
            strand.genes[idx].op = op;
        }
    }
    new_dna
}

fn main() -> Result<(), Box<dyn Error>> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Init Population
    let mut population: Vec<Agent> = (0..POPULATION_SIZE)
        .map(|_| Agent::new(generate_random_dna()))
        .collect();

    let mut generation = 0;
    let mut best_agent_idx = 0;

    loop {
        // Evaluation
        for agent in &mut population {
            agent.reset();
            run_simulation(agent);
        }

        // Sort by fitness desc
        population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap_or(std::cmp::Ordering::Equal));
        best_agent_idx = 0; // Always top after sort

        // Draw Loop (interactive)
        loop {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                    .split(f.area());

                // Left: Physics View
                let best_agent = &population[best_agent_idx];
                let canvas = ratatui::widgets::canvas::Canvas::default()
                    .block(Block::default().borders(Borders::ALL).title(format!("Gen {} | Best Fitness: {:.1}", generation, best_agent.fitness)))
                    .paint(|ctx| {
                        // Draw integrators
                        for integrator in &best_agent.world.integrators {
                             if let Some(disk) = best_agent.world.rigid_body_set.get(integrator.disk_handle) {
                                 let pos = disk.translation();
                                 ctx.draw(&ratatui::widgets::canvas::Circle {
                                     x: pos.x as f64,
                                     y: pos.y as f64,
                                     radius: 5.0,
                                     color: Color::White,
                                 });
                             }
                             if let Some(cyl) = best_agent.world.rigid_body_set.get(integrator.output_handle) {
                                 let pos = cyl.translation();
                                 let rot = cyl.rotation().angle();
                                 ctx.draw(&ratatui::widgets::canvas::Line {
                                     x1: pos.x as f64,
                                     y1: pos.y as f64,
                                     x2: (pos.x + 8.0 * rot.cos()) as f64,
                                     y2: (pos.y + 8.0 * rot.sin()) as f64,
                                     color: Color::Yellow,
                                 });
                             }
                        }

                        // Trace
                        for (i, &angle) in best_agent.output_trace.iter().enumerate() {
                             let x = (i as f64 / SIMULATION_STEPS as f64) * 80.0 - 40.0;
                             let y = angle as f64 * 5.0;
                             ctx.draw(&ratatui::widgets::canvas::Points {
                                 coords: &[(x, y)],
                                 color: Color::Cyan,
                             });
                        }
                    })
                    .x_bounds([-40.0, 40.0])
                    .y_bounds([-30.0, 30.0]);
                f.render_widget(canvas, chunks[0]);

                // Right: Genome & Stats
                let genome_text: String = best_agent.dna.helix.strands.first()
                    .map(|s| s.genes.iter().map(|g| format!("{:?} {:?}", g.op, g.args)).collect::<Vec<_>>().join("\n"))
                    .unwrap_or_default();

                let stats = Paragraph::new(format!("Genome:\n{}\n\nStats:\nIntegrators: {}\nCouplings: {}",
                    genome_text,
                    best_agent.world.integrators.len(),
                    best_agent.world.couplings.len()
                ))
                .block(Block::default().borders(Borders::ALL).title("DNA & Stats"));
                f.render_widget(stats, chunks[1]);
            })?;

            // Input
            if event::poll(Duration::from_millis(16))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => {
                            // Restore Terminal
                            disable_raw_mode()?;
                            execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
                            terminal.show_cursor()?;
                            return Ok(());
                        }
                        KeyCode::Char(' ') => {
                            break; // Next generation
                        }
                        _ => {}
                    }
                }
            }
        }

        // Evolution (Elitism + Mutation)
        let elite_count = 2;
        let mut new_population = Vec::new();
        // Elitism
        for i in 0..elite_count {
            new_population.push(Agent::new(population[i].dna.clone()));
        }
        // Mutation
        while new_population.len() < POPULATION_SIZE {
            let parent_idx = rand::thread_rng().gen_range(0..elite_count); // Pick from elites
            let child_dna = mutate(&population[parent_idx].dna);
            new_population.push(Agent::new(child_dna));
        }
        population = new_population;
        generation += 1;
    }
}
