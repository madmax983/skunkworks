use anyhow::Result;
use chimera_lang::prelude::*;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use rand::Rng;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{
        canvas::{Canvas, Line},
        Block, Borders, Paragraph,
    },
    Frame, Terminal,
};
use std::io::Stdout;
use std::time::{Duration, Instant};
use tui_shared::Tui;

// --- Lattice & Camera Logic (Adapted from lattice-brain) ---

#[derive(Debug, Clone, Copy, PartialEq)]
enum LatticeType {
    SimpleCubic,
    BodyCenteredCubic,
    FaceCenteredCubic,
}

struct Lattice3D {
    points: Vec<[f64; 3]>,
}

impl Lattice3D {
    fn new(l_type: LatticeType, size: usize) -> Self {
        let mut points = Vec::new();
        let range = size as i32;

        for x in -range..=range {
            for y in -range..=range {
                for z in -range..=range {
                    let fx = x as f64;
                    let fy = y as f64;
                    let fz = z as f64;

                    match l_type {
                        LatticeType::SimpleCubic => {
                            points.push([fx, fy, fz]);
                        }
                        LatticeType::BodyCenteredCubic => {
                            points.push([fx, fy, fz]);
                            // Center point
                            if x < range && y < range && z < range {
                                points.push([fx + 0.5, fy + 0.5, fz + 0.5]);
                            }
                        }
                        LatticeType::FaceCenteredCubic => {
                            points.push([fx, fy, fz]);
                            // Face centers
                            if x < range && y < range {
                                points.push([fx + 0.5, fy + 0.5, fz]);
                            }
                            if x < range && z < range {
                                points.push([fx + 0.5, fy, fz + 0.5]);
                            }
                            if y < range && z < range {
                                points.push([fx, fy + 0.5, fz + 0.5]);
                            }
                        }
                    }
                }
            }
        }
        Self { points }
    }
}

struct Camera {
    angle_x: f64,
    angle_y: f64,
    zoom: f64,
}

impl Camera {
    fn new() -> Self {
        Self {
            angle_x: 0.5,
            angle_y: 0.5,
            zoom: 30.0,
        }
    }

    fn rotate_x(&mut self, delta: f64) {
        self.angle_x += delta;
    }

    fn rotate_y(&mut self, delta: f64) {
        self.angle_y += delta;
    }

    fn project(&self, p: &[f64; 3], width: f64, height: f64) -> Option<(f64, f64, f64)> {
        // Rotate Y
        let x1 = p[0] * self.angle_y.cos() - p[2] * self.angle_y.sin();
        let z1 = p[0] * self.angle_y.sin() + p[2] * self.angle_y.cos();

        // Rotate X
        let y2 = p[1] * self.angle_x.cos() - z1 * self.angle_x.sin();
        let z2 = p[1] * self.angle_x.sin() + z1 * self.angle_x.cos();

        // Perspective
        let scale = self.zoom / (z2 + 10.0); // Simple perspective
        let x_proj = x1 * scale + width / 2.0;
        let y_proj = y2 * scale + height / 2.0; // Inverted Y for terminal

        if z2 > -9.0 {
            Some((x_proj, y_proj, z2))
        } else {
            None
        }
    }
}

// --- Chimera Agent Logic ---

#[derive(Clone, Copy, PartialEq)]
enum AgentState {
    Resting,
    Firing,
    Refractory,
}

struct ChimeraAgent {
    vm: ChimeraVM,
    state: AgentState,
    energy: f64,
}

impl ChimeraAgent {
    fn new(dna: Dna) -> Self {
        Self {
            vm: ChimeraVM::new(dna),
            state: AgentState::Resting,
            energy: 100.0,
        }
    }

