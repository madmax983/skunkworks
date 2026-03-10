use macroquad::prelude::*;

pub const WORLD_SIZE: f32 = 1000.0;

pub struct World {
    pub firewalls: Vec<Firewall>,
}

pub struct Firewall {
    pub position: Vec2,
    pub radius: f32,
    pub repulsion_strength: f32,
}

impl World {
    pub fn new() -> Self {
        Self {
            firewalls: Vec::new(),
        }
    }

    pub fn add_firewall(&mut self, x: f32, y: f32) {
        self.firewalls.push(Firewall {
            position: Vec2::new(x, y),
            radius: 100.0,
            repulsion_strength: 5000.0,
        });
    }

    pub fn clear_firewalls(&mut self) {
        self.firewalls.clear();
    }

    pub fn update(&mut self, particles: &mut super::physics::ParticleSystem, server_pos: Vec2) {
        // Simple absorption logic
        let server_radius = 50.0;
        let mut absorbed = 0;

        // Remove particles that got too close to the server (successful packets)
        particles.particles.retain(|p| {
            if (p.position - server_pos).length_squared() < server_radius * server_radius {
                absorbed += 1;
                false // removed
            } else {
                true // kept
            }
        });

        // Optionally use `absorbed` to change server state (e.g. overheating)
        // ...
    }
}
