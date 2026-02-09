pub use locus::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PacketKind {
    Http,    // Green
    Ssh,     // Blue
    Malware, // Red
}

#[derive(Clone, Debug)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub kind: PacketKind,
    pub radius: f64,
    pub active: bool,
}

impl Particle {
    pub fn new(x: f64, y: f64, kind: PacketKind) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::new(0.0, 0.0),
            kind,
            radius: 0.5,
            active: true,
        }
    }

    pub fn update(&mut self, dt: f64, gravity: Vec2) {
        if !self.active {
            return;
        }
        self.vel = self.vel + gravity * dt;
        self.pos = self.pos + self.vel * dt;
        self.vel = self.vel * 0.99; // Friction
    }
}

#[derive(Clone, Debug)]
pub struct NeuronPin {
    pub pos: Vec2,
    pub radius: f64,
    pub neuron_index: usize,
}

impl NeuronPin {
    pub fn new(x: f64, y: f64, neuron_index: usize) -> Self {
        Self {
            pos: Vec2::new(x, y),
            radius: 1.0,
            neuron_index,
        }
    }
}

/// Resolves collision between particle and pin.
/// Returns Some(neuron_index) if collision occurred.
pub fn resolve_collision(particle: &mut Particle, pin: &NeuronPin) -> Option<usize> {
    let diff = particle.pos - pin.pos;
    let dist_sq = diff.magnitude_squared();
    let min_dist = particle.radius + pin.radius;

    if dist_sq < min_dist * min_dist {
        // Collision!
        let dist = dist_sq.sqrt();
        let normal = if dist == 0.0 {
            Vec2::new(0.0, 1.0)
        } else {
            diff * (1.0 / dist)
        };

        let overlap = min_dist - dist;
        particle.pos = particle.pos + normal * overlap;

        let restitution = 0.8;
        let v_dot_n = particle.vel.dot(normal);

        if v_dot_n < 0.0 {
            let j = -(1.0 + restitution) * v_dot_n;
            particle.vel = particle.vel + normal * j;
            return Some(pin.neuron_index);
        }
    }
    None
}