    fn step(&mut self, input_signal: f64) {
        // Feed input to VM
        // We push the signal to the stack so the agent can react to it
        // Or we could set a register. Let's push to stack.
        // If stack is full, we might want to clear it or just push.
        // ChimeraVM usually handles stack limits.

        // Push signal
        self.vm.stack.push(Value::Int(input_signal as i64));

        // Step VM
        // ChimeraVM step runs one instruction. We might want to run a few?
        // Let's run 1 step per tick to keep it synchronized.
        self.vm.step();

        // Check output
        // We peek at the top of the stack without popping (if possible)
        // or pop it.
        // ChimeraVM doesn't have a public peek easily accessible, but stack is public.

        let output = if let Some(val) = self.vm.stack.last() {
             match val {
                 Value::Int(n) => *n as f64,
                 _ => 0.0,
             }
        } else {
            0.0
        };

        // Update State
        match self.state {
            AgentState::Resting => {
                if output > 5.0 {
                    self.state = AgentState::Firing;
                    self.energy -= 10.0;
                }
            }
            AgentState::Firing => {
                self.state = AgentState::Refractory;
            }
            AgentState::Refractory => {
                self.state = AgentState::Resting;
            }
        }

        // Metabolism
        self.energy += 0.1; // Passive regen
        if self.energy > 150.0 {
            self.energy = 150.0;
        }
    }
}

// --- Colony Logic ---

struct LatticeColony {
    lattice: Lattice3D,
    agents: Vec<ChimeraAgent>,
    adj: Vec<Vec<usize>>,
    lattice_type: LatticeType,
    size: usize,
}

impl LatticeColony {
    fn new(l_type: LatticeType, size: usize) -> Self {
        let lattice = Lattice3D::new(l_type, size);
        let n = lattice.points.len();
        let mut agents = Vec::with_capacity(n);

        // Generate random DNA for each agent
        // Simple DNA: Listen, if > 5, Yell (Push 10)
        // DNA: [ Input, Push(10), Add, Output ] -> This is naive.
        // Let's give them random DNA from a pool.
        for _ in 0..n {
             let dna = Self::generate_random_dna();
             agents.push(ChimeraAgent::new(dna));
        }

        // Calculate Adjacency (Nearest Neighbors)
        // Brute force is O(N^2), N ~ 1000. Fine for init.
        let mut adj = vec![Vec::new(); n];
        for i in 0..n {
            for j in (i + 1)..n {
                let p1 = lattice.points[i];
                let p2 = lattice.points[j];
                let dist_sq = (p1[0] - p2[0]).powi(2) + (p1[1] - p2[1]).powi(2) + (p1[2] - p2[2]).powi(2);

                // Threshold depends on lattice type.
                // SC: dist = 1.0 (sq=1.0)
                // BCC: dist = sqrt(0.75) (sq=0.75)
                // FCC: dist = sqrt(0.5) (sq=0.5)
                let threshold = match l_type {
                    LatticeType::SimpleCubic => 1.1,
                    LatticeType::BodyCenteredCubic => 0.8,
                    LatticeType::FaceCenteredCubic => 0.6,
                };

                if dist_sq <= threshold {
                    adj[i].push(j);
                    adj[j].push(i);
                }
            }
        }

        Self {
            lattice,
            agents,
            adj,
            lattice_type: l_type,
            size,
        }
    }

    fn generate_random_dna() -> Dna {
        // Create a simple random genome
        let mut genes = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            let (op, args) = match rng.gen_range(0..5) {
                0 => (OpCode::Add, vec![]),
                1 => (OpCode::Sub, vec![]),
                2 => (
                    OpCode::Push,
                    vec![Nucleotide::Number(rng.gen_range(0..10))],
                ),
                3 => (OpCode::Dup, vec![]),
                4 => (OpCode::Drop, vec![]),
                _ => (OpCode::Nop, vec![]),
            };
            genes.push(Gene { op, args });
        }

        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    fn step(&mut self) {
        // 1. Gather Inputs
        let mut inputs = vec![0.0; self.agents.len()];
        for (i, neighbors) in self.adj.iter().enumerate() {
            let mut sum = 0.0;
            for &n_idx in neighbors {
                if self.agents[n_idx].state == AgentState::Firing {
                    sum += 1.0;
                }
            }
            inputs[i] = sum;
        }

        // 2. Step Agents
        for (i, agent) in self.agents.iter_mut().enumerate() {
            agent.step(inputs[i]);
        }
    }
}

// --- App ---

struct App {
    colony: LatticeColony,
    camera: Camera,
    running: bool,
}

impl App {
    fn new() -> Result<Self> {
        let colony = LatticeColony::new(LatticeType::BodyCenteredCubic, 2);
        Ok(Self {
            colony,
            camera: Camera::new(),
            running: true,
        })
    }

