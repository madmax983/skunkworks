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
            let diffusion = 0.20;
            let wall_insulation = 0.05;
            let cooling = 0.001; // Global cooling
            let evap = 0.02; // Pheromone evaporation
            let pheromone_diffusion = 0.15;
            let heat_gen = 5.0;
            let max_heat = 1000.0;

            // --- Heat Diffusion ---
            if matches!(cell.material, Material::Server) {
                cell.heat = (current_heats[i] + heat_gen).min(max_heat);
            } else {
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

                cell.heat = (current_heats[i] + diff * diff_rate) * (1.0 - cooling);
            }

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
        // We do this sequentially to allow safe mutable access to both Grid and Agents
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

                if matches!(agent.kind, AgentKind::Air) {
                    // Scatter Density & Velocity
                    cell.air_density += 1.0;
                    cell.air_vx += agent.vx;
                    cell.air_vy += agent.vy;

                    // Heat Exchange
                    if matches!(cell.material, Material::Server) {
                        agent.heat += 5.0; // Pick up heat
                        cell.heat -= 0.1;
                    } else if matches!(cell.material, Material::Wall) {
                        // Deposit Heat into Wall (Pheromone Trigger)
                        if agent.heat > 50.0 {
                            cell.pheromone = (cell.pheromone + 1.0).min(100.0);
                            agent.heat *= 0.9; // Lose heat to wall
                        }
                        // Bounce logic is handled in movement phase, but we can lose energy here
                        agent.vx *= 0.9;
                        agent.vy *= 0.9;
                    } else {
                        // Exchange with Air Cell
                        // Cell Heat represents "Ambient Temp"
                        let eq_heat = (cell.heat + agent.heat) * 0.5;
                        let transfer = (eq_heat - agent.heat) * 0.1;
                        agent.heat += transfer;
                        // Cell heat also changes, but air mass is small?
                        // Let's say Cell Heat is dominant or equal mass for simplicity
                        cell.heat -= transfer;
                    }
                } else if matches!(agent.kind, AgentKind::Termite) {
                    // --- Termite Construction Logic ---
                    // Read Grid State (Copying values to avoid borrow issues)
                    let current_mat = self.grid[idx].material;
                    let current_phero = self.grid[idx].pheromone;

                    // Check Neighbors (Simple 4-way)
                    let mut wall_neighbors = 0;
                    let neighbors = [
                        if ix > 0 { Some(idx - 1) } else { None },
                        if ix < WIDTH - 1 { Some(idx + 1) } else { None },
                        if iy > 0 { Some(idx - WIDTH) } else { None },
                        if iy < HEIGHT - 1 {
                            Some(idx + WIDTH)
                        } else {
                            None
                        },
                    ];

                    for n_idx in neighbors.into_iter().flatten() {
                        if matches!(self.grid[n_idx].material, Material::Wall) {
                            wall_neighbors += 1;
                        }
                    }

                    if agent.carrying {
                        // Wants to Drop (Build)
                        // Rule: Build if near existing wall (extend) AND Pheromone is High (Active Vent)
                        // OR Randomly drop if very high pheromone (New nucleation)
                        if matches!(current_mat, Material::Empty) {
                            let should_drop = if wall_neighbors > 0 {
                                // Extend existing wall
                                // Bias towards High Pheromone (Heat Trace)
                                if current_phero > 10.0 {
                                    rng.gen_bool(0.2)
                                } else {
                                    rng.gen_bool(0.001) // Low chance to build in cold areas
                                }
                            } else {
                                // Start new wall?
                                if current_phero > 50.0 {
                                    rng.gen_bool(0.01) // Nucleate on hot spots
                                } else {
                                    false
                                }
                            };

                            if should_drop {
                                self.grid[idx].material = Material::Wall;
                                agent.carrying = false;
                            }
                        }
                    } else {
                        // Wants to Pick (Erode)
                        // Rule: Pick if Wall is Cold (Low Pheromone) OR Isolated (Noise)
                        if matches!(current_mat, Material::Wall) {
                            let should_pick = if current_phero < 5.0 {
                                // Cold Wall -> Erode
                                if wall_neighbors <= 1 {
                                    rng.gen_bool(0.5) // Prune isolated
                                } else {
                                    rng.gen_bool(0.05) // Slowly erode solid cold walls
                                }
                            } else {
                                // Hot Wall -> Keep
                                rng.gen_bool(0.0001) // Very rare accidental damage
                            };

                            if should_pick {
                                self.grid[idx].material = Material::Empty;
                                agent.carrying = true;
                            }
                        }
                    }
                }
            }
        }

        // 3. Parallel Movement Update
        // Split borrows: Grid is Read-Only, Agents are Mutable
        let grid = &self.grid;
        let agents = &mut self.agents;

        agents.par_iter_mut().for_each(|agent| {
            match agent.kind {
                AgentKind::Air => {
                    let ix = agent.x as usize;
                    let iy = agent.y as usize;
                    let idx = iy * WIDTH + ix;

                    // Physics Forces
                    if idx < grid.len() {
                        let cell = &grid[idx];

                        // Buoyancy: Hot air rises (Gravity is +Y, so Up is -Y)
                        // Buoyancy Force = (AgentTemp - AmbientTemp) * k
                        let ambient_temp = cell.heat.max(10.0); // Use cell temp as ambient
                        let buoyancy = (agent.heat - ambient_temp) * 0.005;
                        agent.vy -= buoyancy;

                        // Pressure: Move from High Density to Low Density
                        // Look at neighbors
                        if ix > 0 && ix < WIDTH - 1 && iy > 0 && iy < HEIGHT - 1 {
                            let left = grid[idx - 1].air_density;
                            let right = grid[idx + 1].air_density;
                            let top = grid[idx - WIDTH].air_density;
                            let bottom = grid[idx + WIDTH].air_density;

                            let dx = left - right;
                            let dy = top - bottom; // Higher density top pushes down (+Y)

                            // Pressure Strength
                            let k_p = 0.05;
                            agent.vx += dx * k_p;
                            agent.vy += dy * k_p;
                        }
                    }

                    // Wall Collision (Bounce)
                    // We need to check next position
                    let next_x = agent.x + agent.vx;
                    let next_y = agent.y + agent.vy;
                    let next_ix = next_x.clamp(0.0, width - 1.0) as usize;
                    let next_iy = next_y.clamp(0.0, height - 1.0) as usize;
                    let next_idx = next_iy * WIDTH + next_ix;

                    if next_idx < grid.len() && matches!(grid[next_idx].material, Material::Wall) {
                        // Reflect
                        agent.vx *= -0.8;
                        agent.vy *= -0.8;
                        // Don't move into wall
                    } else {
                        agent.x = next_x;
                        agent.y = next_y;
                    }

                    // Damping / Drag
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
                    // PERF: Only initialize RNG for Termites (avoiding TLS overhead for 50k Air agents)
                    let mut rng = rand::thread_rng();
                    // Simple Random Walk for now (Placeholder for Step 4)
                    agent.vx += rng.gen_range(-0.5..0.5);
                    agent.vy += rng.gen_range(-0.5..0.5);
                    agent.vx *= 0.9;
                    agent.vy *= 0.9;
                    agent.x = (agent.x + agent.vx).clamp(1.0, width - 2.0);
                    agent.y = (agent.y + agent.vy).clamp(1.0, height - 2.0);
                }
            }
        });
    }
}
