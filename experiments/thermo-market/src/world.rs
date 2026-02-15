use rand::prelude::*;
use rayon::prelude::*;
use crate::market::{self, Particle};

pub const WIDTH: usize = 512;
pub const HEIGHT: usize = 512;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Material {
    Empty,
    Wall, // Cooling Fin
    Server, // Exchange / Heat Source
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
    pub market: market::Grid,
    pub step: u64,
    // Double buffers for physics (Reuse memory)
    current_heats: Vec<f32>,
    current_pheros: Vec<f32>,
}

impl World {
    pub fn new() -> Self {
        let grid = vec![Cell::default(); WIDTH * HEIGHT];
        let market = market::Grid::new(WIDTH, HEIGHT);
        Self {
            grid,
            agents: Vec::new(),
            market,
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
        // 1. Sync Thermo -> Market (Walls)
        self.sync_walls_to_market();

        // 2. Inject Random Orders (Simulate Market Activity)
        self.inject_market_orders();

        // 3. Update Market
        self.market.update();

        // 4. Sync Market -> Thermo (Heat from Trades)
        self.sync_heat_from_market();

        // 5. Update Thermo Physics & Agents
        self.update_grid_physics();
        self.update_agents();
        self.step += 1;
    }

    fn sync_walls_to_market(&mut self) {
        // Map walls.
        // Optimization: Only update if changed?
        // For now, full sweep is okay (262k iters is fast in Rust).
        // But we must NOT overwrite Bids/Asks/Trades if Wall appears?
        // Actually, if a Wall is built on top of a Bid, the Bid should be crushed (Empty/Wall).
        for i in 0..self.grid.len() {
            let mat = self.grid[i].material;
            let current_p = self.market.cells[i];

            match mat {
                Material::Wall => {
                    // Force Wall
                    if !matches!(current_p, Particle::Wall) {
                        self.market.cells[i] = Particle::Wall;
                    }
                }
                Material::Empty | Material::Server => {
                    // Remove Wall if present (Termite removed it)
                    if matches!(current_p, Particle::Wall) {
                        self.market.cells[i] = Particle::Empty;
                    }
                }
            }
        }
    }

    fn inject_market_orders(&mut self) {
        let mut rng = rand::thread_rng();
        // Inject some Bids at bottom, Asks at top
        for _ in 0..50 {
            let x = rng.gen_range(0..WIDTH);
            // Bids at bottom
            if matches!(self.market.get(x, HEIGHT - 1), Particle::Empty) {
                self.market.set(x, HEIGHT - 1, Particle::Bid(rng.gen()));
            }

            // Asks at top
            let x = rng.gen_range(0..WIDTH);
            if matches!(self.market.get(x, 0), Particle::Empty) {
                self.market.set(x, 0, Particle::Ask(rng.gen()));
            }
        }
    }

    fn sync_heat_from_market(&mut self) {
        // Scan for new trades
        // New trades have age = DEFAULT_TRADE_AGE - 1
        let new_trade_age = market::DEFAULT_TRADE_AGE - 1;

        for i in 0..self.market.cells.len() {
            if let Particle::Trade { age } = self.market.cells[i] {
                if age == new_trade_age {
                    // Boom! Heat released.
                    // Price Discovery = Energy Release
                    self.grid[i].heat += 50.0;
                    // Also trigger pheromone so Termites come to investigate/cool
                    self.grid[i].pheromone += 5.0;
                }
            }
        }
    }

    fn update_grid_physics(&mut self) {
        let width = WIDTH;
        let height = HEIGHT;

        for (i, cell) in self.grid.iter().enumerate() {
            self.current_heats[i] = cell.heat;
            self.current_pheros[i] = cell.pheromone;
        }

        let current_heats = &self.current_heats;
        let current_pheros = &self.current_pheros;
        let grid = &mut self.grid;

        grid.par_iter_mut()
            .enumerate()
            .for_each(|(i, cell)| {
                let x = i % width;
                let y = i / width;

                if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                    cell.heat *= 0.99;
                    cell.pheromone *= 0.99;
                    return;
                }

                let diffusion = 0.20;
                let wall_insulation = 0.05;
                let cooling = 0.001;
                let evap = 0.02;
                let pheromone_diffusion = 0.15;
                let max_heat = 2000.0; // Higher cap for market heat

                // --- Heat Diffusion ---
                if matches!(cell.material, Material::Server) {
                    // Servers produce baseline heat
                    cell.heat = (current_heats[i] + 1.0).min(max_heat);
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

        self.grid.par_iter_mut().for_each(|cell| {
            cell.air_density = 0.0;
            cell.air_vx = 0.0;
            cell.air_vy = 0.0;
        });

        // Termite Logic
        // We modify grid here (sequentially)
        // Termites build walls to cool down hot spots
        let mut rng = rand::thread_rng();
        for agent in &mut self.agents {
            agent.x = agent.x.clamp(0.0, width - 1.0);
            agent.y = agent.y.clamp(0.0, height - 1.0);

            let ix = agent.x as usize;
            let iy = agent.y as usize;
            let idx = iy * WIDTH + ix;

            if idx < self.grid.len() {
                let cell = &mut self.grid[idx];

                if matches!(agent.kind, AgentKind::Air) {
                    cell.air_density += 1.0;
                    cell.air_vx += agent.vx;
                    cell.air_vy += agent.vy;

                    // Heat Exchange
                    if matches!(cell.material, Material::Server) {
                        agent.heat += 5.0;
                        cell.heat -= 0.1;
                    } else if matches!(cell.material, Material::Wall) {
                        // Wall absorbs heat from air?
                        // Or Air cools wall?
                        // Let's say fins absorb heat.
                        if agent.heat > 50.0 {
                            cell.pheromone = (cell.pheromone + 1.0).min(100.0);
                            agent.heat *= 0.9;
                        }
                        agent.vx *= 0.9;
                        agent.vy *= 0.9;
                    } else {
                        let eq_heat = (cell.heat + agent.heat) * 0.5;
                        let transfer = (eq_heat - agent.heat) * 0.1;
                        agent.heat += transfer;
                        cell.heat -= transfer;
                    }
                } else if matches!(agent.kind, AgentKind::Termite) {
                    let current_mat = self.grid[idx].material;
                    let current_phero = self.grid[idx].pheromone;
                    let current_heat = self.grid[idx].heat;

                    // Neighbors
                    let mut wall_neighbors = 0;
                    let neighbors = [
                        if ix > 0 { Some(idx - 1) } else { None },
                        if ix < WIDTH - 1 { Some(idx + 1) } else { None },
                        if iy > 0 { Some(idx - WIDTH) } else { None },
                        if iy < HEIGHT - 1 { Some(idx + WIDTH) } else { None },
                    ];

                    for n_opt in neighbors {
                        if let Some(n_idx) = n_opt {
                            if matches!(self.grid[n_idx].material, Material::Wall) {
                                wall_neighbors += 1;
                            }
                        }
                    }

                    if agent.carrying {
                        // Want to Drop (Build)
                        if matches!(current_mat, Material::Empty) {
                            let should_drop = if wall_neighbors > 0 {
                                // Extend wall towards Heat (Pheromone)
                                if current_phero > 5.0 {
                                    rng.gen_bool(0.3)
                                } else {
                                    rng.gen_bool(0.001)
                                }
                            } else {
                                // Start new wall if VERY HOT
                                if current_heat > 50.0 {
                                    rng.gen_bool(0.05)
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
                        // Want to Pick (Erode)
                        if matches!(current_mat, Material::Wall) {
                            let should_pick = if current_heat < 10.0 {
                                // Cold wall -> Recycle
                                if wall_neighbors <= 1 {
                                    rng.gen_bool(0.5)
                                } else {
                                    rng.gen_bool(0.01)
                                }
                            } else {
                                // Hot Wall -> Keep! (It's working)
                                false
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

        // Movement
        let grid = &self.grid;
        let agents = &mut self.agents;

        agents.par_iter_mut().for_each(|agent| {
            // ... (Same movement logic as parent, but maybe simplified)
            let mut rng = rand::thread_rng();

            match agent.kind {
                AgentKind::Air => {
                    let ix = agent.x as usize;
                    let iy = agent.y as usize;
                    let idx = iy * WIDTH + ix;

                    if idx < grid.len() {
                        let cell = &grid[idx];
                        let ambient_temp = cell.heat.max(10.0);
                        let buoyancy = (agent.heat - ambient_temp) * 0.005;
                        agent.vy -= buoyancy;

                        // Random jitter (Temperature = Kinetic Energy)
                        agent.vx += rng.gen_range(-0.1..0.1) * (agent.heat * 0.01);
                        agent.vy += rng.gen_range(-0.1..0.1) * (agent.heat * 0.01);
                    }

                    // Simple bounces
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
                    if agent.x <= 0.0 || agent.x >= width - 1.0 { agent.vx *= -1.0; agent.x = agent.x.clamp(0.0, width - 1.0); }
                    if agent.y <= 0.0 || agent.y >= height - 1.0 { agent.vy *= -1.0; agent.y = agent.y.clamp(0.0, height - 1.0); }
                },
                AgentKind::Termite => {
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
