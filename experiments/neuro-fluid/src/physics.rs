use ferrous_core::Platter;
use locus::Vec2;
use neuro_sim::Network;
use rand::Rng;

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
}

impl Particle {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::zero(),
            acc: Vec2::zero(),
        }
    }
}

pub struct NeuroMagnet {
    pub pos: Vec2,
    pub neuron_id: usize,
    pub base_strength: f64,
    pub polarity: bool, // true = North (Pull), false = South (Push/Complex)
}

pub struct Universe {
    pub particles: Vec<Particle>,
    pub magnets: Vec<NeuroMagnet>,
    pub network: Network,
    pub width: f64,
    pub height: f64,
    pub grid: Platter, // Reusing Ferrous Core's Platter for density/pressure
}

impl Universe {
    pub fn new(width: f64, height: f64) -> Self {
        let mut rng = rand::thread_rng();
        let mut particles = Vec::with_capacity(1000);
        for _ in 0..1000 {
            particles.push(Particle::new(
                rng.gen_range(0.0..width),
                rng.gen_range(0.0..height),
            ));
        }

        let mut network = Network::new();
        let mut magnets = Vec::new();

        // Create a central pattern generator (ring oscillator) of neurons
        let num_neurons = 5;
        let mut neurons = vec![];
        for _ in 0..num_neurons {
            neurons.push(network.add_neuron());
        }

        // Connect them in a ring to create rhythm
        for i in 0..num_neurons {
            let next = (i + 1) % num_neurons;
            // Excitatory connection
            network.add_synapse(neurons[i], neurons[next], 15.0);
        }

        // Add corresponding magnets for each neuron, arranged in a circle
        let center = Vec2::new(width / 2.0, height / 2.0);
        let radius = 30.0;
        for i in 0..num_neurons {
            let angle = (i as f64 / num_neurons as f64) * std::f64::consts::TAU;
            let pos = center + Vec2::new(angle.cos() * radius, angle.sin() * radius);
            magnets.push(NeuroMagnet {
                pos,
                neuron_id: neurons[i],
                base_strength: 50.0,
                polarity: i % 2 == 0,
            });
        }

        // Kickstart the network
        network.neurons[neurons[0]].v = 40.0; // Force a spike

        Self {
            particles,
            magnets,
            network,
            width,
            height,
            grid: Platter::new(width as usize, height as usize),
        }
    }

    pub fn tick(&mut self) {
        // Update Neural Network
        let inputs = vec![0.0; self.network.neurons.len()];
        self.network.step(&inputs);

        // Reset grid
        self.grid.clear();

        // 1. Scatter particles to grid to calculate density
        for p in &self.particles {
            if p.pos.x >= 0.0 && p.pos.x < self.width && p.pos.y >= 0.0 && p.pos.y < self.height {
                self.grid.accumulate(p.pos.x as usize, p.pos.y as usize, 1.0);
            }
        }

        // 2. Calculate forces
        for i in 0..self.particles.len() {
            let pos = self.particles[i].pos;
            let mut total_force = Vec2::zero();

            // Gravity
            total_force.y += 0.05;

            // Neuro-Magnetic Forces
            for magnet in &self.magnets {
                let dist_sq = pos.distance_squared(magnet.pos);
                if dist_sq < 10000.0 && dist_sq > 1.0 { // Cap range and singularity
                    let dist = dist_sq.sqrt();
                    let dir = (magnet.pos - pos) / dist;

                    // Base force
                    let mut force_mag = magnet.base_strength / dist;

                    // Spike modulation (act like an electromagnet that pulses)
                    if self.network.is_spiking(magnet.neuron_id) {
                        force_mag *= 5.0; // Huge pulse of force on spike
                    }

                    if magnet.polarity {
                        total_force += dir * force_mag; // Pull
                    } else {
                        // Push, but with a slight twist for chaos
                        total_force -= dir * force_mag;
                        total_force += Vec2::new(-dir.y, dir.x) * (force_mag * 0.2);
                    }
                }
            }

            // Grid Pressure (Fluid Dynamics)
            if pos.x >= 1.0 && pos.x < self.width - 1.0 && pos.y >= 1.0 && pos.y < self.height - 1.0 {
                let px = pos.x as usize;
                let py = pos.y as usize;

                let right = self.grid.get(px + 1, py);
                let left = self.grid.get(px - 1, py);
                let down = self.grid.get(px, py + 1);
                let up = self.grid.get(px, py - 1);

                // Gradient points towards lower density
                let pressure_grad = Vec2::new(left - right, up - down);
                total_force += pressure_grad * 0.1;
            }

            self.particles[i].acc = total_force;
        }

        // 3. Integrate & Enforce Bounds
        let damping = 0.95;
        for p in &mut self.particles {
            p.vel += p.acc;
            p.vel = p.vel * damping;
            p.pos += p.vel;

            // Bounce off walls
            if p.pos.x < 0.0 {
                p.pos.x = 0.0;
                p.vel.x *= -0.5;
            } else if p.pos.x >= self.width {
                p.pos.x = self.width - 0.1;
                p.vel.x *= -0.5;
            }

            if p.pos.y < 0.0 {
                p.pos.y = 0.0;
                p.vel.y *= -0.5;
            } else if p.pos.y >= self.height {
                p.pos.y = self.height - 0.1;
                p.vel.y *= -0.5;
            }
        }
    }
}
