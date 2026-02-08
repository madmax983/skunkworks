use macroquad::prelude::*;
use rayon::prelude::*;
use ::rand::Rng;

#[derive(Clone, Copy)]
pub struct Server {
    pub position: Vec2,
    pub health: f32,
    pub active: bool,
}

pub struct Locust {
    pub position: Vec2,
    pub velocity: Vec2,
    pub target: Option<usize>,
}

impl Locust {
    pub fn new(x: f32, y: f32, target: Option<usize>) -> Self {
        Self {
            position: vec2(x, y),
            velocity: vec2(0.0, 0.0),
            target,
        }
    }

    pub fn update(&mut self, dt: f32, servers: &[Server]) -> Option<usize> {
        let mut seeking = false;
        let mut hit = None;
        if let Some(target_idx) = self.target {
            if let Some(server) = servers.get(target_idx) {
                if server.active {
                    let diff = server.position - self.position;
                    let dist_sq = diff.length_squared();

                    if dist_sq < 100.0 {
                        // Reached target, apply damage
                        self.velocity *= 0.5;
                        hit = Some(target_idx);
                    } else {
                        let dir = diff.normalize_or_zero();
                        // Simple steering: accelerate towards target
                        self.velocity += dir * 200.0 * dt;
                    }
                    seeking = true;
                }
            }
        }

        if !seeking {
             // Wander behavior if no target or target dead
             // Random perturbation
             let mut rng = ::rand::thread_rng();
             let angle = rng.gen_range(0.0..std::f32::consts::TAU);
             let force = vec2(angle.cos(), angle.sin()) * 50.0 * dt;
             self.velocity += force;
        }

        // Drag
        self.velocity *= 0.98;

        // Move
        self.position += self.velocity * dt;

        hit
    }
}

pub struct World {
    pub width: f32,
    pub height: f32,
    pub locusts: Vec<Locust>,
    pub servers: Vec<Server>,
}

impl World {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            locusts: Vec::new(),
            servers: Vec::new(),
        }
    }

    pub fn add_server(&mut self, x: f32, y: f32) -> usize {
        self.servers.push(Server {
            position: vec2(x, y),
            health: 100.0,
            active: true,
        });
        self.servers.len() - 1
    }

    pub fn add_locust(&mut self, x: f32, y: f32, target: Option<usize>) {
        self.locusts.push(Locust::new(x, y, target));
    }

    pub fn get_locust(&self, index: usize) -> &Locust {
        &self.locusts[index]
    }

    pub fn update(&mut self, dt: f32) {
        // Parallel update of locusts
        // We need to pass a read-only view of servers to the locusts
        let servers = &self.servers;

        // Collect damage events
        let hits: Vec<usize> = self.locusts.par_iter_mut()
            .filter_map(|locust| locust.update(dt, servers))
            .collect();

        // Apply damage sequentially
        for server_idx in hits {
             if let Some(server) = self.servers.get_mut(server_idx) {
                 if server.active {
                     server.health -= 0.1; // Small damage per tick per locust
                     if server.health <= 0.0 {
                         server.health = 0.0;
                         server.active = false;
                     }
                 }
             }
        }
    }
}
