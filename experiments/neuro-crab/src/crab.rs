use crate::network::Network;
use rand::Rng;
use std::f32::consts::PI;

pub struct Muscle {
    pub activation: f32,
    pub decay: f32,
    pub strength: f32,
}

impl Muscle {
    pub fn new() -> Self {
        Self {
            activation: 0.0,
            decay: 0.90, // Fast decay for responsiveness
            strength: 0.3,
        }
    }
    pub fn update(&mut self, spike: bool) {
        if spike {
            self.activation += self.strength;
        }
        self.activation *= self.decay;
        if self.activation > 1.0 { self.activation = 1.0; }
    }
}

pub struct Leg {
    pub angle: f32,
    pub target_angle: f32,
    pub flexor_idx: usize,
    pub extensor_idx: usize,
    pub flexor_muscle: Muscle,
    pub extensor_muscle: Muscle,
    pub base_angle: f32,
    pub range: f32,
}

impl Leg {
    pub fn new(flexor_idx: usize, extensor_idx: usize, base_angle: f32) -> Self {
        Self {
            angle: base_angle,
            target_angle: base_angle,
            flexor_idx,
            extensor_idx,
            flexor_muscle: Muscle::new(),
            extensor_muscle: Muscle::new(),
            base_angle,
            range: 0.8, // radians swing amplitude
        }
    }

    pub fn update(&mut self, network: &Network) {
        let f_spike = network.is_spiking(self.flexor_idx);
        let e_spike = network.is_spiking(self.extensor_idx);

        self.flexor_muscle.update(f_spike);
        self.extensor_muscle.update(e_spike);

        // F = swing forward (positive), E = swing back (negative) relative to base
        let diff = self.flexor_muscle.activation - self.extensor_muscle.activation;
        self.target_angle = self.base_angle + diff * self.range;

        // Smooth kinematic movement
        self.angle += (self.target_angle - self.angle) * 0.15;
    }
}

pub struct Crab {
    pub network: Network,
    pub legs: Vec<Leg>,
}

impl Crab {
    pub fn new() -> Self {
        let mut network = Network::new();
        let mut legs = Vec::new();

        // 6 legs x 2 neurons = 12 neurons
        for _ in 0..12 {
            network.add_neuron();
        }

        // Leg indices:
        // 0: L1 (Top Left)
        // 1: L2 (Mid Left)
        // 2: L3 (Bot Left)
        // 3: R1 (Top Right)
        // 4: R2 (Mid Right)
        // 5: R3 (Bot Right)

        // Angles relative to Center (0 is Right, -PI/2 is Up)
        // L1: -150 deg (-5PI/6) -> Top Left
        // L2: 180 deg (PI) -> Left
        // L3: 150 deg (5PI/6) -> Bottom Left
        // R1: -30 deg (-PI/6) -> Top Right
        // R2: 0 deg (0) -> Right
        // R3: 30 deg (PI/6) -> Bottom Right

        let base_angles = vec![
            -5.0*PI/6.0, PI, 5.0*PI/6.0,
            -PI/6.0, 0.0, PI/6.0,
        ];

        for i in 0..6 {
            let f = i * 2;
            let e = i * 2 + 1;
            legs.push(Leg::new(f, e, base_angles[i]));

            // Intra-leg inhibition (Half-Center Oscillator)
            // Flexor inhibits Extensor, Extensor inhibits Flexor
            network.add_synapse(f, e, -15.0);
            network.add_synapse(e, f, -15.0);
        }

        // Inter-leg Inhibition (Tripod Gait topology)
        // Neighbors inhibit each other to encourage anti-phase

        let neighbor_pairs = vec![
            (0, 1), (1, 2), // Left chain
            (3, 4), (4, 5), // Right chain
            (0, 3), (1, 4), (2, 5) // Cross links
        ];

        for (a_leg, b_leg) in neighbor_pairs {
            let a_f = a_leg * 2; // Flexor of leg A
            let b_f = b_leg * 2; // Flexor of leg B

            // Flexors inhibit each other strongly
            network.add_synapse(a_f, b_f, -5.0);
            network.add_synapse(b_f, a_f, -5.0);
        }

        Self { network, legs }
    }

    pub fn update(&mut self, drive_current: f32) {
        let mut rng = rand::thread_rng();
        let mut inputs = vec![0.0; 12];

        for i in 0..12 {
            // Apply drive to all neurons + noise
            inputs[i] = drive_current + rng.gen_range(-2.0..2.0);
        }

        self.network.step(&inputs);

        for leg in &mut self.legs {
            leg.update(&self.network);
        }
    }
}
