use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Izhikevich {
    pub v: f32,
    pub u: f32,
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}

impl Izhikevich {
    pub fn new() -> Self {
        // Default to Regular Spiking
        Self {
            v: -65.0,
            u: -13.0, // b * v approx
            a: 0.02,
            b: 0.2,
            c: -65.0,
            d: 8.0,
        }
    }

    pub fn random(rng: &mut impl Rng) -> Self {
        // Mix of types for choir texture
        let r = rng.gen::<f32>();
        if r < 0.6 {
            // Regular Spiking
            Self {
                v: -65.0,
                u: -13.0,
                a: 0.02,
                b: 0.2,
                c: -65.0,
                d: 8.0,
            }
        } else if r < 0.8 {
             // Fast Spiking
             Self {
                 v: -65.0,
                 u: -13.0,
                 a: 0.1,
                 b: 0.2,
                 c: -65.0,
                 d: 2.0,
             }
        } else {
            // Chattering
            Self {
                v: -65.0,
                u: -13.0,
                a: 0.02,
                b: 0.2,
                c: -50.0,
                d: 2.0,
            }
        }
    }

    // Returns voltage for audio output
    pub fn update(&mut self, dt: f32, i_inj: f32) -> f32 {
        // v' = 0.04v^2 + 5v + 140 - u + I
        // Using smaller sub-steps for stability if dt is large,
        // but for audio rate (dt ~ 0.02) single step Euler might be okay-ish.
        // Actually, let's use 2 substeps for safety.

        let substeps = 2;
        let dt_sub = dt / substeps as f32;

        for _ in 0..substeps {
            let dv = 0.04 * self.v * self.v + 5.0 * self.v + 140.0 - self.u + i_inj;
            self.v += dv * dt_sub;

            let du = self.a * (self.b * self.v - self.u);
            self.u += du * dt_sub;

            if self.v >= 30.0 {
                self.v = self.c;
                self.u += self.d;
                // We spike.
                // In a pure digital synth we might output a pulse, but here we output v.
                // When v resets, it jumps down. This creates the sawtooth-like wave.
            }
        }

        self.v
    }
}
