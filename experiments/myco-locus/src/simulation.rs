use locus::{Topology, Vec2};
use rand::Rng;
use rayon::prelude::*;
use std::f64::consts::PI;

pub struct Agent {
    pub position: Vec2,
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
            position: Vec2::new(x, y),
            angle,
            home_city,
            target_city,
            commuted_count: 0,
        }
    }

    /// Sense using Topology normalization
    pub fn sense(&self, world: &World, angle_offset: f64, sensor_dist: f64) -> f64 {
        let sensor_angle = self.angle + angle_offset;
        let sensor_x = self.position.x + sensor_angle.cos() * sensor_dist;
        let sensor_y = self.position.y + sensor_angle.sin() * sensor_dist;

        let w = world.width;
        let h = world.height;
        let mut trail_strength = 0.0;

        if let Some((ny, nx)) = world.topology.normalize(sensor_y.round() as i64, sensor_x.round() as i64, w, h) {
            trail_strength = world.get_trail(nx, ny);
        }

        // Distance to target, ignoring boundary wrapping for simplicity
        let target_pos = world.cities[self.target_city];
        let dx = target_pos.x - sensor_x;
        let dy = target_pos.y - sensor_y;
        let dist = (dx * dx + dy * dy).sqrt().max(1.0);

        let gradient_strength = 2000.0 / dist;

        trail_strength + gradient_strength
    }

    pub fn update(&mut self, world: &World) -> Option<AgentUpdateResult> {
        let target_pos = world.cities[self.target_city];
        let dx = target_pos.x - self.position.x;
        let dy = target_pos.y - self.position.y;
        let dist_sq = dx * dx + dy * dy;

        if dist_sq < 25.0 {
            std::mem::swap(&mut self.home_city, &mut self.target_city);
            self.angle += PI;
            self.commuted_count += 1;
            return None;
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

        self.position.x += self.angle.cos() * speed;
        self.position.y += self.angle.sin() * speed;

        if let Some((ny, nx)) = world.topology.normalize(self.position.y.round() as i64, self.position.x.round() as i64, world.width, world.height) {
            self.position.x = nx as f64;
            self.position.y = ny as f64;
            Some(AgentUpdateResult {
                deposit_x: nx,
                deposit_y: ny,
                deposit_amount: 5.0,
            })
        } else {
            // Agent died off bounds
            None
        }
    }
}

pub struct World {
    pub width: usize,
    pub height: usize,
    pub trails: Vec<f64>,
    pub next_trails: Vec<f64>,
    pub cities: Vec<Vec2>,
    pub topology: Topology,
}

impl World {
    pub fn new(width: usize, height: usize, topology: Topology) -> Self {
        Self {
            width,
            height,
            trails: vec![0.0; width * height],
            next_trails: vec![0.0; width * height],
            cities: Vec::new(),
            topology,
        }
    }

    pub fn with_cities_and_agents(
        width: usize,
        height: usize,
        num_cities: usize,
        topology: Topology,
    ) -> (Self, Vec<Agent>) {
        let mut rng = rand::thread_rng();
        let mut cities = Vec::new();
        let padding = 10.0;
        for _ in 0..num_cities {
            cities.push(Vec2::new(
                rng.gen_range(padding..(width as f64 - padding)),
                rng.gen_range(padding..(height as f64 - padding)),
            ));
        }

        let mut agents = Vec::new();
        let agents_per_city = 1250;

        for (i, c) in cities.iter().enumerate() {
            for _ in 0..agents_per_city {
                let mut target = rng.gen_range(0..num_cities);
                if num_cities > 1 {
                    while target == i {
                        target = rng.gen_range(0..num_cities);
                    }
                }
                agents.push(Agent::new(c.x, c.y, rng.gen_range(0.0..2.0 * PI), i, target));
            }
        }

        let mut world = Self::new(width, height, topology);
        world.cities = cities;
        (world, agents)
    }

    pub fn get_trail(&self, x: usize, y: usize) -> f64 {
        if x < self.width && y < self.height {
            self.trails[y * self.width + x]
        } else {
            0.0
        }
    }

    pub fn update_agents_parallel(&mut self, agents: &mut [Agent]) {
        let deposits: Vec<AgentUpdateResult> = agents
            .par_iter_mut()
            .filter_map(|agent| agent.update(self))
            .collect();

        for deposit in deposits {
            let idx = deposit.deposit_y * self.width + deposit.deposit_x;
            self.trails[idx] = (self.trails[idx] + deposit.deposit_amount).min(255.0);
        }
    }

    pub fn diffuse_and_decay(&mut self) {
        let decay_factor = 0.9;
        let width = self.width;
        let height = self.height;
        let topo = self.topology;
        let trails_ref = &self.trails;

        self.next_trails
            .par_chunks_mut(width)
            .enumerate()
            .for_each(|(y, row)| {
                for (x, pixel) in row.iter_mut().enumerate() {
                    let mut sum = 0.0;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let tx = x as i64 + dx;
                            let ty = y as i64 + dy;
                            if let Some((ny, nx)) = topo.normalize(ty, tx, width, height) {
                                sum += trails_ref[ny * width + nx];
                            }
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
        let mut world = World::new(width, height, Topology::Plane);

        world.cities.push(Vec2::new(10.0, 10.0));
        world.cities.push(Vec2::new(90.0, 90.0));

        let mut agent = Agent::new(10.0, 10.0, 0.0, 0, 1);
        agent.position.x = 89.0;
        agent.position.y = 89.0;

        agent.update(&world);

        assert_eq!(agent.home_city, 1);
        assert_eq!(agent.target_city, 0);
        assert_eq!(agent.commuted_count, 1);

        assert!(
            (agent.angle - PI).abs() < 0.001 || (agent.angle + PI).abs() < 0.001,
            "Angle should flip"
        );
    }
}
