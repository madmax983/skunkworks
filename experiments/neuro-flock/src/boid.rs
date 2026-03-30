use locus::Vec2;
use rand::Rng;
use ratatui::style::Color;
use std::f64::consts::TAU;

use neuro_sim::Network;

#[derive(Clone, Debug)]
pub struct Dna {
    pub max_speed: f64,
    pub view_radius: f64,
    pub color: Color,
    pub char_representation: char,
}

impl Dna {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            max_speed: rng.gen_range(0.8..1.5),
            view_radius: rng.gen_range(10.0..20.0),
            color: Color::Indexed(rng.gen_range(20..230)),
            char_representation: if rng.gen_bool(0.5) { '✦' } else { '•' },
        }
    }
}

pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub dna: Dna,
    pub color: Color,
    pub char_representation: char,
    pub phase: f64, // Used for flash timing if needed

    // Neural Network
    pub brain: Network,
    pub sensory_neurons: Vec<usize>, // Input nodes
    pub motor_neurons: Vec<usize>,   // Output nodes
}

impl Boid {
    pub fn new(x: f64, y: f64) -> Self {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..TAU);
        let dna = Dna::random();

        let mut brain = Network::new();
        // Setup simple neural brain. We need positive and negative X, Y for each component
        let s_align_x_pos = brain.add_neuron();
        let s_align_x_neg = brain.add_neuron();
        let s_align_y_pos = brain.add_neuron();
        let s_align_y_neg = brain.add_neuron();

        let s_cohere_x_pos = brain.add_neuron();
        let s_cohere_x_neg = brain.add_neuron();
        let s_cohere_y_pos = brain.add_neuron();
        let s_cohere_y_neg = brain.add_neuron();

        let s_separate_x_pos = brain.add_neuron();
        let s_separate_x_neg = brain.add_neuron();
        let s_separate_y_pos = brain.add_neuron();
        let s_separate_y_neg = brain.add_neuron();

        let m_x_pos = brain.add_neuron();
        let m_x_neg = brain.add_neuron();
        let m_y_pos = brain.add_neuron();
        let m_y_neg = brain.add_neuron();

        // Connect sensory to motor. We want alignment X pos to drive motor X pos, etc.
        let weight = 20.0;

        brain.add_synapse_with_delay(s_align_x_pos, m_x_pos, weight, rng.gen_range(0..5));
        brain.add_synapse_with_delay(s_align_x_neg, m_x_neg, weight, rng.gen_range(0..5));
        brain.add_synapse_with_delay(s_align_y_pos, m_y_pos, weight, rng.gen_range(0..5));
        brain.add_synapse_with_delay(s_align_y_neg, m_y_neg, weight, rng.gen_range(0..5));

        brain.add_synapse_with_delay(s_cohere_x_pos, m_x_pos, weight, rng.gen_range(0..5));
        brain.add_synapse_with_delay(s_cohere_x_neg, m_x_neg, weight, rng.gen_range(0..5));
        brain.add_synapse_with_delay(s_cohere_y_pos, m_y_pos, weight, rng.gen_range(0..5));
        brain.add_synapse_with_delay(s_cohere_y_neg, m_y_neg, weight, rng.gen_range(0..5));

        // Separation pushes away, so it should be heavily weighted
        let sep_weight = 40.0;
        brain.add_synapse_with_delay(s_separate_x_pos, m_x_pos, sep_weight, rng.gen_range(0..5));
        brain.add_synapse_with_delay(s_separate_x_neg, m_x_neg, sep_weight, rng.gen_range(0..5));
        brain.add_synapse_with_delay(s_separate_y_pos, m_y_pos, sep_weight, rng.gen_range(0..5));
        brain.add_synapse_with_delay(s_separate_y_neg, m_y_neg, sep_weight, rng.gen_range(0..5));

        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::new(angle.cos() * dna.max_speed, angle.sin() * dna.max_speed),
            acceleration: Vec2::zero(),
            color: dna.color,
            char_representation: dna.char_representation,
            dna,
            phase: rng.r#gen::<f64>(),
            brain,
            sensory_neurons: vec![
                s_align_x_pos, s_align_x_neg, s_align_y_pos, s_align_y_neg,
                s_cohere_x_pos, s_cohere_x_neg, s_cohere_y_pos, s_cohere_y_neg,
                s_separate_x_pos, s_separate_x_neg, s_separate_y_pos, s_separate_y_neg,
            ],
            motor_neurons: vec![m_x_pos, m_x_neg, m_y_pos, m_y_neg],
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    pub fn update_physics(&mut self, width: f64, height: f64) {
        self.velocity += self.acceleration;
        self.velocity = self.velocity.limit(self.dna.max_speed);
        self.position += self.velocity;
        self.acceleration = Vec2::zero();

        // Wrap around edges
        if self.position.x < 0.0 {
            self.position.x += width;
        }
        if self.position.x >= width {
            self.position.x -= width;
        }
        if self.position.y < 0.0 {
            self.position.y += height;
        }
        if self.position.y >= height {
            self.position.y -= height;
        }
    }
}
