use crate::eye::Eye;
use crate::physics::PendulumSystem;
use glam::Vec2;
use rand::Rng;

#[derive(PartialEq, Clone, Copy)]
pub enum Mode {
    Stabilize, // Watcher calms the chaos (damping)
    Excite,    // Watcher adds energy (kicks)
}

pub struct App {
    pub system: PendulumSystem,
    pub eye: Eye,
    pub mode: Mode,
    pub grid_width: usize,
    pub grid_height: usize,
    pub energy_grid: Vec<f32>,
    pub should_quit: bool,
    pub width: f32,
    pub height: f32,
}

impl App {
    pub fn new(width: f32, height: f32) -> Self {
        let mut system = PendulumSystem::new();

        // Create a 10x10 fabric
        let rows = 10;
        let cols = 15;
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

        // Add links (Grid + Diagonals for stiffness)
        for y in 0..rows {
            for x in 0..cols {
                let idx = node_indices[y][x];
                // Right
                if x + 1 < cols {
                    let right = node_indices[y][x + 1];
                    let dist = system.nodes[idx].pos.distance(system.nodes[right].pos);
                    system.add_link(idx, right, dist);
                }
                // Down
                if y + 1 < rows {
                    let down = node_indices[y + 1][x];
                    let dist = system.nodes[idx].pos.distance(system.nodes[down].pos);
                    system.add_link(idx, down, dist);
                }
            }
        }

        let grid_width = width as usize;
        let grid_height = height as usize;
        let energy_grid = vec![0.0; grid_width * grid_height];

        Self {
            system,
            eye: Eye::new(width, height),
            mode: Mode::Stabilize,
            grid_width,
            grid_height,
            energy_grid,
            should_quit: false,
            width,
            height,
        }
    }

    pub fn on_tick(&mut self) {
        let dt = 0.16; // Fixed time step for simulation

        // 1. Step Physics
        self.system.step(dt);

        // 2. Calculate Energy Grid
        // Reset grid
        self.energy_grid.fill(0.0);

        for node in &self.system.nodes {
            if node.fixed {
                continue;
            }

            // Kinetic Energy ~ Velocity^2
            let velocity = node.pos - node.prev_pos;
            let ke = velocity.length_squared() * 1000.0; // Scale up for visibility

            // Map position to grid
            let gx = node.pos.x as i32;
            let gy = node.pos.y as i32;

            if gx >= 0 && gx < self.grid_width as i32 && gy >= 0 && gy < self.grid_height as i32 {
                let idx = (gy as usize) * self.grid_width + (gx as usize);
                self.energy_grid[idx] += ke;

                // Diffuse slightly to neighbors? (Optional, maybe expensive)
            }
        }

        // 3. Update Eye
        self.eye.update(&self.energy_grid, self.grid_width);

        // 4. Apply Forces (The Observer Effect)
        let mut rng = rand::thread_rng();
        let damping = 0.90; // Strong damping when looked at
        let kick_strength = 0.5;

        for node in &mut self.system.nodes {
            if node.fixed {
                continue;
            }

            if self.eye.in_fovea(node.pos.x, node.pos.y) {
                match self.mode {
                    Mode::Stabilize => {
                        // Dampen velocity: move prev_pos closer to current pos
                        // v = pos - prev
                        // new_v = v * damping
                        // prev = pos - new_v
                        let v = node.pos - node.prev_pos;
                        node.prev_pos = node.pos - v * damping;
                    }
                    Mode::Excite => {
                        // Add random energy
                        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                        let kick = Vec2::new(angle.cos(), angle.sin()) * kick_strength;
                        node.prev_pos -= kick; // Verlet kick: subtract from prev to add to next velocity
                    }
                }
            }
        }
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            Mode::Stabilize => Mode::Excite,
            Mode::Excite => Mode::Stabilize,
        };
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.grid_width = width as usize;
        self.grid_height = height as usize;
        self.energy_grid = vec![0.0; self.grid_width * self.grid_height];
        self.eye.width = width;
        self.eye.height = height;
    }
}
