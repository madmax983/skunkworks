use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use quipu::Cord;
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
use std::collections::HashMap;
use std::io::Stdout;
use std::time::{Duration, Instant};
use tui_shared::Tui;

// --- Lattice & Camera Logic ---

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

// --- Agent Logic ---

#[derive(Clone, Copy, PartialEq)]
enum AgentState {
    Moving,
    TyingKnot,
}

struct Agent {
    node_idx: usize,
    target_node_idx: Option<usize>,
    state: AgentState,
    progress: f64, // 0.0 to 1.0 along the bond
}

impl Agent {
    fn new(start_node: usize) -> Self {
        Self {
            node_idx: start_node,
            target_node_idx: None,
            state: AgentState::Moving,
            progress: 0.0,
        }
    }
}

// --- Colony Logic ---

struct QuipuLattice {
    lattice: Lattice3D,
    agents: Vec<Agent>,
    // Adjacency list: node_idx -> vec of neighbors
    adj: Vec<Vec<usize>>,
    // Bonds: (min_idx, max_idx) -> Cord
    bonds: HashMap<(usize, usize), Cord>,
    lattice_type: LatticeType,
    // size: usize, // Removed unused field
}

impl QuipuLattice {
    fn new(l_type: LatticeType, size: usize) -> Self {
        let lattice = Lattice3D::new(l_type, size);
        let n = lattice.points.len();

        // Calculate Adjacency
        let mut adj = vec![Vec::new(); n];
        let mut bonds = HashMap::new();

        for i in 0..n {
            for j in (i + 1)..n {
                let p1 = lattice.points[i];
                let p2 = lattice.points[j];
                let dist_sq = (p1[0] - p2[0]).powi(2) + (p1[1] - p2[1]).powi(2) + (p1[2] - p2[2]).powi(2);

                let threshold = match l_type {
                    LatticeType::SimpleCubic => 1.1,
                    LatticeType::BodyCenteredCubic => 0.8,
                    LatticeType::FaceCenteredCubic => 0.6,
                };

                if dist_sq <= threshold {
                    adj[i].push(j);
                    adj[j].push(i);
                    // Initialize empty cord (value 0)
                    bonds.insert((i, j), Cord::from(0));
                }
            }
        }

        // Spawn Agents
        let num_agents = n / 5 + 1;
        let mut agents = Vec::with_capacity(num_agents);
        let mut rng = rand::thread_rng();
        for _ in 0..num_agents {
             agents.push(Agent::new(rng.gen_range(0..n)));
        }

        Self {
            lattice,
            agents,
            adj,
            bonds,
            lattice_type: l_type,
            // size,
        }
    }

    fn step(&mut self) {
        let mut rng = rand::thread_rng();

        for agent in &mut self.agents {
            match agent.state {
                AgentState::Moving => {
                    if let Some(target) = agent.target_node_idx {
                        // Move towards target
                        agent.progress += 0.1;
                        if agent.progress >= 1.0 {
                            // Arrived
                            agent.node_idx = target;
                            agent.target_node_idx = None;
                            agent.progress = 0.0;

                            // Chance to tie a knot
                            if rng.gen_bool(0.3) {
                                agent.state = AgentState::TyingKnot;
                            }
                        }
                    } else {
                        // Pick a random neighbor
                        if let Some(neighbors) = self.adj.get(agent.node_idx) {
                             if !neighbors.is_empty() {
                                 let target = neighbors[rng.gen_range(0..neighbors.len())];
                                 agent.target_node_idx = Some(target);
                                 agent.progress = 0.0;
                             }
                        }
                    }
                }
                AgentState::TyingKnot => {
                    // Tie a knot on a random connected bond
                    if let Some(neighbors) = self.adj.get(agent.node_idx) {
                        if !neighbors.is_empty() {
                            let neighbor = neighbors[rng.gen_range(0..neighbors.len())];
                            let key = if agent.node_idx < neighbor {
                                (agent.node_idx, neighbor)
                            } else {
                                (neighbor, agent.node_idx)
                            };

                            if let Some(cord) = self.bonds.get_mut(&key) {
                                // Add 1 to cord value
                                // This simulates "traffic count" or "interaction"
                                let new_val = cord.value() + 1;
                                // Limit to avoid massive cords
                                if new_val < 999 {
                                     *cord = Cord::from(new_val);
                                } else {
                                     // Reset on overflow to keep it dynamic
                                     *cord = Cord::from(0);
                                }
                            }
                        }
                    }
                    agent.state = AgentState::Moving;
                }
            }
        }
    }
}

// --- App ---

struct App {
    sim: QuipuLattice,
    camera: Camera,
    running: bool,
}

