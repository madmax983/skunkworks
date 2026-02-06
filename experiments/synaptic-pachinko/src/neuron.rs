use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Izhikevich {
    pub v: f32,
    pub u: f32,
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub current_decay: f32, // For decaying injected current
}

impl Izhikevich {
    pub fn new() -> Self {
        Self {
            v: -65.0,
            u: -13.0,
            a: 0.02,
            b: 0.2,
            c: -65.0,
            d: 8.0,
            current_decay: 0.0,
        }
    }

    pub fn random(rng: &mut impl Rng) -> Self {
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
                current_decay: 0.0,
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
                 current_decay: 0.0,
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
                current_decay: 0.0,
            }
        }
    }

    pub fn inject(&mut self, current: f32) {
        self.current_decay += current;
    }

    // Returns voltage for audio output
    pub fn update(&mut self, dt: f32) -> f32 {
        let substeps = 2;
        let dt_sub = dt / substeps as f32;

        for _ in 0..substeps {
            // Decay the injected current
            self.current_decay *= 0.95; // Exponential decay

            let dv = 0.04 * self.v * self.v + 5.0 * self.v + 140.0 - self.u + self.current_decay;
            self.v += dv * dt_sub;

            let du = self.a * (self.b * self.v - self.u);
            self.u += du * dt_sub;

            if self.v >= 30.0 {
                self.v = self.c;
                self.u += self.d;
            }
        }

        self.v
    }
}
