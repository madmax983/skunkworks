use crate::neuron::Izhikevich;
use ::rand::Rng;
use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BeeState {
    Wandering,
    Dancing,    // Spiking
    Refractory, // Recovering
}

#[derive(Clone, Copy, Debug)]
pub struct Bee {
    pub position: Vec2,
    pub velocity: Vec2,
    pub state: BeeState,
    pub neuron: Izhikevich,
    pub dance_timer: f32,
    pub dance_angle: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct StimulusSource {
    pub position: Vec2,
    pub strength: f32, // Input current magnitude
    pub radius: f32,
}

impl Bee {
    pub fn new(position: Vec2, rng: &mut impl Rng) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            state: BeeState::Wandering,
            neuron: Izhikevich::random(rng),
            dance_timer: 0.0,
            dance_angle: 0.0,
        }
    }

    pub fn update(
        &mut self,
        sources: &[StimulusSource],
        nearby_dancers: usize, // Number of bees dancing nearby
        bounds: Vec2,
        tick: u64,
    ) {
        let mut rng = ::rand::thread_rng();
        let max_speed = 2.0;
        let dt = 1.0; // 1ms time step

        // Physics Update (Brownian Motion + Boundary)
        match self.state {
            BeeState::Wandering | BeeState::Refractory => {
                self.velocity += vec2(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5));
                if self.velocity.length() > max_speed {
                    self.velocity = self.velocity.normalize() * max_speed;
                }
                self.position += self.velocity;

                // Keep in bounds
                if self.position.x < 0.0 {
                    self.position.x = 0.0;
                    self.velocity.x *= -1.0;
                }
                if self.position.x > bounds.x {
                    self.position.x = bounds.x;
                    self.velocity.x *= -1.0;
                }
                if self.position.y < 0.0 {
                    self.position.y = 0.0;
                    self.velocity.y *= -1.0;
                }
                if self.position.y > bounds.y {
                    self.position.y = bounds.y;
                    self.velocity.y *= -1.0;
                }
            }
            BeeState::Dancing => {
                // Jitter in place
                self.position += vec2(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
            }
        }

        // Neuron Update
        let mut input_current = 0.0;

        // 1. Base noise
        input_current += rng.gen_range(-2.0..2.0);

        // 2. Stimulus Sources
        for source in sources {
            let dist = self.position.distance(source.position);
            if dist < source.radius {
                // Inverse square law or linear? Let's go linear falloff within radius
                let intensity = (1.0 - dist / source.radius).max(0.0);
                input_current += source.strength * intensity;
            }
        }

        // 3. Synaptic Input (from nearby dancers)
        // Each dancer adds some current
        let synaptic_weight = 5.0;
        input_current += nearby_dancers as f32 * synaptic_weight;

        // Update Izhikevich Model
        let spiked = self.neuron.update(dt, input_current, tick);

        // State Transitions
        match self.state {
            BeeState::Wandering => {
                if spiked {
                    self.state = BeeState::Dancing;
                    self.dance_timer = 10.0; // Dance for 10 ticks
                    self.dance_angle = rng.gen_range(0.0..std::f32::consts::TAU);
                }
            }
            BeeState::Dancing => {
                self.dance_timer -= 1.0;
                if self.dance_timer <= 0.0 {
                    self.state = BeeState::Refractory;
                    self.dance_timer = 20.0; // Refractory period
                }
            }
            BeeState::Refractory => {
                self.dance_timer -= 1.0;
                if self.dance_timer <= 0.0 {
                    self.state = BeeState::Wandering;
                }
            }
        }
    }
}
