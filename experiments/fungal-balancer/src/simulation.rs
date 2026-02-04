use rand::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub struct Node {
    pub x: usize,
    pub y: usize,
    pub load: f32,
    pub capacity: f32,
    pub max_capacity: f32,
}

pub struct Agent {
    pub x: f64,
    pub y: f64,
    pub heading: f64,
    pub payload: f32,
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub capacity_pheromone: Vec<f32>,
    pub trail_pheromone: Vec<f32>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            capacity_pheromone: vec![0.0; width * height],
            trail_pheromone: vec![0.0; width * height],
        }
    }

    pub fn diffuse(&mut self, decay: f32) {
        let w = self.width;
        let h = self.height;

        // Helper for diffusion
        let diffuse_layer = |source: &Vec<f32>| -> Vec<f32> {
            let mut next = vec![0.0; w * h];
            for y in 0..h {
                for x in 0..w {
                    let mut sum = 0.0;
                    let mut count = 0.0;

                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let nx = (x as isize + dx).rem_euclid(w as isize) as usize;
                            let ny = (y as isize + dy).rem_euclid(h as isize) as usize;
                            sum += source[ny * w + nx];
                            count += 1.0;
                        }
                    }
                    next[y * w + x] = (sum / count) * (1.0 - decay);
                }
            }
            next
        };

        self.capacity_pheromone = diffuse_layer(&self.capacity_pheromone);
        self.trail_pheromone = diffuse_layer(&self.trail_pheromone);
    }

    pub fn add_signal(&mut self, x: usize, y: usize, amount: f32) {
        if x < self.width && y < self.height {
            self.capacity_pheromone[y * self.width + x] += amount;
        }
    }

    pub fn add_trail(&mut self, x: f64, y: f64, amount: f32) {
        let ix = x.round().rem_euclid(self.width as f64) as usize;
        let iy = y.round().rem_euclid(self.height as f64) as usize;
        if ix < self.width && iy < self.height {
            self.trail_pheromone[iy * self.width + ix] += amount;
        }
    }

    pub fn sample_capacity(&self, x: f64, y: f64) -> f32 {
        let ix = x.round().rem_euclid(self.width as f64) as usize;
        let iy = y.round().rem_euclid(self.height as f64) as usize;
        if ix < self.width && iy < self.height {
            self.capacity_pheromone[iy * self.width + ix]
        } else {
            0.0
        }
    }
}

pub struct World {
    pub grid: Grid,
    pub nodes: Vec<Node>,
    pub agents: Vec<Agent>,
    pub rng: ThreadRng,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Grid::new(width, height),
            nodes: Vec::new(),
            agents: Vec::new(),
            rng: thread_rng(),
        }
    }

    pub fn add_node(&mut self, x: usize, y: usize, capacity: f32, load: f32) {
        self.nodes.push(Node {
            x,
            y,
            load,
            capacity,
            max_capacity: capacity,
        });
    }

    pub fn update(&mut self) {
        // 1. Emit Pheromones
        // Nodes with spare capacity emit signal
        for node in &self.nodes {
            let spare = node.capacity - node.load;
            if spare > 0.0 {
                // Stronger signal for more capacity, capped
                self.grid.add_signal(node.x, node.y, spare.min(10.0));
            }
        }

        // 2. Diffuse
        self.grid.diffuse(0.02); // Decay (slower decay for persistence)

        // 3. Spawn Agents from Overloaded Nodes
        let mut new_agents = Vec::new();
        for node in &mut self.nodes {
            if node.load > node.capacity {
                // Deterministically spawn if overloaded
                // Max 1 agent per tick to prevent explosion
                if node.load >= 1.0 {
                    node.load -= 1.0;
                    new_agents.push(Agent {
                        x: node.x as f64,
                        y: node.y as f64,
                        heading: self.rng.gen_range(0.0..std::f64::consts::TAU),
                        payload: 1.0,
                    });
                }
            }
        }
        self.agents.append(&mut new_agents);

        // 4. Move Agents & Interact
        let sensor_dist = 5.0;
        let sensor_angle = std::f64::consts::FRAC_PI_4; // 45 degrees
        let turn_rate = 0.5; // radians per tick
        let speed = 0.8;

        let grid = &self.grid; // Immutable borrow for sensing
        let nodes = &mut self.nodes; // Mutable borrow for interaction

        let mut dead_indices = Vec::new();
        let mut trail_deposits = Vec::new();

        for (i, agent) in self.agents.iter_mut().enumerate() {
            // Sense
            let heading = agent.heading;
            let l_angle = heading - sensor_angle;
            let r_angle = heading + sensor_angle;

            let sample = |angle: f64| -> f32 {
                let sx = agent.x + angle.cos() * sensor_dist;
                let sy = agent.y + angle.sin() * sensor_dist;
                grid.sample_capacity(sx, sy)
            };

            let v_c = sample(heading);
            let v_l = sample(l_angle);
            let v_r = sample(r_angle);

            // Physarum-style turning
            if v_c > v_l && v_c > v_r {
                // Continue straight (maybe slight wobble)
            } else if v_c < v_l && v_c < v_r {
                // Random turn
                agent.heading += (self.rng.gen::<f64>() - 0.5) * 2.0;
            } else if v_l < v_r {
                agent.heading += turn_rate;
            } else if v_l > v_r {
                agent.heading -= turn_rate;
            }

            // Move
            agent.x += agent.heading.cos() * speed;
            agent.y += agent.heading.sin() * speed;

            // Wrap
            agent.x = agent.x.rem_euclid(grid.width as f64);
            agent.y = agent.y.rem_euclid(grid.height as f64);

            // Deposit Trail (deferred)
            trail_deposits.push((agent.x, agent.y));

            // Interact with Nodes
            for node in nodes.iter_mut() {
                let dx = agent.x - node.x as f64;
                let dy = agent.y - node.y as f64;
                let dist_sq = dx * dx + dy * dy;

                if dist_sq < 2.0 { // Radius ~1.4
                    // Check if node can accept payload
                    // Only deposit if node is not the one we just spawned from (implied by distance usually, but also capacity)
                    // If node has capacity, deposit
                    if node.load < node.capacity {
                        node.load += agent.payload;
                        dead_indices.push(i);
                        break; // Process one interaction per tick
                    }
                }
            }
        }

        // Apply trail deposits
        for (x, y) in trail_deposits {
            self.grid.add_trail(x, y, 5.0);
        }

        // Remove dead agents
        dead_indices.sort_unstable_by(|a, b| b.cmp(a));
        dead_indices.dedup();
        for idx in dead_indices {
            self.agents.swap_remove(idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_and_transfer() {
        let mut world = World::new(20, 20);

        // Overloaded node: Load 100, Capacity 50
        world.add_node(5, 5, 50.0, 100.0);

        // Underloaded node: Load 0, Capacity 50
        world.add_node(15, 15, 50.0, 0.0);

        // Run update once - should spawn agent
        world.update();
        assert!(!world.agents.is_empty(), "Should have spawned an agent from overloaded node");

        // Check that load decreased on source
        assert!(world.nodes[0].load < 100.0, "Source node load should decrease when agent spawns");

        // Run many updates to allow travel
        // Distance is ~14 units. Speed 0.8. ~18 ticks minimum if straight line.
        // But need diffusion to happen first to create gradient.
        // Let's give it plenty of time.

        let mut transferred = false;
        for _ in 0..200 {
            world.update();
            if world.nodes[1].load > 0.0 {
                transferred = true;
                break;
            }
        }

        assert!(transferred, "Load should eventually transfer to the underloaded node");
    }
}
