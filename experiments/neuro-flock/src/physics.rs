//! 🧬 Splice: Cross `neuro-sim` × `flocking`
//!
//! This module implements Neural Swarming. The continuous physical flocking
//! algorithms are bridged with a Spiking Neural Network (SNN).
//! The global flock dynamics affect the inputs to the neural network,
//! and the neural network's spiking output dynamically controls the global
//! swarming weights (alignment, cohesion, separation).

use flocking::{compute_force, FlockingParams};
use locus::Vec2;
use neuro_sim::Network;
use rand::Rng;

pub struct Boid {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl Boid {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::zero(),
        }
    }
}

pub struct NeuronPole {
    pub id: usize,
    pub pos: Vec2,
    pub is_spiking: bool,
}

pub struct Universe {
    pub boids: Vec<Boid>,
    pub neuron_poles: Vec<NeuronPole>,
    pub network: Network,
    pub width: f64,
    pub height: f64,
    pub flocking_params: FlockingParams,
}

impl Universe {
    pub fn new(width: f64, height: f64) -> Self {
        let mut boids = Vec::new();
        let mut rng = rand::thread_rng();

        // Spawn flocking boids
        for _ in 0..150 {
            boids.push(Boid::new(
                rng.gen_range(0.0..width),
                rng.gen_range(0.0..height),
            ));
        }

        let mut network = Network::new();
        let mut neuron_poles = Vec::new();

        // Create a central cluster of neurons
        let num_neurons = 10;
        let cx = width / 2.0;
        let cy = height / 2.0;
        let radius = height * 0.15;

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

        // Connect them randomly with delays to cause unpredictable cascading spikes
        for i in 0..num_neurons {
            let targets = rand::seq::index::sample(&mut rng, num_neurons, 3);
            for t in targets {
                if i != t {
                    let weight = rng.gen_range(5.0..25.0);
                    let delay = rng.gen_range(0..10);
                    network.add_synapse_with_delay(i, t, weight as f32, delay);
                }
            }
        }

        let flocking_params = FlockingParams {
            view_radius: 30.0,
            separation_radius: 10.0,
            max_speed: 4.0,
            max_force: 0.15,
            separation_weight: 1.5,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
        };

        Self {
            boids,
            neuron_poles,
            network,
            width,
            height,
            flocking_params,
        }
    }

    pub fn update(&mut self) {
        // 1. Compute Flocking Inputs to Neural Network
        // The average speed/cohesion of the boids acts as a sensory input to the neurons
        let mut avg_vel = Vec2::zero();
        for boid in &self.boids {
            avg_vel += boid.vel;
        }

        let avg_speed = if self.boids.is_empty() {
            0.0
        } else {
            (avg_vel.magnitude() / self.boids.len() as f64) as f32
        };

        // Base external input driven by flocking activity
        let mut external_inputs = vec![avg_speed * 0.5; self.neuron_poles.len()];

        // Randomly perturb neurons to keep activity alive
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.1) {
            let random_idx = rng.gen_range(0..self.neuron_poles.len());
            external_inputs[random_idx] += 30.0;
        }

        // 2. Update Neural Network
        self.network.step(&external_inputs);

        // Track spiking state
        let mut total_spikes = 0;
        for pole in &mut self.neuron_poles {
            pole.is_spiking = self.network.is_spiking(pole.id);
            if pole.is_spiking {
                total_spikes += 1;
            }
        }

        // 3. Spiking Activity modulates Flocking Parameters (Magneto-Neural Symbiosis analogue)
        // High spiking activity increases alignment (highly synchronized panic/swarm)
        // Low spiking activity increases separation (foraging/relaxed)
        let spike_ratio = total_spikes as f64 / self.neuron_poles.len().max(1) as f64;

        // Smooth transition of parameters
        let target_alignment = 0.5 + spike_ratio * 3.0;
        let target_separation = 2.0 - spike_ratio * 1.5;
        let target_cohesion = 1.0 + spike_ratio * 1.0;

        self.flocking_params.alignment_weight += (target_alignment - self.flocking_params.alignment_weight) * 0.1;
        self.flocking_params.separation_weight += (target_separation - self.flocking_params.separation_weight) * 0.1;
        self.flocking_params.cohesion_weight += (target_cohesion - self.flocking_params.cohesion_weight) * 0.1;

        // 4. Update Boids
        let positions: Vec<Vec2> = self.boids.iter().map(|b| b.pos).collect();
        let velocities: Vec<Vec2> = self.boids.iter().map(|b| b.vel).collect();

        for i in 0..self.boids.len() {
            let steering = compute_force(&positions, &velocities, i, &self.flocking_params);

            let mut vel = self.boids[i].vel + steering;
            vel = vel.limit(self.flocking_params.max_speed);
            self.boids[i].vel = vel;

            let mut pos = self.boids[i].pos + vel;

            // Screen Wrap
            if pos.x < 0.0 { pos.x = self.width; }
            if pos.x > self.width { pos.x = 0.0; }
            if pos.y < 0.0 { pos.y = self.height; }
            if pos.y > self.height { pos.y = 0.0; }

            self.boids[i].pos = pos;
        }
    }
}
