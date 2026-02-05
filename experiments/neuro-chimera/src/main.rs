use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use rand::Rng;
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols::Marker,
    text::Span,
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph, Sparkline,
    },
    Frame, Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};
use tui_shared::Tui;

use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::{ChimeraVM, Value};

const POPULATION_SIZE: usize = 20;
const TICKS_PER_NEURON: usize = 50;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();
    let res = run_app(&mut tui.terminal, &mut app);

    if let Err(err) = res {
        tui.exit()?;
        println!("{:?}", err);
    }
    Ok(())
}

#[derive(Clone)]
struct BioNeuron {
    vm: ChimeraVM,
    genome: Dna,
}

impl BioNeuron {
    fn new_random() -> Self {
        let genome = generate_random_dna();
        Self {
            vm: ChimeraVM::new(genome.clone()),
            genome,
        }
    }

    fn run(&mut self, inputs: &[f64]) -> f64 {
        // Reset VM state but keep DNA
        self.vm = ChimeraVM::new(self.genome.clone());

        // Input encoding: Write scaled inputs to Grid (0,0), (0,1), etc.
        for (i, &val) in inputs.iter().enumerate() {
            if i < 16 {
                // Scale f64 (-1.0 to 1.0) to i64 (-100 to 100)
                let int_val = (val * 100.0) as i64;
                self.vm.grid[0][i] = Value::Int(int_val);
            }
        }

        // Run for fixed ticks
        for _ in 0..TICKS_PER_NEURON {
            if self.vm.halted {
                break;
            }
            self.vm.step();
        }

        // Output decoding: Pop from stack, or read Grid(0,0)
        // Prefer Stack.
        if let Some(val) = self.vm.stack.pop() {
            match val {
                Value::Int(n) => (n as f64) / 100.0,
                Value::Str(s) => (s.len() as f64) / 10.0, // Arbitrary mapping
            }
        } else {
            // Fallback: Read Grid(0,0) if stack empty
            match &self.vm.grid[0][0] {
                Value::Int(n) => (*n as f64) / 100.0,
                _ => 0.0,
            }
        }
    }
}

#[derive(Clone)]
struct BioNetwork {
    // Layers of Neurons
    layers: Vec<Vec<BioNeuron>>,
}

impl BioNetwork {
    fn new(topology: Vec<usize>) -> Self {
        let mut layers = Vec::new();
        // Skip input layer size in terms of neurons, as inputs are fed to first hidden layer neurons?
        // Standard NN: Layer 0 is Input (no neurons, just values). Layer 1 is Hidden.
        // Here, let's say topology=[2, 5, 4, 1] means:
        // Input has 2 values.
        // Layer 1 has 5 neurons. Each takes 2 inputs.
        // Layer 2 has 4 neurons. Each takes 5 inputs.
        // Layer 3 has 1 neuron. Takes 4 inputs.

        for i in 1..topology.len() {
            let num_neurons = topology[i];
            let mut layer = Vec::new();
            for _ in 0..num_neurons {
                layer.push(BioNeuron::new_random());
            }
            layers.push(layer);
        }

        Self { layers }
    }

    fn forward(&mut self, inputs: &[f64]) -> Vec<f64> {
        let mut current_inputs = inputs.to_vec();

        for layer in &mut self.layers {
            let mut next_inputs = Vec::new();
            for neuron in layer {
                // Each neuron receives ALL inputs from previous layer (Dense)
                // In Chimera, we put them on the grid.
                let out = neuron.run(&current_inputs);
                next_inputs.push(out.tanh()); // Activation function to keep stable? Or let VM handle it?
                                              // Let's use tanh to normalize between layers so values don't explode
            }
            current_inputs = next_inputs;
        }

        current_inputs
    }
}

