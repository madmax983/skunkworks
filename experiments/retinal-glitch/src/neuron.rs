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
        Self {
            v: -65.0,
            u: -13.0,
            a: 0.02,
            b: 0.2,
            c: -65.0,
            d: 8.0,
        }
    }

    pub fn update(&mut self, dt: f32, input: f32) -> bool {
        let substeps = 2;
        let dt_sub = dt / substeps as f32;
        let mut spiked = false;

        for _ in 0..substeps {
            let v = self.v;
            let u = self.u;

            let dv = 0.04 * v * v + 5.0 * v + 140.0 - u + input;
            let du = self.a * (self.b * v - u);

            self.v += dv * dt_sub;
            self.u += du * dt_sub;

            if self.v >= 30.0 {
                self.v = self.c;
                self.u += self.d;
                spiked = true;
            }
        }
        spiked
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spike_generation() {
        let mut neuron = Izhikevich::new();
        let mut spiked = false;
        // Inject strong current for 100ms
        for _ in 0..100 {
            if neuron.update(1.0, 20.0) {
                spiked = true;
                break;
            }
        }
        assert!(spiked, "Neuron should spike with strong input");
    }
}
