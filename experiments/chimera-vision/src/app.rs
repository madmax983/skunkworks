use crate::agent::Agent;
use crate::eye::Eye;
use crate::physics::PendulumSystem;
use chimera_lang::prelude::*;
use glam::Vec2;
use rand::Rng;

#[derive(PartialEq, Clone, Copy)]
pub enum Mode {
    Stabilize, // Gaze calms chaos
    Excite,    // Gaze excites chaos
}

pub struct App {
    pub system: PendulumSystem,
    pub eye: Eye,
    pub agents: Vec<Agent>,
    pub mode: Mode,
    pub width: f32,
    pub height: f32,
    pub grid_width: usize,
    pub grid_height: usize,
    pub chaos_grid: Vec<f32>, // Cache for rendering
}

impl App {
    pub fn new(width: f32, height: f32) -> Self {
        let mut system = PendulumSystem::new();

        // Create a fabric
        let rows = 15;
        let cols = 20;
        let spacing_x = width / (cols as f32 + 2.0);
        let spacing_y = height / (rows as f32 + 2.0);
        let start_x = spacing_x;
        let start_y = spacing_y;

        let mut node_indices = vec![vec![0; cols]; rows];

        for y in 0..rows {
            for x in 0..cols {
                let pos = Vec2::new(
                    start_x + x as f32 * spacing_x,
                    start_y + y as f32 * spacing_y,
                );
                let fixed = y == 0; // Fix top row
                let mass = if fixed { 0.0 } else { 1.0 };
                let idx = system.add_node(pos, mass, fixed, format!("Node {}-{}", x, y));
                node_indices[y][x] = idx;
            }
        }

        // Add links
        for y in 0..rows {
            for x in 0..cols {
                let idx = node_indices[y][x];
                if x + 1 < cols {
                    let right = node_indices[y][x + 1];
                    let dist = system.nodes[idx].pos.distance(system.nodes[right].pos);
                    system.add_link(idx, right, dist);
                }
                if y + 1 < rows {
                    let down = node_indices[y + 1][x];
                    let dist = system.nodes[idx].pos.distance(system.nodes[down].pos);
                    system.add_link(idx, down, dist);
                }
            }
        }

        let mut agents = Vec::new();
        // Spawn initial population
        for _ in 0..20 {
            let pos = Vec2::new(width / 2.0, height / 2.0);
            // Simple DNA: Move around
            let genes = vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
                Gene { op: OpCode::Photosynthesize, args: vec![] }, // Eat
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] }, // Chance to jump back
                Gene { op: OpCode::Brz, args: vec![Nucleotide::Number(0)] },
            ];
            let dna = Dna { helix: Helix { strands: vec![Strand { genes }] } };
            agents.push(Agent::new(dna, pos));
        }

        Self {
            system,
            eye: Eye::new(width, height),
            agents,
            mode: Mode::Stabilize,
            width,
            height,
            grid_width: width as usize,
            grid_height: height as usize,
            chaos_grid: vec![0.0; (width as usize) * (height as usize)],
        }
    }

    pub fn on_tick(&mut self) {
        let dt = 0.16;
        let mut rng = rand::thread_rng();

        // 1. Physics Step
        self.system.step(dt);

        // 2. Gaze Effect on Physics
        for node in &mut self.system.nodes {
            if node.fixed { continue; }
            if self.eye.in_fovea(node.pos.x, node.pos.y) {
                match self.mode {
                    Mode::Stabilize => {
                        let v = node.pos - node.prev_pos;
                        node.prev_pos = node.pos - v * 0.90; // Damp
                    }
                    Mode::Excite => {
                        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                        let kick = Vec2::new(angle.cos(), angle.sin()) * 0.5;
                        node.prev_pos -= kick;
                    }
                }
            }
        }

        // 3. Update Chaos Grid (for Eye tracking and Agent sensing)
        self.chaos_grid.fill(0.0);
        for node in &self.system.nodes {
            if node.fixed { continue; }
            let velocity = node.pos - node.prev_pos;
            let ke = velocity.length_squared() * 1000.0;

            // Map to grid (simple point splat)
            let gx = node.pos.x as usize;
            let gy = node.pos.y as usize;
            if gx < self.grid_width && gy < self.grid_height {
                self.chaos_grid[gy * self.grid_width + gx] = ke;
            }
        }

        // 4. Eye Update
        self.eye.update(&self.chaos_grid, self.grid_width);

        // 5. Agent Update
        let w = self.width;
        let h = self.height;
        for agent in &mut self.agents {
            let chaos = self.system.get_chaos_level(agent.pos);
            let in_fovea = self.eye.in_fovea(agent.pos.x, agent.pos.y);
            agent.update(dt, chaos, in_fovea, w, h);
        }

        // 6. Population Control
        self.agents.retain(|a| !a.is_dead());

        // Spawn immigrants if low pop
        if self.agents.len() < 10 {
            let pos = Vec2::new(
                rng.gen_range(0.0..self.width),
                rng.gen_range(0.0..self.height),
            );
            // New random DNA
             let genes = vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
                Gene { op: OpCode::Photosynthesize, args: vec![] },
                Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
            ];
            let dna = Dna { helix: Helix { strands: vec![Strand { genes }] } };
            self.agents.push(Agent::new(dna, pos));
        }
    }

    pub fn on_mouse(&mut self, x: u16, y: u16) {
        // TUI coordinates (y is down) match our physics (y is down) usually?
        // But in `gaze-attractor`, ui flipped Y.
        // If I render Y=0 at Top, then mouse Y is correct.
        self.eye.set_target(x as f32, y as f32);
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            Mode::Stabilize => Mode::Excite,
            Mode::Excite => Mode::Stabilize,
        };
    }

    pub fn resize(&mut self, w: f32, h: f32) {
        self.width = w;
        self.height = h;
        self.grid_width = w as usize;
        self.grid_height = h as usize;
        self.eye.width = w;
        self.eye.height = h;
        self.chaos_grid = vec![0.0; (w as usize) * (h as usize)];
    }
}
