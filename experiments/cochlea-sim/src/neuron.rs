pub struct HairCell {
    potential: f32,
    smoothing: f32, // 0.0 to 1.0, closer to 1.0 is smoother
}

impl HairCell {
    pub fn new(smoothing: f32) -> Self {
        Self {
            potential: 0.0,
            smoothing,
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        // Half-wave rectification
        let rectified = input.max(0.0);

        // Low-pass filter (Leaky Integrator)
        self.potential = self.potential * self.smoothing + rectified * (1.0 - self.smoothing);

        self.potential
    }
}

pub struct LIFNeuron {
    potential: f32,
    threshold: f32,
    decay: f32,
    reset_val: f32,
    refractory_period: usize,
    refractory_timer: usize,
}

impl LIFNeuron {
    pub fn new(threshold: f32, decay: f32) -> Self {
        Self {
            potential: 0.0,
            threshold,
            decay,
            reset_val: 0.0,
            refractory_period: 44, // ~1ms at 44.1kHz
            refractory_timer: 0,
        }
    }

    pub fn process(&mut self, input_current: f32) -> bool {
        if self.refractory_timer > 0 {
            self.refractory_timer -= 1;
            return false;
        }

        self.potential = self.potential * self.decay + input_current;

        if self.potential >= self.threshold {
            self.potential = self.reset_val;
            self.refractory_timer = self.refractory_period;
            true
        } else {
            false
        }
    }
}
