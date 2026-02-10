use crate::neuron::Izhikevich;

pub struct Population {
    pub neurons: Vec<Izhikevich>,
    pub size: usize,
    pub coupling_strength: f32,
    pub mean_field: f32,
}

impl Population {
    pub fn new(size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let neurons = (0..size).map(|_| Izhikevich::random(&mut rng)).collect();
        Self {
            neurons,
            size,
            coupling_strength: 0.0,
            mean_field: -65.0,
        }
    }

    pub fn update(&mut self, dt: f32, base_current: f32) -> f32 {
        let mut sum_v = 0.0;

        // Electrical coupling (Gap Junctions) approximation:
        // Each neuron is pulled towards the mean field.
        // I_gap = g * (V_mean - V_i)
        // This is physically correct for gap junctions.

        let v_mean_prev = self.mean_field;

        for neuron in &mut self.neurons {
            // Gap junction current
            let i_gap = self.coupling_strength * (v_mean_prev - neuron.v);

            let val = neuron.update(dt, base_current + i_gap);
            sum_v += val;
        }

        self.mean_field = sum_v / self.size as f32;
        self.mean_field
    }
}
