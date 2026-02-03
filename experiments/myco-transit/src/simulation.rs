use rand::prelude::*;
use rayon::prelude::*;
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Agent {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
    pub home_city: usize,
    pub target_city: usize,
    pub commuted_count: u32,
}

pub struct AgentUpdateResult {
    pub deposit_x: usize,
    pub deposit_y: usize,
    pub deposit_amount: f64,
}

impl Agent {
    pub fn new(x: f64, y: f64, angle: f64, home_city: usize, target_city: usize) -> Self {
        Self {
            x,
            y,
            angle,
            home_city,
            target_city,
            commuted_count: 0,
        }
    }

    /// Senses combined value of Trail + Gradient
    /// Gradient is simulated as 1/distance_to_target
    pub fn sense(&self, world: &World, angle_offset: f64, sensor_dist: f64) -> f64 {
        let sensor_angle = self.angle + angle_offset;
        let sensor_x = self.x + sensor_angle.cos() * sensor_dist;
        let sensor_y = self.y + sensor_angle.sin() * sensor_dist;

        // Wrap coords for sensing
        let w = world.width as f64;
        let h = world.height as f64;
        let sx = (sensor_x.rem_euclid(w)) as usize;
        let sy = (sensor_y.rem_euclid(h)) as usize;

        let trail_strength = world.get_trail(sx, sy);

        // Gradient Attraction
        // Calculate distance from sensor tip to target city
        // Note: We don't account for world wrapping in distance here for simplicity,
        // so agents might take the "long way" around the torus if targets are across the seam.
        // Given visualization is flat 2D, this is acceptable.
        let (tx, ty) = world.cities[self.target_city];

        // Simple Euclidean distance
        // We use world coords directly.
        let dx = tx - sensor_x; // If sensor wraps, this might be large, but acceptable.
        let dy = ty - sensor_y;
        let dist_sq = dx * dx + dy * dy;
        let dist = dist_sq.sqrt().max(1.0); // Avoid div by zero

        // Weighting: Trail is usually 0..255.
        // 1/dist is small (e.g. 1/100 = 0.01).
        // We want Gradient to be a "nudge".
        // Let's multiply by a large factor so it competes with trail.
        // If dist = 100, val = 1000/100 = 10. Comparable to faint trail.
        // If dist = 10, val = 1000/10 = 100. Strong attraction.
        let gradient_strength = 2000.0 / dist;

        // Blend
        trail_strength + gradient_strength
    }

    /// Updates agent position/angle and returns where it wants to deposit pheromone
    pub fn update(&mut self, world: &World) -> AgentUpdateResult {
        // Check arrival first
        let (tx, ty) = world.cities[self.target_city];
        let dx = tx - self.x;
        let dy = ty - self.y;
        let dist_sq = dx*dx + dy*dy;

        if dist_sq < 25.0 { // Radius 5
             // Arrived!
             std::mem::swap(&mut self.home_city, &mut self.target_city);
             self.angle += PI; // Turn around
             self.commuted_count += 1;

             // No deposit on this frame, just turn
             return AgentUpdateResult {
                 deposit_x: self.x as usize,
                 deposit_y: self.y as usize,
                 deposit_amount: 0.0,
             };
        }

        let sensor_angle = PI / 4.0;
        let sensor_dist = 9.0;
        let turn_angle = PI / 8.0;
        let speed = 1.0;

        let left = self.sense(world, -sensor_angle, sensor_dist);
        let center = self.sense(world, 0.0, sensor_dist);
        let right = self.sense(world, sensor_angle, sensor_dist);

        let mut rng = rand::thread_rng();

        if center > left && center > right {
            // Keep going
        } else if center < left && center < right {
            // Random turn
            if rng.gen_bool(0.5) {
                self.angle += turn_angle;
            } else {
                self.angle -= turn_angle;
            }
        } else if left > right {
            self.angle -= turn_angle;
        } else if right > left {
            self.angle += turn_angle;
        }

        // Move
        self.x += self.angle.cos() * speed;
        self.y += self.angle.sin() * speed;

        // Wrap
        self.x = self.x.rem_euclid(world.width as f64);
        self.y = self.y.rem_euclid(world.height as f64);

        // Deposit
        let ix = self.x as usize;
        let iy = self.y as usize;

        AgentUpdateResult {
            deposit_x: ix,
            deposit_y: iy,
            deposit_amount: 5.0,
        }
    }
}

