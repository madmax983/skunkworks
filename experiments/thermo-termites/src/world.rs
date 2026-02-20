use rand::prelude::*;
use rayon::prelude::*;

pub const WIDTH: usize = 512;
pub const HEIGHT: usize = 512;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Material {
    Empty,
    Wall,
    Server,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AgentKind {
    Termite,
    Air,
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub material: Material,
    pub heat: f32,
    pub pheromone: f32,
    // Fluid Dynamics Fields (accumulated from particles)
    pub air_density: f32,
    pub air_vx: f32,
    pub air_vy: f32,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            material: Material::Empty,
            heat: 0.0,
            pheromone: 0.0,
            air_density: 0.0,
            air_vx: 0.0,
            air_vy: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heat_diffusion_simple() {
        let mut world = World::new();
        // Add a heat source
        world.grid[50 * WIDTH + 50].heat = 100.0;

        // Run update
        world.update_grid_physics();

        // Neighbors should heat up
        let neighbor_heat = world.grid[50 * WIDTH + 51].heat;
        assert!(neighbor_heat > 0.0);
    }

    #[test]
    fn test_agent_movement() {
        let mut world = World::new();
        world.agents.push(Agent::new_air(10.0, 10.0));
        let x_start = world.agents[0].x;

        world.update_agents();

        let x_end = world.agents[0].x;
        assert!(x_start != x_end);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Agent {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub kind: AgentKind,
    pub carrying: bool, // For Termite
    pub heat: f32,      // For Air
}

impl Agent {
    pub fn new_termite(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            kind: AgentKind::Termite,
            carrying: false,
            heat: 0.0,
        }
    }

    pub fn new_air(x: f32, y: f32) -> Self {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        Self {
            x,
            y,
            vx: angle.cos(),
            vy: angle.sin(),
            kind: AgentKind::Air,
            carrying: false,
            heat: 0.0, // Starts cold
        }
    }
}

pub struct World {
    pub grid: Vec<Cell>,
    pub agents: Vec<Agent>,
    pub step: u64,
    // Double buffers for physics (Reuse memory)
    current_heats: Vec<f32>,
    current_pheros: Vec<f32>,
}

impl World {
    pub fn new() -> Self {
        let grid = vec![Cell::default(); WIDTH * HEIGHT];
        Self {
            grid,
            agents: Vec::new(),
            step: 0,
            current_heats: vec![0.0; WIDTH * HEIGHT],
            current_pheros: vec![0.0; WIDTH * HEIGHT],
        }
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        y * WIDTH + x
    }

    pub fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.grid[self.get_index(x, y)]
    }

    pub fn get_cell_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        let idx = self.get_index(x, y);
        &mut self.grid[idx]
    }

    pub fn add_server_block(&mut self, x: usize, y: usize, w: usize, h: usize) {
        for dy in 0..h {
            for dx in 0..w {
                if x + dx < WIDTH && y + dy < HEIGHT {
                    let cell = self.get_cell_mut(x + dx, y + dy);
                    cell.material = Material::Server;
                    cell.heat = 100.0;
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.update_grid_physics();
        self.update_agents();
        self.step += 1;
    }

    pub fn get_average_server_temp(&self) -> f32 {
        let mut total_heat = 0.0;
        let mut count = 0;
        for cell in &self.grid {
            if matches!(cell.material, Material::Server) {
                total_heat += cell.heat;
                count += 1;
            }
        }
        if count == 0 {
            0.0
        } else {
            total_heat / count as f32
        }
    }

    fn update_grid_physics(&mut self) {
        let width = WIDTH;
        let height = HEIGHT;

        // 1. Copy current state to buffers (Sequential copy is fast enough, ~1ms)
        // Avoiding allocation in loop
        for (i, cell) in self.grid.iter().enumerate() {
            self.current_heats[i] = cell.heat;
            self.current_pheros[i] = cell.pheromone;
        }

        // Split borrows for closure
        let current_heats = &self.current_heats;
        let current_pheros = &self.current_pheros;
        let grid = &mut self.grid;

        // Parallel update
        grid.par_iter_mut().enumerate().for_each(|(i, cell)| {
            let x = i % width;
            let y = i / width;

            // Boundary check
            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                cell.heat *= 0.99; // Boundary cooling
                cell.pheromone *= 0.99;
                return;
            }

            // Physics Constants
            let diffusion = 0.10; // Reduced diffusion to make convection more important
            let wall_insulation = 0.05;
            let cooling = 0.001; // Global cooling
            let evap = 0.02; // Pheromone evaporation
            let pheromone_diffusion = 0.15;
            let heat_gen = 5.0;
            let max_heat = 1000.0;

            // --- Heat Diffusion ---
            // Servers generate heat but also diffuse it
            let heat_source = if matches!(cell.material, Material::Server) {
                heat_gen
            } else {
                0.0
            };

            let top = current_heats[i - width];
            let bottom = current_heats[i + width];
            let left = current_heats[i - 1];
            let right = current_heats[i + 1];

            let avg = (top + bottom + left + right) * 0.25;
            let diff = avg - current_heats[i];

            let diff_rate = if matches!(cell.material, Material::Wall) {
                diffusion * wall_insulation
            } else {
                diffusion
            };

            // Apply diffusion and source
            cell.heat = (current_heats[i] + heat_source + diff * diff_rate) * (1.0 - cooling);
            cell.heat = cell.heat.clamp(0.0, max_heat);

            // --- Pheromone Diffusion ---
            let top_p = current_pheros[i - width];
            let bottom_p = current_pheros[i + width];
            let left_p = current_pheros[i - 1];
            let right_p = current_pheros[i + 1];

            let avg_p = (top_p + bottom_p + left_p + right_p) * 0.25;
            cell.pheromone = (current_pheros[i]
                + (avg_p - current_pheros[i]) * pheromone_diffusion)
                * (1.0 - evap);
        });
    }

    fn update_agents(&mut self) {
        let width = WIDTH as f32;
        let height = HEIGHT as f32;

        // 1. Reset Grid Fluid Fields
        self.grid.par_iter_mut().for_each(|cell| {
            cell.air_density = 0.0;
            cell.air_vx = 0.0;
            cell.air_vy = 0.0;
        });

        // 2. Sequential Interaction (Scatter & Heat Exchange)
        let mut rng = rand::thread_rng();
        for agent in &mut self.agents {
            // Bounds Check
            agent.x = agent.x.clamp(0.0, width - 1.0);
            agent.y = agent.y.clamp(0.0, height - 1.0);

            let ix = agent.x as usize;
            let iy = agent.y as usize;
            let idx = iy * WIDTH + ix;

            if idx < self.grid.len() {
                let cell = &mut self.grid[idx];

                match agent.kind {
                    AgentKind::Air => {
                        // Scatter Density & Velocity
                        cell.air_density += 1.0;
                        cell.air_vx += agent.vx;
                        cell.air_vy += agent.vy;

                        // Heat Exchange
                        // Use a balanced exchange equation
                        let k = 0.5; // Thermal conductivity constant
                        let transfer = (cell.heat - agent.heat) * k;

                        if matches!(cell.material, Material::Server) {
                            // Server heats up agent actively
                            // Server generates heat in physics step, here it gives it to agents
                            // Assume server maintains high temp, or we just pull from it
                            agent.heat += 5.0;
                            cell.heat -= 1.0; // Cooling the server!
                        } else if matches!(cell.material, Material::Wall) {
                            // Wall interaction
                            // If Agent is hot, deposit Pheromone (Heat Trace)
                            if agent.heat > 40.0 {
                                cell.pheromone = (cell.pheromone + 2.0).min(100.0);
                                agent.heat *= 0.95; // Wall absorbs some heat
                            }
                            // Wall is insulating, small transfer
                            // But wall can heat up agent if wall is hot
                            let wall_transfer = (cell.heat - agent.heat) * 0.05;
                            agent.heat += wall_transfer;
                            cell.heat -= wall_transfer;
                        } else {
                            // Air-Air (Cell is ambient air)
                            agent.heat += transfer;
                            cell.heat -= transfer;
                        }
                    }
                    AgentKind::Termite => {
                        // --- Termite Construction Logic ---
                        let current_mat = self.grid[idx].material;
                        let current_phero = self.grid[idx].pheromone;
                        let flow_speed = (self.grid[idx].air_vx.powi(2) + self.grid[idx].air_vy.powi(2)).sqrt();

                        // Count Wall Neighbors
                        let mut wall_neighbors = 0;
                        let neighbors = [
                            if ix > 0 { Some(idx - 1) } else { None },
                            if ix < WIDTH - 1 { Some(idx + 1) } else { None },
                            if iy > 0 { Some(idx - WIDTH) } else { None },
                            if iy < HEIGHT - 1 { Some(idx + WIDTH) } else { None },
                        ];
                        for n_idx in neighbors.into_iter().flatten() {
                            if matches!(self.grid[n_idx].material, Material::Wall) {
                                wall_neighbors += 1;
                            }
                        }

                        if agent.carrying {
                            // Carrying Dirt -> Look to Build
                            if matches!(current_mat, Material::Empty) {
                                // Rule: Build if Pheromone is High (Heat path) AND Flow is NOT too strong
                                // We want to guide flow, not block it.
                                // Avoid building if Pheromone is TOO high (Stagnation/Encapsulation)

                                let build_prob = if current_phero > 20.0 && current_phero < 85.0 {
                                     // Sweet spot for building fins/chimneys
                                     if flow_speed < 0.5 { 0.1 } else { 0.01 }
                                } else if wall_neighbors > 0 && current_phero < 85.0 {
                                    // Extend existing walls slightly, but not in super hot zones
                                    0.001
                                } else {
                                    0.0
                                };

                                if rng.gen_bool(build_prob) {
                                    self.grid[idx].material = Material::Wall;
                                    agent.carrying = false;
                                }
                            }
                        } else {
                            // Empty -> Look to Pick
                            if matches!(current_mat, Material::Wall) {
                                // Rule: Pick if Flow is blocked
                                // Or if Pheromone is Low (Cold, useless wall)
                                // Or if Pheromone is TOO HIGH (Stagnation - open a vent!)

                                let pick_prob = if current_phero > 90.0 {
                                    // Emergency Venting!
                                    0.1
                                } else if current_phero < 5.0 {
                                    if wall_neighbors <= 1 { 0.5 } else { 0.05 } // Clean up cold/noise
                                } else {
                                    // Moderate Hot Wall - Keep it
                                    0.001
                                };

                                if rng.gen_bool(pick_prob) {
                                    self.grid[idx].material = Material::Empty;
                                    agent.carrying = true;
                                }
                            }
                        }
                    }
                }
            }
        }

        // 3. Parallel Movement Update
        let grid = &self.grid;
        let agents = &mut self.agents;

        agents.par_iter_mut().for_each(|agent| {
            let ix = agent.x as usize;
            let iy = agent.y as usize;
            let idx = iy * WIDTH + ix;

            match agent.kind {
                AgentKind::Air => {
                    // Physics Forces
                    if idx < grid.len() {
                        let cell = &grid[idx];
                        // Buoyancy
                        let ambient_temp = cell.heat.max(10.0);
                        let buoyancy = (agent.heat - ambient_temp) * 0.05; // Even stronger buoyancy
                        agent.vy -= buoyancy;

                        // Pressure/Flow
                        if ix > 0 && ix < WIDTH - 1 && iy > 0 && iy < HEIGHT - 1 {
                            let left = grid[idx - 1].air_density;
                            let right = grid[idx + 1].air_density;
                            let top = grid[idx - WIDTH].air_density;
                            let bottom = grid[idx + WIDTH].air_density;

                            let dx = left - right;
                            let dy = top - bottom;

                            let k_p = 0.1;
                            agent.vx += dx * k_p;
                            agent.vy += dy * k_p;
                        }
                    }

                    // Update Position & Bounce
                    let next_x = agent.x + agent.vx;
                    let next_y = agent.y + agent.vy;
                    let next_ix = next_x.clamp(0.0, width - 1.0) as usize;
                    let next_iy = next_y.clamp(0.0, height - 1.0) as usize;
                    let next_idx = next_iy * WIDTH + next_ix;

                    if next_idx < grid.len() && matches!(grid[next_idx].material, Material::Wall) {
                        agent.vx *= -0.8;
                        agent.vy *= -0.8;
                    } else {
                        agent.x = next_x;
                        agent.y = next_y;
                    }

                    agent.vx *= 0.98;
                    agent.vy *= 0.98;

                     // Bounds
                    if agent.x <= 0.0 || agent.x >= width - 1.0 {
                        agent.vx *= -1.0;
                        agent.x = agent.x.clamp(0.0, width - 1.0);
                    }
                    if agent.y <= 0.0 || agent.y >= height - 1.0 {
                        agent.vy *= -1.0;
                        agent.y = agent.y.clamp(0.0, height - 1.0);
                    }
                }
                AgentKind::Termite => {
                    let mut rng = rand::thread_rng();
                    // Gradient Sensing
                    // Sample random neighbor to see if it's better
                    let sample_angle = rng.gen_range(0.0..std::f32::consts::TAU);
                    let sample_dist = 5.0;
                    let sx = (agent.x + sample_angle.cos() * sample_dist).clamp(0.0, width - 1.0) as usize;
                    let sy = (agent.y + sample_angle.sin() * sample_dist).clamp(0.0, height - 1.0) as usize;
                    let s_idx = sy * WIDTH + sx;

                    // Current Pheromone/Heat
                    let current_phero = grid[idx].pheromone;
                    let target_phero = grid[s_idx].pheromone;

                    // Decision:
                    // If Carrying: Go to Higher Pheromone
                    // If Empty: Go to Lower Pheromone (find cold dirt)
                    let better = if agent.carrying {
                        target_phero > current_phero
                    } else {
                        target_phero < current_phero
                    };

                    if better {
                        // Turn towards sample
                        agent.vx += sample_angle.cos() * 0.1;
                        agent.vy += sample_angle.sin() * 0.1;
                    } else {
                        // Random walk / Turn away
                        agent.vx += rng.gen_range(-0.2..0.2);
                        agent.vy += rng.gen_range(-0.2..0.2);
                    }

                    // Limit speed
                    let speed = (agent.vx.powi(2) + agent.vy.powi(2)).sqrt();
                    if speed > 1.0 {
                        agent.vx /= speed;
                        agent.vy /= speed;
                    }

                    // Move
                    agent.x = (agent.x + agent.vx).clamp(1.0, width - 2.0);
                    agent.y = (agent.y + agent.vy).clamp(1.0, height - 2.0);
                }
            }
        });
    }
}
