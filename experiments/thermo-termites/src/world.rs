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
    pub next_heat: f32,
    pub pheromone: f32,
    pub next_pheromone: f32,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            material: Material::Empty,
            heat: 0.0,
            next_heat: 0.0,
            pheromone: 0.0,
            next_pheromone: 0.0,
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
        world.diffuse_grid();

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

    #[test]
    fn test_boundary_preservation() {
        let mut world = World::new();
        // Set a boundary cell (top-left corner) heat
        let idx = world.get_index(0, 0);
        world.grid[idx].heat = 100.0;

        world.diffuse_grid();

        // Should not be zero (maybe cooled slightly by global cooling 0.005)
        assert!(
            world.grid[idx].heat > 90.0,
            "Boundary heat was reset to zero! expected > 90.0, got {}",
            world.grid[idx].heat
        );
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
}

impl World {
    pub fn new() -> Self {
        let grid = vec![Cell::default(); WIDTH * HEIGHT];
        Self {
            grid,
            agents: Vec::new(),
            step: 0,
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
        self.diffuse_grid();
        self.update_agents();
        self.step += 1;
    }

    fn diffuse_grid(&mut self) {
        // Parallel diffusion
        // We need to read from 'heat' and write to 'next_heat'
        // We can iterate chunks of rows

        // Heat diffusion rate
        let diffusion = 0.20; // Fast
        let cooling = 0.005; // Global cooling
        let evap = 0.01; // Pheromone evaporation
        let pheromone_diffusion = 0.1;

        // Use par_chunks_mut to allow writing to next_* safely if we split correctly.
        // Or better: parallel loop over indices.
        // We need read access to the whole grid (read-only) and write access to the current chunk.
        // Since we can't easily share the read-ref with mut-ref in safe Rust without unsafe or splitting,
        // we will use a simpler approach:
        // 1. Compute next values into a temporary buffer?
        // No, we have next_heat in the struct.
        // But we can't iterate self.grid mutably AND read neighbors immutably easily.
        // Classic Double Buffer problem.
        //
        // Strategy:
        // Unsafe pointer magic is fastest but dangerous.
        // Index-based loop with `split_at_mut` is safe but complex.
        //
        // Let's use `par_iter` on indices and collect results? No, allocation.
        //
        // Let's do it single threaded for the grid for now. 512x512 is 262k ops. fast enough.
        // Actually, with 262k, single thread is fine. It's < 1ms.

        let width = WIDTH;
        let height = HEIGHT;

        // Clone grid state for reading? Expensive.
        // We can just iterate linearly. For strict CA, we need a snapshot.
        // Let's just use the current values as approximation (Gauss-Seidel style),
        // but that introduces bias.
        //
        // Correct way:
        // let old_grid = self.grid.clone(); // Ouch, 3MB copy per frame.
        // Maybe we just alternate buffers? Struct of Arrays would be better.
        //
        // Optimization: Use `next_heat` as the write buffer.
        // We read from `grid` (heat) and write to `grid` (next_heat).
        // But we can't have &mut self.grid and &self.grid.
        //
        // We can iterate indices.

        self.grid.par_iter_mut().enumerate().for_each(|(i, cell)| {
            let x = i % width;
            let y = i / width;

            // Boundary check optimization
            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                cell.next_heat = cell.heat * (1.0 - cooling);
                cell.next_pheromone = cell.pheromone * (1.0 - evap);
                return;
            }

            // Neighbors indices
            // old_grid is not available here.
            // WAIT. We can't access neighbors inside par_iter_mut of the same vec.
        });

        // Okay, simpler: sequential is fine for 512x512.
        // Or use `chunks_exact`.

        // Let's implement sequential first.
        // Initialize with current values to preserve boundaries
        let mut next_heats: Vec<f32> = self.grid.iter().map(|c| c.heat).collect();
        let mut next_pheros: Vec<f32> = self.grid.iter().map(|c| c.pheromone).collect();

        for y in 1..HEIGHT - 1 {
            for x in 1..WIDTH - 1 {
                let idx = y * WIDTH + x;
                let cell = &self.grid[idx];

                // Heat Logic
                if matches!(cell.material, Material::Server) {
                    next_heats[idx] = (cell.heat + 5.0).min(500.0);
                } else {
                    let top = self.grid[idx - WIDTH].heat;
                    let bottom = self.grid[idx + WIDTH].heat;
                    let left = self.grid[idx - 1].heat;
                    let right = self.grid[idx + 1].heat;

                    let avg = (top + bottom + left + right) * 0.25;
                    let diff = avg - cell.heat;

                    // Walls insulate?
                    let diff_rate = if matches!(cell.material, Material::Wall) {
                        diffusion * 0.1
                    } else {
                        diffusion
                    };

                    next_heats[idx] = (cell.heat + diff * diff_rate) * (1.0 - cooling);
                }

                // Pheromone Logic
                let top_p = self.grid[idx - WIDTH].pheromone;
                let bottom_p = self.grid[idx + WIDTH].pheromone;
                let left_p = self.grid[idx - 1].pheromone;
                let right_p = self.grid[idx + 1].pheromone;

                let avg_p = (top_p + bottom_p + left_p + right_p) * 0.25;
                next_pheros[idx] = (cell.pheromone
                    + (avg_p - cell.pheromone) * pheromone_diffusion)
                    * (1.0 - evap);
            }
        }

        // Apply back
        // Parallel apply
        self.grid.par_iter_mut().enumerate().for_each(|(i, cell)| {
            if i < next_heats.len() {
                cell.heat = next_heats[i];
                cell.pheromone = next_pheros[i];
            }
        });
    }

