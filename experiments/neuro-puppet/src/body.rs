pub struct Creature {
    pub muscle_l: f64,
    pub muscle_r: f64,
}

impl Creature {
    pub fn new() -> Self {
        Self {
            muscle_l: 0.0,
            muscle_r: 0.0,
        }
    }

    pub fn update(&mut self, spikes: &[bool]) {
        // Neuron 0 -> Left Muscle
        // Neuron 1 -> Right Muscle
        let spike_l = if spikes.len() > 0 && spikes[0] { 1.0 } else { 0.0 };
        let spike_r = if spikes.len() > 1 && spikes[1] { 1.0 } else { 0.0 };

        // Leaky integrator for smooth muscle movement
        // decay factor must be < 1.0
        let decay = 0.95;
        let strength = 2.0;

        self.muscle_l = self.muscle_l * decay + spike_l * strength;
        self.muscle_r = self.muscle_r * decay + spike_r * strength;
    }
}