impl App {
    fn new() -> Result<Self> {
        let sim = QuipuLattice::new(LatticeType::BodyCenteredCubic, 2);
        Ok(Self {
            sim,
            camera: Camera::new(),
            running: true,
        })
    }

    fn change_lattice(&mut self) {
        let (new_type, new_size) = match self.sim.lattice_type {
            LatticeType::SimpleCubic => (LatticeType::BodyCenteredCubic, 2),
            LatticeType::BodyCenteredCubic => (LatticeType::FaceCenteredCubic, 2),
            LatticeType::FaceCenteredCubic => (LatticeType::SimpleCubic, 3),
        };
        self.sim = QuipuLattice::new(new_type, new_size);
    }

    fn run(mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        let tick_rate = Duration::from_millis(50);
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
                self.sim.step();
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
            KeyCode::Char('r') => {
                 self.sim = QuipuLattice::new(self.sim.lattice_type, 2); // Reset
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
                state: AgentState,
            },
            Knot {
                x: f64,
                y: f64,
                symbol: String,
            },
            Line {
                x1: f64,
                y1: f64,
                x2: f64,
                y2: f64,
            },
        }

        let mut items = Vec::new();

        // 1. Edges and Knots
        for ((u, v), cord) in &self.sim.bonds {
            let p1 = &self.sim.lattice.points[*u];
            let p2 = &self.sim.lattice.points[*v];

            if let (Some((x1, y1, z1)), Some((x2, y2, z2))) = (
                self.camera.project(p1, width, height),
                self.camera.project(p2, width, height),
            ) {
                let avg_z = (z1 + z2) / 2.0;

                // Draw line
                items.push((
                    avg_z,
                    Item::Line {
                        x1, y1, x2, y2,
                    },
                ));

                // Draw Knots along the line
                // Cord clusters: [Units, Tens, Hundreds...]
                // We map them along the line segment.
                // Total knots?
                let mut all_knots = Vec::new();
                for cluster in &cord.clusters {
                    for knot in cluster {
                        all_knots.push(knot);
                    }
                }

                if !all_knots.is_empty() {
                    let step = 1.0 / (all_knots.len() as f64 + 1.0);
                    for (i, knot) in all_knots.iter().enumerate() {
                        let t = step * (i as f64 + 1.0);
                        let kx = x1 + (x2 - x1) * t;
                        let ky = y1 + (y2 - y1) * t;

                        items.push((
                            avg_z - 0.1, // Slightly in front of line
                            Item::Knot {
                                x: kx,
                                y: ky,
                                symbol: knot.symbol(),
                            },
                        ));
                    }
                }
            }
        }

        // 2. Agents
        for agent in &self.sim.agents {
             let p1 = &self.sim.lattice.points[agent.node_idx];
             // Interpolate if moving
             let pos = if let Some(target) = agent.target_node_idx {
                 let p2 = &self.sim.lattice.points[target];
                 let t = agent.progress;
                 [
                     p1[0] + (p2[0] - p1[0]) * t,
                     p1[1] + (p2[1] - p1[1]) * t,
                     p1[2] + (p2[2] - p1[2]) * t,
                 ]
             } else {
                 *p1
             };

             if let Some((x, y, z)) = self.camera.project(&pos, width, height) {
                 items.push((z - 0.5, Item::Agent { x, y, state: agent.state })); // In front of everything
             }
        }

        // Sort by Z (painter's algorithm)
        // Z decreases into distance (in my camera logic z2 > -9.0 means visible, larger Z is further?)
        // Wait, camera logic:
        // scale = self.zoom / (z2 + 10.0);
        // z2 is camera-space Z. Larger Z means further away.
        // So we sort descending Z (draw far first).
        items.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        let canvas = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Quipu Lattice: 3D Data Structure"),
            )
            .x_bounds([0.0, width])
            .y_bounds([0.0, height])
            .paint(move |ctx| {
                for (_, item) in &items {
                    match item {
                        Item::Agent { x, y, state, .. } => {
                            let (color, sym) = match state {
                                AgentState::TyingKnot => (Color::Red, "☿"), // Mercury/Hermes/Weaver
                                AgentState::Moving => (Color::Green, "@"),
                            };
                            ctx.print(*x, *y, Span::styled(sym, Style::default().fg(color)));
                        }
                        Item::Knot { x, y, symbol, .. } => {
                             ctx.print(*x, *y, Span::styled(symbol.clone(), Style::default().fg(Color::Yellow)));
                        }
                        Item::Line {
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
            "Lattice: {:?} | Agents: {} | [Space]: Switch | [R]: Reset | [Arrows]: Rotate/Zoom",
            self.sim.lattice_type, self.sim.agents.len()
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
