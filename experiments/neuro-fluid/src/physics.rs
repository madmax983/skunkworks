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

pub struct NeuronPole {
    pub id: usize,
    pub pos: Vec2,
    pub is_spiking: bool,
}

pub struct Universe {
    pub particles: Vec<Particle>,
    pub neuron_poles: Vec<NeuronPole>,
    pub network: Network,
    pub width: f64,
    pub height: f64,
    pub platter: Platter,
}

impl Universe {
    pub fn new(width: f64, height: f64) -> Self {
        let mut particles = Vec::new();
        let mut rng = rand::thread_rng();

        // Spawn fluid particles
        for _ in 0..600 {
            particles.push(Particle::new(
                rng.gen_range(width * 0.2..width * 0.8),
                rng.gen_range(height * 0.1..height * 0.5),
            ));
        }

        let grid_w = width as usize + 1;
        let grid_h = height as usize + 1;

        let mut network = Network::new();
        let mut neuron_poles = Vec::new();

        // Create a ring of neurons
        let num_neurons = 20;
        let cx = width / 2.0;
        let cy = height / 2.0;
        let radius = height * 0.3;

        for i in 0..num_neurons {
            let angle = (i as f64 / num_neurons as f64) * std::f64::consts::TAU;
            let nx = cx + angle.cos() * radius;
            let ny = cy + angle.sin() * radius;

            let id = network.add_neuron();
            neuron_poles.push(NeuronPole {
                id,
                pos: Vec2::new(nx, ny),
                is_spiking: false,
            });
        }

        // Connect them in a ring to cause propagating spikes
        for i in 0..num_neurons {
            let next = (i + 1) % num_neurons;
            network.add_synapse_with_delay(i, next, 25.0, 5);
        }

        // Give an initial kick to the first neuron
        network.neurons[0].v = 40.0;

        Self {
            particles,
            neuron_poles,
            network,
            width,
            height,
            platter: Platter::new(grid_w, grid_h),
        }
    }

    pub fn update(&mut self, dt: f64) {
        let gravity = Vec2::new(0.0, -20.0);
        let damping = 0.96;

        // 1. Clear Grid (Platter)
        self.platter.clear();

        // 2. Populate Grid (Density)
        for p in &self.particles {
            let gx = p.pos.x.round() as usize;
            let gy = p.pos.y.round() as usize;
            self.platter.accumulate(gx, gy, 1.0);
        }

        // 3. Update Neural Network
        // The external inputs are based on fluid density near the neuron
        let mut external_inputs = vec![0.0; self.neuron_poles.len()];
        for (i, pole) in self.neuron_poles.iter().enumerate() {
            let gx = pole.pos.x.round() as usize;
            let gy = pole.pos.y.round() as usize;
            let density = self.platter.get(gx, gy);

            // Convert density to a weak input current
            external_inputs[i] = (density as f32) * 0.5;
        }

        self.network.step(&external_inputs);

        // Update the visual/spiking state of poles
        for pole in &mut self.neuron_poles {
            pole.is_spiking = self.network.is_spiking(pole.id);
        }

        // 4. Update Particles
        for i in 0..self.particles.len() {
            let mut force = gravity;

            // Spiking neurons act as temporary magnetic attractors
            for pole in &self.neuron_poles {
                if pole.is_spiking {
                    let delta = pole.pos - self.particles[i].pos;
                    let dist_sq = delta.magnitude_squared();

                    if dist_sq > 1.0 {
                        let dir = delta.normalize();
                        let mag_force = (3000.0 / dist_sq).min(300.0);

                        // Attract
                        force = force + dir * mag_force;
                    }
                }
            }

            // Fluid Pressure (from Platter)
            let p_pos = self.particles[i].pos;
            let gx = p_pos.x.round() as usize;
            let gy = p_pos.y.round() as usize;

            if gx > 0
                && gx < (self.platter.width() - 1)
                && gy > 0
                && gy < (self.platter.height() - 1)
            {
                // Gradient
                let left = self.platter.get(gx - 1, gy);
                let right = self.platter.get(gx + 1, gy);
                let down = self.platter.get(gx, gy - 1);
                let up = self.platter.get(gx, gy + 1);

                let dx = right - left;
                let dy = up - down;

                let pressure_force = Vec2::new(-dx, -dy) * 50.0; // Push away from high density
                force = force + pressure_force;
            }

            self.particles[i].acc = force;
        }

        // 5. Integrate Particles
        for p in &mut self.particles {
            p.vel = p.vel + p.acc * dt;
            p.vel = p.vel * damping;
            p.pos = p.pos + p.vel * dt;

            // Boundaries
            if p.pos.y < 0.0 {
                p.pos.y = 0.0;
                p.vel.y = -p.vel.y * 0.6;
            }
            if p.pos.x < 0.0 {
                p.pos.x = 0.0;
                p.vel.x = -p.vel.x * 0.6;
            }
            if p.pos.x > self.width {
                p.pos.x = self.width;
                p.vel.x = -p.vel.x * 0.6;
            }
            if p.pos.y > self.height {
                p.pos.y = self.height;
                p.vel.y = -p.vel.y * 0.6;
            }
        }
    }
}