fn generate_random_dna() -> Dna {
    let mut rng = rand::thread_rng();
    let num_genes = rng.gen_range(5..20);
    let mut genes = Vec::new();

    let ops = [
        OpCode::Push,
        OpCode::Add,
        OpCode::Sub,
        OpCode::Mul,
        OpCode::Div,
        OpCode::Dup,
        OpCode::Swap,
        OpCode::GRead,
        OpCode::GWrite,
        OpCode::Brz,
        OpCode::Jump, // Flow control
    ];

    for _ in 0..num_genes {
        let op = ops[rng.gen_range(0..ops.len())].clone();
        let args = match op {
            OpCode::Push => vec![Nucleotide::Number(rng.gen_range(-50..50))],
            OpCode::Jump | OpCode::Brz => vec![Nucleotide::Number(rng.gen_range(0..5))], // Short jumps
            _ => vec![],
        };
        genes.push(Gene { op, args });
    }

    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

struct App {
    population: Vec<BioNetwork>,
    best_network: Option<BioNetwork>,
    generation: usize,
    loss_history: Vec<u64>,
    inputs: Vec<Vec<f64>>,
    targets: Vec<Vec<f64>>,
    paused: bool,
}

impl App {
    fn new() -> Self {
        // Circle Problem Data
        let mut inputs = vec![];
        let mut targets = vec![];
        let mut rng = rand::thread_rng();

        for _ in 0..50 {
            let x: f64 = rng.gen_range(-1.0..1.0);
            let y: f64 = rng.gen_range(-1.0..1.0);
            inputs.push(vec![x, y]);
            let dist = x.hypot(y);
            targets.push(vec![if dist < 0.6 { 1.0 } else { 0.0 }]);
        }

        let topology = vec![2, 4, 1]; // Smaller topology for speed
        let mut population = Vec::with_capacity(POPULATION_SIZE);
        for _ in 0..POPULATION_SIZE {
            population.push(BioNetwork::new(topology.clone()));
        }

        Self {
            population,
            best_network: None,
            generation: 0,
            loss_history: Vec::new(),
            inputs,
            targets,
            paused: false,
        }
    }

    fn evolve(&mut self) {
        if self.paused {
            return;
        }

        let mut fitnesses: Vec<(usize, f64)> = Vec::new();

        // Evaluate Fitness (MSE)
        for (i, network) in self.population.iter_mut().enumerate() {
            let mut error_sum = 0.0;
            for (in_vec, tgt_vec) in self.inputs.iter().zip(self.targets.iter()) {
                let output = network.forward(in_vec);
                let diff = output[0] - tgt_vec[0];
                error_sum += diff * diff;
            }
            let mse = error_sum / self.inputs.len() as f64;
            fitnesses.push((i, mse));
        }

        // Sort by error (lower is better)
        fitnesses.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let best_idx = fitnesses[0].0;
        self.best_network = Some(self.population[best_idx].clone());
        self.loss_history.push((fitnesses[0].1 * 1000.0) as u64);
        if self.loss_history.len() > 100 {
            self.loss_history.remove(0);
        }

        // Selection & Reproduction
        let mut new_pop = Vec::new();
        let mut rng = rand::thread_rng();

        // Elitism: Keep top 2
        new_pop.push(self.population[fitnesses[0].0].clone());
        new_pop.push(self.population[fitnesses[1].0].clone());

        while new_pop.len() < POPULATION_SIZE {
            // Tournament selection
            let p1 = &self.population[tournament(&fitnesses)];
            let p2 = &self.population[tournament(&fitnesses)];

            let mut child = p1.clone();

            // Crossover: Swap neurons (genomes) between parents
            for (l_idx, layer) in child.layers.iter_mut().enumerate() {
                for (n_idx, neuron) in layer.iter_mut().enumerate() {
                    if rng.gen_bool(0.5) {
                        neuron.genome = p2.layers[l_idx][n_idx].genome.clone();
                    }

                    // Mutation
                    if rng.gen_bool(0.1) {
                        mutate_genome(&mut neuron.genome);
                    }
                }
            }
            new_pop.push(child);
        }

        self.population = new_pop;
        self.generation += 1;
    }
}

fn tournament(fitnesses: &[(usize, f64)]) -> usize {
    let mut rng = rand::thread_rng();
    let idx1 = rng.gen_range(0..fitnesses.len());
    let idx2 = rng.gen_range(0..fitnesses.len());
    if fitnesses[idx1].1 < fitnesses[idx2].1 {
        fitnesses[idx1].0
    } else {
        fitnesses[idx2].0
    }
}

fn mutate_genome(dna: &mut Dna) {
    let mut rng = rand::thread_rng();
    if dna.helix.strands.is_empty() {
        return;
    }
    let strand = &mut dna.helix.strands[0]; // Assume single strand for simplicity

    if rng.gen_bool(0.5) && !strand.genes.is_empty() {
        // Point mutation
        let idx = rng.gen_range(0..strand.genes.len());
        if rng.gen_bool(0.5) {
            // Change Op
            let ops = [
                OpCode::Push,
                OpCode::Add,
                OpCode::Sub,
                OpCode::Dup,
                OpCode::Drop,
                OpCode::Swap,
            ];
            strand.genes[idx].op = ops[rng.gen_range(0..ops.len())].clone();
        } else if !strand.genes[idx].args.is_empty() {
            // Change Arg
            strand.genes[idx].args[0] = Nucleotide::Number(rng.gen_range(-20..20));
        }
    } else {
        // Structural mutation (Insert/Delete)
        if rng.gen_bool(0.5) && !strand.genes.is_empty() {
            // Delete
            let idx = rng.gen_range(0..strand.genes.len());
            strand.genes.remove(idx);
        } else {
            // Insert
            let op = OpCode::Push;
            let args = vec![Nucleotide::Number(rng.gen_range(-10..10))];
            strand.genes.push(Gene { op, args });
        }
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('p') => app.paused = !app.paused,
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.evolve();
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(f.area());

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(vertical_chunks[1]);

    draw_header(f, app, vertical_chunks[0]);
    draw_decision_boundary(f, app, body_chunks[0]);
    draw_network(f, app, body_chunks[1]);
    draw_footer(f, app, vertical_chunks[2]);
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(area);

    let title = Paragraph::new(Span::styled(
        " NEURO-CHIMERA 🧬🧠 ",
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    ))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green)),
    )
    .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let stats_block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Loss (Gen: {}) ", app.generation));
    let sparkline = Sparkline::default()
        .block(stats_block)
        .data(&app.loss_history)
        .style(Style::default().fg(Color::Magenta));
    f.render_widget(sparkline, chunks[1]);

    let status = if app.paused { "PAUSED" } else { "EVOLVING" };
    f.render_widget(
        Paragraph::new(status)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center),
        chunks[2],
    );
}