    fn update_agents(&mut self) {
        let width = WIDTH as f32;
        let height = HEIGHT as f32;

        // Parallel update of agents
        // But agents interact with grid.
        // Grid is shared resource.
        // Data race if multiple agents write to same cell.
        //
        // Solution:
        // 1. Agents READ grid to decide movement.
        // 2. Agents WRITE to separate "Action Buffer"?
        //
        // Simplified:
        // Update positions first (local only).
        // Then handle interactions serially or with atomics?
        //
        // For Termites (Wall building):
        // Only one termite can modify a cell at a time.
        //
        // Let's do sequential update for interactions to be safe for now,
        // or use `AtomicU32` for grid state (hard with floats).
        //
        // We can optimize later. 100k agents sequential update:
        // 100,000 * 100 cycles = 10M cycles. ~5ms. Safe for 60fps.

        let mut rng = rand::thread_rng();

        for agent in &mut self.agents {
            match agent.kind {
                AgentKind::Air => {
                    // Move
                    agent.x += agent.vx;
                    agent.y += agent.vy;

                    // Convection (Heat rises -> negative Y)
                    if agent.heat > 10.0 {
                        agent.vy -= 0.05;
                    }

                    // Bounds & Bounce
                    if agent.x <= 0.0 || agent.x >= width - 1.0 {
                        agent.vx *= -1.0;
                        agent.x = agent.x.clamp(0.0, width - 1.0);
                    }
                    if agent.y <= 0.0 || agent.y >= height - 1.0 {
                        agent.vy *= -1.0;
                        agent.y = agent.y.clamp(0.0, height - 1.0);
                        // Floor/Ceiling thermal interaction?
                        if agent.y < 5.0 {
                            agent.heat *= 0.8;
                        } // Top is cooling sink (if -y is up)
                          // Wait, y=0 is TOP in standard grid? usually.
                          // Let's assume y=0 is TOP.
                    }

                    let ix = agent.x as usize;
                    let iy = agent.y as usize;
                    let idx = iy * WIDTH + ix;

                    if idx < self.grid.len() {
                        let cell = &mut self.grid[idx];

                        // Bounce off walls
                        if matches!(cell.material, Material::Wall) {
                            agent.vx *= -1.0;
                            agent.vy *= -1.0;
                            // Simple bounce
                            agent.x += agent.vx;
                            agent.y += agent.vy;
                        } else if matches!(cell.material, Material::Server) {
                            agent.heat += 5.0;
                            cell.heat -= 0.1; // Cool the server slightly
                        }

                        // Exchange heat with air
                        let eq_heat = (cell.heat + agent.heat) * 0.5;
                        let transfer = (eq_heat - agent.heat) * 0.1;
                        agent.heat += transfer;
                        cell.heat -= transfer * 0.01; // Air has less thermal mass
                    }
                }
                AgentKind::Termite => {
                    // Move Randomly
                    agent.vx += rng.gen_range(-0.5..0.5);
                    agent.vy += rng.gen_range(-0.5..0.5);

                    // Dampen
                    agent.vx *= 0.9;
                    agent.vy *= 0.9;

                    agent.x += agent.vx;
                    agent.y += agent.vy;

                    // Clamp
                    agent.x = agent.x.clamp(1.0, width - 2.0);
                    agent.y = agent.y.clamp(1.0, height - 2.0);

                    let ix = agent.x as usize;
                    let iy = agent.y as usize;
                    let idx = iy * WIDTH + ix;

                    // Actions
                    if idx < self.grid.len() {
                        // Pick/Drop
                        // Count neighbors
                        let mut neighbors = 0;
                        // Simple 4-neighbor check
                        if ix > 0 && matches!(self.grid[idx - 1].material, Material::Wall) {
                            neighbors += 1;
                        }
                        if ix < WIDTH - 1 && matches!(self.grid[idx + 1].material, Material::Wall) {
                            neighbors += 1;
                        }
                        if iy > 0 && matches!(self.grid[idx - WIDTH].material, Material::Wall) {
                            neighbors += 1;
                        }
                        if iy < HEIGHT - 1
                            && matches!(self.grid[idx + WIDTH].material, Material::Wall)
                        {
                            neighbors += 1;
                        }

                        let cell = &mut self.grid[idx];

                        // Drop Pheromone
                        if agent.carrying {
                            cell.pheromone = (cell.pheromone + 10.0).min(100.0);
                        }

                        if agent.carrying {
                            // Wants to drop
                            // If near other walls (building) OR High Pheromone
                            // Avoid dropping on servers or existing walls
                            if matches!(cell.material, Material::Empty) {
                                let should_drop = if neighbors > 0 {
                                    rng.gen_bool(0.05) // Build onto existing
                                } else if cell.pheromone > 20.0 {
                                    rng.gen_bool(0.1) // Stigmergy
                                } else {
                                    rng.gen_bool(0.001) // Random drop
                                };

                                if should_drop {
                                    cell.material = Material::Wall;
                                    agent.carrying = false;
                                }
                            }
                        } else {
                            // Wants to pick
                            if matches!(cell.material, Material::Wall) {
                                // Pick if isolated
                                let should_pick = if neighbors <= 1 {
                                    rng.gen_bool(0.1)
                                } else {
                                    rng.gen_bool(0.0001) // Rarely break walls
                                };

                                if should_pick {
                                    cell.material = Material::Empty;
                                    agent.carrying = true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