pub struct World {
    pub width: usize,
    pub height: usize,
    pub trails: Vec<f64>,
    pub next_trails: Vec<f64>,
    pub cities: Vec<(f64, f64)>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            trails: vec![0.0; width * height],
            next_trails: vec![0.0; width * height],
            cities: Vec::new(),
        }
    }

    pub fn with_cities_and_agents(
        width: usize,
        height: usize,
        num_cities: usize,
    ) -> (Self, Vec<Agent>) {
        let mut rng = rand::thread_rng();
        let mut cities = Vec::new();
        // Generate cities
        let padding = 10.0;
        for _ in 0..num_cities {
            cities.push((
                rng.gen_range(padding..(width as f64 - padding)),
                rng.gen_range(padding..(height as f64 - padding)),
            ));
        }

        let mut agents = Vec::new();
        // Scale up!
        let agents_per_city = 1250;

        for (i, (cx, cy)) in cities.iter().enumerate() {
            for _ in 0..agents_per_city {
                // Pick a random target city that is not home
                let mut target = rng.gen_range(0..num_cities);
                // Ensure target != home if possible
                if num_cities > 1 {
                    while target == i {
                        target = rng.gen_range(0..num_cities);
                    }
                }

                agents.push(Agent::new(
                    *cx,
                    *cy,
                    rng.gen_range(0.0..2.0 * PI),
                    i,
                    target
                ));
            }
        }

        let mut world = Self::new(width, height);
        world.cities = cities;
        (world, agents)
    }

    pub fn get_trail(&self, x: usize, y: usize) -> f64 {
        if x >= self.width || y >= self.height {
            return 0.0;
        }
        self.trails[y * self.width + x]
    }

    pub fn set_trail(&mut self, x: usize, y: usize, value: f64) {
        if x < self.width && y < self.height {
            self.trails[y * self.width + x] = value;
        }
    }

    pub fn update_agents_parallel(&mut self, agents: &mut [Agent]) {
        let deposits: Vec<AgentUpdateResult> = agents.par_iter_mut()
            .map(|agent| agent.update(self))
            .collect();

        for deposit in deposits {
            if deposit.deposit_x < self.width && deposit.deposit_y < self.height {
                let idx = deposit.deposit_y * self.width + deposit.deposit_x;
                self.trails[idx] = (self.trails[idx] + deposit.deposit_amount).min(255.0);
            }
        }
    }

    pub fn diffuse_and_decay(&mut self) {
        let decay_factor = 0.9;
        let width = self.width;
        let height = self.height;

        let trails_ref = &self.trails;

        self.next_trails.par_chunks_mut(width).enumerate().for_each(|(y, row)| {
             for (x, pixel) in row.iter_mut().enumerate() {
                let mut sum = 0.0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = (x as isize + dx).rem_euclid(width as isize) as usize;
                        let ny = (y as isize + dy).rem_euclid(height as isize) as usize;
                        sum += trails_ref[ny * width + nx];
                    }
                }
                let avg = sum / 9.0;
                *pixel = avg * decay_factor;
             }
        });

        std::mem::swap(&mut self.trails, &mut self.next_trails);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commuter_turnaround() {
        let width = 100;
        let height = 100;
        let mut world = World::new(width, height);

        // Define cities manually
        world.cities.push((10.0, 10.0)); // City 0
        world.cities.push((90.0, 90.0)); // City 1

        // Create agent at City 0, target City 1
        let mut agent = Agent::new(10.0, 10.0, 0.0, 0, 1);

        // Teleport agent close to target
        agent.x = 89.0;
        agent.y = 89.0;

        // Update
        agent.update(&world);

        // Check if home/target swapped
        assert_eq!(agent.home_city, 1);
        assert_eq!(agent.target_city, 0);
        assert_eq!(agent.commuted_count, 1);

        // Check if it turned around (angle changed by PI approx)
        // Init angle 0.0 -> PI
        assert!((agent.angle - PI).abs() < 0.001 || (agent.angle + PI).abs() < 0.001, "Angle should flip");
    }

    #[test]
    fn test_agent_senses_gradient() {
        let width = 100;
        let height = 100;
        let mut world = World::new(width, height);
        world.cities.push((0.0, 0.0)); // Dummy
        world.cities.push((90.0, 50.0)); // Target City 1 far right

        // Create a vertical gradient increasing towards the right (positive x)
        for x in 0..width {
            for y in 0..height {
                world.set_trail(x, y, x as f64); // Linear gradient 0..99
            }
        }

        let mut agent = Agent::new(50.0, 50.0, -PI / 2.0, 0, 1);
        let initial_angle = agent.angle;

        // Use the new update interface which does not mutate world directly
        let _res = agent.update(&world);

        assert!(
            agent.angle > initial_angle,
            "Agent should turn towards higher concentration (Right). Init: {}, New: {}",
            initial_angle,
            agent.angle
        );
    }
}
