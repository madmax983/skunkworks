use ferrous_core::Platter;
use rand::Rng;
use ratatui::style::Color;
use synaptic_physics::Izhikevich;
pub use locus::Vec2;

#[derive(Debug, Clone)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub mass: f64,
    pub radius: f64,
    pub color: Color,
    pub trail: Vec<Vec2>,
    pub neuron: Izhikevich,
    pub is_spiking: bool,
    pub base_color: Color,
}

impl Body {
    pub fn new(x: f64, y: f64, mass: f64, radius: f64, color: Color) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::zero(),
            acc: Vec2::zero(),
            mass,
            radius,
            color,
            base_color: color,
            trail: Vec::with_capacity(50),
            neuron: Izhikevich::random(&mut rng), // Randomize neuron type
            is_spiking: false,
        }
    }

    pub fn with_velocity(mut self, vx: f64, vy: f64) -> Self {
        self.vel = Vec2::new(vx, vy);
        self
    }
}

pub struct Universe {
    pub bodies: Vec<Body>,
    pub edges: Vec<(usize, usize)>, // Indices into bodies
    pub platter: Platter,
    pub pending_spikes: Vec<(usize, f64)>, // Target ID, Strength
}

impl Universe {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            edges: Vec::new(),
            platter: Platter::new(200, 200), // Fixed grid size covering [-100, 100] with offset
            pending_spikes: Vec::new(),
        }
    }

    pub fn add_body(&mut self, body: Body) -> usize {
        let id = self.bodies.len();
        self.bodies.push(body);
        id
    }

    pub fn add_edge(&mut self, source: usize, target: usize) {
        self.edges.push((source, target));
    }

    pub fn step(&mut self, dt: f64) {
        let len = self.bodies.len();
        let mut forces = vec![Vec2::zero(); len];

        // Constants
        let g_edge = 10000.0;
        let g_repulse = 5000.0;
        let center_pull = 0.5;
        let drag = 0.999;

        // Magnetism constants
        let mag_strength = 2000.0;
        let base_mag_write = 5.0;
        let decay_rate = 0.99; // Per step

        // Distribute pending spikes
        let mut input_currents = vec![0.0; len];
        for (target, strength) in self.pending_spikes.drain(..) {
            if target < len {
                input_currents[target] += strength;
            }
        }

        // Random background noise (thalamic input)
        let mut rng = rand::thread_rng();
        for i in 0..len {
            if rng.gen_bool(0.01) {
                input_currents[i] += 15.0;
            }
        }

        // Collect new spikes
        let mut new_spikes = Vec::new();

        // Update Neurons & Physics Loop
        for i in 0..len {
            // Update Neuron
            // We use a fixed dt for neuron simulation (1.0ms) for stability/predictability
            let (v, spiked) = self.bodies[i].neuron.update(1.0, input_currents[i] as f32);
            self.bodies[i].is_spiking = spiked;

            if spiked {
                self.bodies[i].color = Color::Red; // Visual spike
                new_spikes.push(i);
            } else {
                // Decay color back to base
                // Simple state check: if v > -40.0 (near threshold), look reddish?
                if v > -40.0 {
                    self.bodies[i].color = Color::Magenta;
                } else {
                    self.bodies[i].color = self.bodies[i].base_color;
                }
            }
        }

        // Propagate Spikes along Edges
        for &(source, target) in &self.edges {
            if self.bodies[source].is_spiking {
                // Axonal delay is instant here for simplicity, or we could buffer it
                self.pending_spikes.push((target, 20.0)); // Strong excitatory connection

                // Reciprocal? Maybe weak back-prop
                // self.pending_spikes.push((source, 5.0));
            }
        }

        // 1. Repulsion
        for i in 0..len {
            for j in (i + 1)..len {
                let p1 = self.bodies[i].pos;
                let p2 = self.bodies[j].pos;
                let delta = p2 - p1;
                let dist_sq = delta.magnitude_squared().max(100.0);

                // Spiking neurons repel strongly (Electro-Magnetic Burst)
                let mut repulsion_mult = 1.0;
                if self.bodies[i].is_spiking || self.bodies[j].is_spiking {
                    repulsion_mult = 5.0;
                }

                let force = (g_repulse
                    * repulsion_mult
                    * self.bodies[i].mass.sqrt()
                    * self.bodies[j].mass.sqrt())
                    / dist_sq;
                let dir = delta.normalize();

                let f_vec = -dir * force;

                forces[i] += f_vec;
                forces[j] -= f_vec;
            }
        }

        // 2. Attraction
        for &(i, j) in &self.edges {
            let p1 = self.bodies[i].pos;
            let p2 = self.bodies[j].pos;
            let delta = p2 - p1;
            let dist_sq = delta.magnitude_squared().max(100.0);

            // Spiking neurons pull tighter on their synapses (Hebbian contraction?)
            let mut attract_mult = 1.0;
            if self.bodies[i].is_spiking && self.bodies[j].is_spiking {
                attract_mult = 2.0; // Fire together, Wire together (physically)
            }

            let force =
                (g_edge * attract_mult * self.bodies[i].mass * self.bodies[j].mass) / dist_sq;
            let dir = delta.normalize();

            let f_vec = dir * force;
            forces[i] += f_vec;
            forces[j] -= f_vec;
        }

        // 3. Central Pull, Integration, and Magnetism
        for (i, body) in self.bodies.iter_mut().enumerate() {
            let f_center = -body.pos * center_pull * body.mass;
            forces[i] += f_center;

            // Magnetism Interaction
            // Map pos to grid. [-100, 100] -> [0, 200]
            let gx = (body.pos.x + 100.0) as i32;
            let gy = (body.pos.y + 100.0) as i32;

            if gx >= 0
                && gx < self.platter.width as i32
                && gy >= 0
                && gy < self.platter.height as i32
            {
                let x = gx as usize;
                let y = gy as usize;

                // Write to platter
                // Spiking neurons write MUCH stronger magnetism
                let mag_mult = if body.is_spiking { 10.0 } else { 1.0 };
                self.platter.magnetize(x, y, base_mag_write * mag_mult * dt);

                // Read from platter (Gradient)
                let left = if x > 0 {
                    self.platter.get_magnetism(x - 1, y)
                } else {
                    0.0
                };
                let right = if x < self.platter.width - 1 {
                    self.platter.get_magnetism(x + 1, y)
                } else {
                    0.0
                };
                let down = if y > 0 {
                    self.platter.get_magnetism(x, y - 1)
                } else {
                    0.0
                };
                let up = if y < self.platter.height - 1 {
                    self.platter.get_magnetism(x, y + 1)
                } else {
                    0.0
                };

                let grad_x = (right - left) * 0.5;
                let grad_y = (up - down) * 0.5;

                // Spiking neurons interact stronger with the field
                let mag_force = Vec2::new(grad_x, grad_y) * mag_strength * mag_mult;
                forces[i] += mag_force;
            }

            body.acc = forces[i] / body.mass;

            body.vel += body.acc * dt;
            body.vel *= drag;
            body.pos += body.vel * dt;

            // Trail Logic
            if rand::random::<u8>() % 10 == 0 {
                body.trail.push(body.pos);
                if body.trail.len() > 20 {
                    body.trail.remove(0);
                }
            }
        }

        // 4. Platter Decay
        self.platter.decay(decay_rate);
    }
}
