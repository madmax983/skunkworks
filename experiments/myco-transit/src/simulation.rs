use rand::prelude::*;
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Agent {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
}

impl Agent {
    pub fn new(x: f64, y: f64, angle: f64) -> Self {
        Self { x, y, angle }
    }

    pub fn sense(&self, world: &World, angle_offset: f64, sensor_dist: f64) -> f64 {
        let sensor_angle = self.angle + angle_offset;
        let sensor_x = self.x + sensor_angle.cos() * sensor_dist;
        let sensor_y = self.y + sensor_angle.sin() * sensor_dist;

        // Wrap coords for sensing
        let w = world.width as f64;
        let h = world.height as f64;
        let sx = (sensor_x.rem_euclid(w)) as usize;
        let sy = (sensor_y.rem_euclid(h)) as usize;

        world.get_trail(sx, sy)
    }

    pub fn update(&mut self, world: &mut World) {
        let sensor_angle = PI / 4.0;
        let sensor_dist = 9.0;
        let turn_angle = PI / 8.0; // Rotation Angle (RA)
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
        let current = world.get_trail(ix, iy);
        // Cap at some value to prevent explosion, though decay handles it
        world.set_trail(ix, iy, (current + 5.0).min(255.0));
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
        for _ in 0..num_cities {
            cities.push((
                rng.gen_range(0.0..width as f64),
                rng.gen_range(0.0..height as f64),
            ));
        }

        let mut agents = Vec::new();
        let agents_per_city = 100; // Total agents = num_cities * 100

        for (cx, cy) in &cities {
            for _ in 0..agents_per_city {
                agents.push(Agent::new(*cx, *cy, rng.gen_range(0.0..2.0 * PI)));
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

    pub fn diffuse_and_decay(&mut self) {
        let decay_factor = 0.9;

        for y in 0..self.height {
            for x in 0..self.width {
                // 3x3 box blur
                let mut sum = 0.0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = (x as isize + dx).rem_euclid(self.width as isize) as usize;
                        let ny = (y as isize + dy).rem_euclid(self.height as isize) as usize;
                        sum += self.trails[ny * self.width + nx];
                    }
                }
                let avg = sum / 9.0;
                self.next_trails[y * self.width + x] = avg * decay_factor;
            }
        }
        std::mem::swap(&mut self.trails, &mut self.next_trails);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_senses_gradient() {
        let width = 100;
        let height = 100;
        let mut world = World::new(width, height);

        // Create a vertical gradient increasing towards the right (positive x)
        for x in 0..width {
            for y in 0..height {
                world.set_trail(x, y, x as f64); // Linear gradient 0..99
            }
        }

        // Place agent in middle (50, 50), facing Up (negative y, -PI/2).
        // To the left (x < 50) is lower val.
        // To the right (x > 50) is higher val.
        // Forward is (50, 49) -> val 50
        // Left sensor (relative to agent) checks towards x < 50.
        // Right sensor (relative to agent) checks towards x > 50.
        // Agent should turn Right (increase angle from -PI/2 towards 0).

        let mut agent = Agent::new(50.0, 50.0, -PI / 2.0);
        let initial_angle = agent.angle;

        // One update might not be enough if sensors land on same x column,
        // but given sensor_dist=9 and angle=PI/4, they should sample distinct X.
        // Left sensor: (-PI/2 - PI/4) = -3PI/4. cos = -0.7. x approx 50 - 6.3 = 43.7. Val ~ 43.
        // Right sensor: (-PI/2 + PI/4) = -PI/4. cos = 0.7. x approx 50 + 6.3 = 56.3. Val ~ 56.
        // Right > Left. Should turn Right (+ angle).

        agent.update(&mut world);

        // Check if angle changed towards 0 (right)
        // Since it starts at -PI/2 (-1.57), turning right means increasing angle (e.g. -1.17).
        assert!(
            agent.angle > initial_angle,
            "Agent should turn towards higher concentration (Right). Init: {}, New: {}",
            initial_angle,
            agent.angle
        );
    }

    #[test]
    fn test_pheromone_decay() {
        let mut world = World::new(10, 10);
        // Set center to 100
        world.set_trail(5, 5, 100.0);

        world.diffuse_and_decay();

        let value = world.get_trail(5, 5);
        assert!(
            value < 100.0,
            "Pheromone should decay. Was 100, now {}",
            value
        );

        // Also check diffusion happened (neighbor should be > 0)
        let neighbor = world.get_trail(5, 6);
        // 100/9 * 0.9 = 10
        assert!(
            neighbor > 0.0,
            "Pheromone should diffuse to neighbors. Got {}",
            neighbor
        );
    }
}
