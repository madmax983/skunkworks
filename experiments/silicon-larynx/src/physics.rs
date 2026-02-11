use std::f32::consts::PI;

pub struct KellyLochbaum;

impl KellyLochbaum {
    /// Calculate reflection coefficient r = (A1 - A2) / (A1 + A2)
    /// This corresponds to pressure reflection coefficient for area discontinuity.
    /// Note: Some texts use (A2 - A1) / (A2 + A1) for admittance.
    /// Here we simulate pressure waves, where Z ~ 1/A.
    /// r = (Z2 - Z1) / (Z2 + Z1) = (1/A2 - 1/A1) / (1/A2 + 1/A1)
    ///   = ((A1 - A2) / (A1*A2)) / ((A1 + A2) / (A1*A2))
    ///   = (A1 - A2) / (A1 + A2)
    pub fn calculate_reflection(a1: f32, a2: f32) -> f32 {
        if (a1 + a2).abs() < 1e-6 {
            0.0
        } else {
            (a1 - a2) / (a1 + a2)
        }
    }
}

pub struct VocalTract {
    pub areas: Vec<f32>,
    pub reflections: Vec<f32>,
    pub forward: Vec<f32>,
    pub backward: Vec<f32>,
    pub n_tubes: usize,
    pub output_sample: f32,
}

impl VocalTract {
    pub fn new(n_tubes: usize) -> Self {
        Self {
            areas: vec![1.0; n_tubes],
            reflections: vec![0.0; n_tubes], // r[i] is between i and i+1
            forward: vec![0.0; n_tubes],
            backward: vec![0.0; n_tubes],
            n_tubes,
            output_sample: 0.0,
        }
    }

    pub fn calculate_reflections(&mut self) {
        for i in 0..self.n_tubes - 1 {
            self.reflections[i] = KellyLochbaum::calculate_reflection(self.areas[i], self.areas[i + 1]);
        }
    }

    pub fn step(&mut self, glottal_input: f32) {
        self.calculate_reflections();

        let mut next_forward = vec![0.0; self.n_tubes];
        let mut next_backward = vec![0.0; self.n_tubes];

        // Junction 0 (Glottis -> Tube 0)
        // Assume glottis has impedance match or handle reflection?
        // Simplified: Forward[0] = Input + Reflection from back
        // But usually input is flow source.
        next_forward[0] = glottal_input + 0.7 * self.backward[0];

        // Scattering at junctions 0 to N-2
        for i in 0..self.n_tubes - 1 {
            let r = self.reflections[i];
            let f_in = self.forward[i];
            let b_in = self.backward[i + 1];

            // Scattering
            // f_out = (1+r)f_in - r*b_in
            // b_out = r*f_in + (1-r)*b_in
            let f_out = (1.0 + r) * f_in - r * b_in;
            let b_out = r * f_in + (1.0 - r) * b_in;

            next_forward[i + 1] = f_out;
            next_backward[i] = b_out;
        }

        // Lips (Tube N-1 -> Air)
        // Assume open end, reflection coeff approx -0.8 to -0.99 for pressure
        let r_lips = -0.85;
        let f_lips = self.forward[self.n_tubes - 1];
        // b_lips comes from 0 (no backward wave from outside world)
        // so b_out_lips = r_lips * f_lips
        next_backward[self.n_tubes - 1] = r_lips * f_lips;

        self.forward = next_forward;
        self.backward = next_backward;

        // Output is the flow at the lips, which is approx derivative of volume velocity?
        // Or just the forward wave + backward wave at lips?
        // Let's just tap the forward wave at the lips.
        self.output_sample = self.forward[self.n_tubes - 1];
    }
}

pub struct Glottis {
    pub phase: f32,
    pub frequency: f32,
    pub sample_rate: f32,
}

impl Glottis {
    pub fn new(freq: f32, sr: f32) -> Self {
        Self { phase: 0.0, frequency: freq, sample_rate: sr }
    }

    pub fn step(&mut self) -> f32 {
        self.phase += self.frequency / self.sample_rate;
        if self.phase > 1.0 { self.phase -= 1.0; }

        // Simple polynomial pulse (Rosenberg C)
        let t = self.phase;
        if t < 0.5 {
            0.5 * (1.0 - (PI * t / 0.5).cos())
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflection_coefficient() {
        let a1 = 1.0;
        let a2 = 3.0;
        let r = KellyLochbaum::calculate_reflection(a1, a2);
        // (1 - 3) / (1 + 3) = -2 / 4 = -0.5
        assert!((r - -0.5).abs() < 1e-6, "Reflection coeff should be -0.5");

        let r_eq = KellyLochbaum::calculate_reflection(2.0, 2.0);
        assert!((r_eq - 0.0).abs() < 1e-6, "Reflection coeff should be 0.0");
    }

    #[test]
    fn test_vocal_tract_init() {
        let tract = VocalTract::new(10);
        assert_eq!(tract.areas.len(), 10);
        assert_eq!(tract.forward.len(), 10);
        assert_eq!(tract.backward.len(), 10);
    }
}