    fn change_lattice(&mut self) {
        let (new_type, new_size) = match self.colony.lattice_type {
            LatticeType::SimpleCubic => (LatticeType::BodyCenteredCubic, 2),
            LatticeType::BodyCenteredCubic => (LatticeType::FaceCenteredCubic, 2),
            LatticeType::FaceCenteredCubic => (LatticeType::SimpleCubic, 3), // Larger for SC
        };
        self.colony = LatticeColony::new(new_type, new_size);
    }

    fn run(mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        let tick_rate = Duration::from_millis(50); // Slower tick for VM
        let mut last_tick = Instant::now();

        while self.running {
            terminal.draw(|f| self.ui(f))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    self.handle_input(key);
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.colony.step();
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn handle_input(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.running = false,
            KeyCode::Left => self.camera.rotate_y(-0.1),
            KeyCode::Right => self.camera.rotate_y(0.1),
            KeyCode::Up => self.camera.rotate_x(-0.1),
            KeyCode::Down => self.camera.rotate_x(0.1),
            KeyCode::Char('+') => self.camera.zoom *= 1.1,
            KeyCode::Char('-') => self.camera.zoom /= 1.1,
            KeyCode::Char(' ') => self.change_lattice(),
            KeyCode::Char('i') => {
                // Inject excitement
                let idx = rand::thread_rng().gen_range(0..self.colony.agents.len());
                self.colony.agents[idx].state = AgentState::Firing;
            }
            _ => {}
        }
    }

    fn ui(&self, f: &mut Frame) {
        let area = f.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(area);

        let width = chunks[0].width as f64;
        let height = chunks[0].height as f64;

        // Render Lists
        enum Item {
            Agent {
                x: f64,
                y: f64,
                z: f64,
                state: AgentState,
            },
            Edge {
                x1: f64,
                y1: f64,
                x2: f64,
                y2: f64,
                z: f64,
            },
        }

        let mut items = Vec::new();

        // Project Agents
        for (i, p) in self.colony.lattice.points.iter().enumerate() {
            if let Some((x, y, z)) = self.camera.project(p, width, height) {
                let state = self.colony.agents[i].state;
                items.push((z, Item::Agent { x, y, z, state }));
            }
        }

        // Project Edges
        for (u, neighbors) in self.colony.adj.iter().enumerate() {
            let u_pos = &self.colony.lattice.points[u];
            if let Some((x1, y1, z1)) = self.camera.project(u_pos, width, height) {
                for &v in neighbors {
                    if u < v {
                        let v_pos = &self.colony.lattice.points[v];
                        if let Some((x2, y2, z2)) = self.camera.project(v_pos, width, height) {
                            let avg_z = (z1 + z2) / 2.0;
                            items.push((
                                avg_z,
                                Item::Edge {
                                    x1,
                                    y1,
                                    x2,
                                    y2,
                                    z: avg_z,
                                },
                            ));
                        }
                    }
                }
            }
        }

        items.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        let canvas = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Chimera Lattice: Crystallographic Intelligence"),
            )
            .x_bounds([0.0, width])
            .y_bounds([0.0, height])
            .paint(move |ctx| {
                for (_, item) in &items {
                    match item {
                        Item::Agent { x, y, state, .. } => {
                            let (color, sym) = match state {
                                AgentState::Firing => (Color::Red, "●"),
                                AgentState::Refractory => (Color::Yellow, "○"),
                                AgentState::Resting => (Color::Blue, "."),
                            };
                            ctx.print(*x, *y, Span::styled(sym, Style::default().fg(color)));
                        }
                        Item::Edge {
                            x1,
                            y1,
                            x2,
                            y2,
                            ..
                        } => {
                            ctx.draw(&Line {
                                x1: *x1,
                                y1: *y1,
                                x2: *x2,
                                y2: *y2,
                                color: Color::DarkGray,
                            });
                        }
                    }
                }
            });

        f.render_widget(canvas, chunks[0]);

        let status = format!(
            "Lattice: {:?} | Agents: {} | [Space]: Switch | [I]: Inject | [Arrows]: Move",
            self.colony.lattice_type, self.colony.agents.len()
        );

        f.render_widget(
            Paragraph::new(status).block(Block::default().borders(Borders::ALL)),
            chunks[1],
        );
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let app = App::new();
    match app {
        Ok(app) => {
            if let Err(e) = app.run(&mut tui.terminal) {
                eprintln!("Error: {:?}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to init app: {:?}", e);
        }
    }
    tui.exit()?;
    Ok(())
}