fn draw_decision_boundary(f: &mut Frame, app: &App, area: Rect) {
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Phenotype Boundary "),
        )
        .marker(Marker::Block)
        .x_bounds([-1.0, 1.0])
        .y_bounds([-1.0, 1.0])
        .paint(|ctx| {
            if let Some(net) = &app.best_network {
                // Background (Decision Boundary)
                // Use lower resolution for performance
                for x_i in 0..30 {
                    for y_i in 0..15 {
                        let x = -1.0 + x_i as f64 * 2.0 / 29.0;
                        let y = -1.0 + y_i as f64 * 2.0 / 14.0;

                        // We need a mutable clone to run forward :(
                        // This is expensive in render loop.
                        // Optimization: cache grid? Or just accept lag.
                        let mut net_clone = net.clone();
                        let out = net_clone.forward(&[x, y]);

                        if out[0] > 0.5 {
                            ctx.draw(&Points {
                                coords: &[(x, y)],
                                color: Color::Cyan,
                            });
                        }
                    }
                }
            }

            // Data Points
            for (i, input) in app.inputs.iter().enumerate() {
                let color = if app.targets[i][0] > 0.5 {
                    Color::Green
                } else {
                    Color::Red
                };
                ctx.print(
                    input[0],
                    input[1],
                    Span::styled("●", Style::default().fg(color)),
                );
            }
        });
    f.render_widget(canvas, area);
}

fn draw_network(f: &mut Frame, app: &App, area: Rect) {
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Neural Architecture "),
        )
        .marker(Marker::Braille)
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0])
        .paint(|ctx| {
            if let Some(net) = &app.best_network {
                let layer_count = net.layers.len() + 1; // +1 for input layer
                let x_step = 100.0 / (layer_count as f64 + 1.0);

                // Draw Input Layer
                let y_step_in = 100.0 / (3.0); // 2 inputs
                for i in 0..2 {
                    let x = x_step;
                    let y = y_step_in * (i as f64 + 1.0);
                    ctx.print(x, y, Span::styled("I", Style::default().fg(Color::Yellow)));
                }

                // Draw Hidden/Output Layers
                for (l_idx, layer) in net.layers.iter().enumerate() {
                    let x = x_step * (l_idx as f64 + 2.0);
                    let y_step = 100.0 / (layer.len() as f64 + 1.0);

                    for (n_idx, _neuron) in layer.iter().enumerate() {
                        let y = y_step * (n_idx as f64 + 1.0);
                        ctx.print(x, y, Span::styled("N", Style::default().fg(Color::Green)));

                        // Draw connection lines roughly
                        // (Omitted for simplicity, just nodes)
                    }
                }
            }
        });
    f.render_widget(canvas, area);
}

fn draw_footer(f: &mut Frame, _app: &App, area: Rect) {
    let text = "Q: Quit | P: Pause/Resume";
    f.render_widget(Paragraph::new(text).alignment(Alignment::Center), area);
}
