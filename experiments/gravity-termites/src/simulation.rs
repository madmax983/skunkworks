use rayon::prelude::*;
use rand::prelude::*;
use macroquad::prelude::Vec2;

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Material {
    Empty,
    Dust,
    Rock,
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub material: Material,
    pub potential: f32, // Gravitational potential field (High = Mass)
    pub next_potential: f32,
    pub mass: f32,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            material: Material::Empty,
            potential: 0.0,
            next_potential: 0.0,
            mass: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_step() {
        let mut world = World::new();
        // Run a few steps
        for _ in 0..10 {
            world.update();
        }
        // Check if potential is non-zero (mass exists)
        let has_potential = world.grid.iter().any(|c| c.potential != 0.0);
        assert!(has_potential);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Agent {
    pub pos: Vec2,
    pub vel: Vec2,
    pub carrying: bool,
}

pub struct World {
    pub grid: Vec<Cell>,
    pub agents: Vec<Agent>,
    pub step: u64,
}

impl World {
    pub fn new() -> Self {
        let mut grid = vec![Cell::default(); WIDTH * HEIGHT];
        let mut rng = rand::thread_rng();

        // Initialize with random dust cloud
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let dx = x as f32 - WIDTH as f32 / 2.0;
                let dy = y as f32 - HEIGHT as f32 / 2.0;
                let dist = (dx*dx + dy*dy).sqrt();

                // Ring of dust
                if dist > 50.0 && dist < 120.0 && rng.gen_bool(0.05) {
                    grid[y * WIDTH + x].material = Material::Dust;
                }
            }
        }

        // Add a central seed (rock)
        let cx = WIDTH / 2;
        let cy = HEIGHT / 2;
        // Make a small asteroid
        for dy in -2..=2 {
            for dx in -2..=2 {
                if dx*dx + dy*dy <= 4 {
                    let idx = (cy as isize + dy) as usize * WIDTH + (cx as isize + dx) as usize;
                    grid[idx].material = Material::Rock;
                }
            }
        }

        // Initialize agents in orbit
        let agents = (0..5000).map(|_| {
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let r = rng.gen_range(80.0..140.0);
            let x = cx as f32 + angle.cos() * r;
            let y = cy as f32 + angle.sin() * r;

            // Orbital velocity roughly
            let v_angle = angle + std::f32::consts::FRAC_PI_2;
            let speed = 0.8;

            Agent {
                pos: Vec2::new(x, y),
                vel: Vec2::new(v_angle.cos() * speed, v_angle.sin() * speed),
                carrying: false,
            }
        }).collect();

        Self {
            grid,
            agents,
            step: 0,
        }
    }

    pub fn update(&mut self) {
        self.update_physics();
        self.update_agents();
        self.step += 1;
    }

    fn update_physics(&mut self) {
        // 1. Update Mass from Material
        // We use par_iter_mut for speed
        self.grid.par_iter_mut().for_each(|cell| {
             cell.mass = match cell.material {
                 Material::Rock => 2.0, // High mass
                 Material::Dust => 0.1, // Low mass
                 Material::Empty => 0.0,
             };
        });

        // 2. Solve Poisson for Potential (Gravity)
        // Using Jacobi iteration to diffuse the "mass influence"
        let passes = 5;
        for _ in 0..passes {
             // Sequential pass to avoid complexity with parallel neighbor access
             // With 256x256, sequential is fast enough (~65k iters)
             for i in 0..WIDTH*HEIGHT {
                 let x = i % WIDTH;
                 let y = i / WIDTH;

                 // Boundary condition: potential 0 at edges
                 if x == 0 || x == WIDTH - 1 || y == 0 || y == HEIGHT - 1 {
                     self.grid[i].next_potential = 0.0;
                     continue;
                 }

                 let up = self.grid[i - WIDTH].potential;
                 let down = self.grid[i + WIDTH].potential;
                 let left = self.grid[i - 1].potential;
                 let right = self.grid[i + 1].potential;
                 let mass = self.grid[i].mass;

                 // Poisson/Diffusion step:
                 // Potential propagates from mass.
                 // This effectively creates a gradient pointing towards mass.
                 self.grid[i].next_potential = 0.25 * (up + down + left + right) + mass * 0.5;
             }

             // Update potential buffer
             for cell in &mut self.grid {
                 cell.potential = cell.next_potential;
             }
        }
    }

    fn update_agents(&mut self) {
        let width_f = WIDTH as f32;
        let height_f = HEIGHT as f32;
        let mut rng = rand::thread_rng();

        for agent in &mut self.agents {
            let ix = agent.pos.x as usize;
            let iy = agent.pos.y as usize;

            // 1. Calculate Gravity Gradient (Force)
            let mut ax = 0.0;
            let mut ay = 0.0;

            if ix > 0 && ix < WIDTH - 1 && iy > 0 && iy < HEIGHT - 1 {
                let idx = iy * WIDTH + ix;
                let l = self.grid[idx - 1].potential;
                let r = self.grid[idx + 1].potential;
                let u = self.grid[idx - WIDTH].potential;
                let d = self.grid[idx + WIDTH].potential;

                // Gradient points towards higher potential (mass)
                ax = (r - l) * 0.5;
                ay = (d - u) * 0.5;
            }

            // Apply Gravity Acceleration
            agent.vel.x += ax * 0.05;
            agent.vel.y += ay * 0.05;

            // Drag (simulated vacuum friction/viscosity)
            agent.vel *= 0.99;

            // Update Position
            agent.pos += agent.vel;

            // Bounce off world bounds
            if agent.pos.x < 1.0 || agent.pos.x >= width_f - 1.0 {
                agent.vel.x *= -1.0;
                agent.pos.x = agent.pos.x.clamp(1.0, width_f - 1.0);
            }
            if agent.pos.y < 1.0 || agent.pos.y >= height_f - 1.0 {
                agent.vel.y *= -1.0;
                agent.pos.y = agent.pos.y.clamp(1.0, height_f - 1.0);
            }

            // Interaction with World
            let nx = agent.pos.x as usize;
            let ny = agent.pos.y as usize;
            let idx = ny * WIDTH + nx;

            if idx < self.grid.len() {
                // Read-only access first
                let cell_material = self.grid[idx].material;
                let cell_potential = self.grid[idx].potential;

                if matches!(cell_material, Material::Rock) {
                    // Collision with Rock
                    // Termites can "walk" on rock
                    agent.vel *= 0.8; // High friction on rock

                    // Random walk behavior
                    agent.vel.x += rng.gen_range(-0.1..0.1);
                    agent.vel.y += rng.gen_range(-0.1..0.1);
                }

                // Carrying Logic
                if agent.carrying {
                    // Drop Logic:
                    // Drop if carrying and near other rock (accretion)

                    if matches!(cell_material, Material::Empty) {
                         // Check neighbors for adjacency
                         let mut neighbors = 0;
                         if nx > 0 && matches!(self.grid[idx-1].material, Material::Rock) { neighbors += 1; }
                         if nx < WIDTH-1 && matches!(self.grid[idx+1].material, Material::Rock) { neighbors += 1; }
                         if ny > 0 && matches!(self.grid[idx-WIDTH].material, Material::Rock) { neighbors += 1; }
                         if ny < HEIGHT-1 && matches!(self.grid[idx+WIDTH].material, Material::Rock) { neighbors += 1; }

                         // Drop if adjacent to rock OR high gravity (center of planet)
                         if neighbors > 0 || cell_potential > 5.0 {
                             if rng.gen_bool(0.2) {
                                 self.grid[idx].material = Material::Rock; // Compact dust into rock
                                 agent.carrying = false;
                             }
                         }
                    }
                } else {
                    // Pick Logic:
                    // Pick Dust
                    if matches!(cell_material, Material::Dust) {
                        self.grid[idx].material = Material::Empty;
                        agent.carrying = true;
                    }
                }
            }
        }
    }
}
