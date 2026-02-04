use ndarray::{Array1, azip};
use rand::Rng;

pub struct IzhikevichPopulation {
    pub v: Array1<f32>,
    pub u: Array1<f32>,
    pub a: Array1<f32>,
    pub b: Array1<f32>,
    pub c: Array1<f32>,
    pub d: Array1<f32>,
    pub size: usize,
}

impl IzhikevichPopulation {
    pub fn new(size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut v = Vec::with_capacity(size);
        let mut u = Vec::with_capacity(size);
        let mut a = Vec::with_capacity(size);
        let mut b = Vec::with_capacity(size);
        let mut c = Vec::with_capacity(size);
        let mut d = Vec::with_capacity(size);

        for _ in 0..size {
            let r = rng.gen::<f32>();
            if r < 0.8 {
                // RS (Regular Spiking) - Excitatory
                let noise = rng.gen::<f32>();
                a.push(0.02);
                b.push(0.2);
                c.push(-65.0 + 15.0 * noise.powi(2));
                d.push(8.0 - 6.0 * noise.powi(2));
                v.push(-65.0);
                u.push(0.2 * -65.0);
            } else {
                // FS (Fast Spiking) - Inhibitory
                let noise = rng.gen::<f32>();
                a.push(0.02 + 0.08 * noise);
                b.push(0.25 - 0.05 * noise);
                c.push(-65.0);
                d.push(2.0);
                v.push(-65.0);
                u.push(0.2 * -65.0);
            }
        }

        Self {
            v: Array1::from(v),
            u: Array1::from(u),
            a: Array1::from(a),
            b: Array1::from(b),
            c: Array1::from(c),
            d: Array1::from(d),
            size,
        }
    }

    pub fn update(&mut self, dt: f32, input_current: &Array1<f32>) -> Vec<bool> {
        let mut spikes = vec![false; self.size];

        // Izhikevich integration: v += dt * (0.04*v^2 + 5v + 140 - u + I)
        // u += dt * a * (b*v - u)
        // Using Euler method. For higher precision, Runge-Kutta could be used, but Euler is standard for Izhikevich.

        azip!((v in &mut self.v, u in &mut self.u, a in &self.a, b in &self.b, i_in in input_current) {
            let dv = 0.04 * (*v) * (*v) + 5.0 * (*v) + 140.0 - (*u) + (*i_in);
            let du = (*a) * ((*b) * (*v) - (*u));

            *v += dt * dv;
            *u += dt * du;
        });

        // Reset logic
        azip!((index i, v in &mut self.v, u in &mut self.u, c in &self.c, d in &self.d) {
            if *v >= 30.0 {
                *v = *c;
                *u += *d;
                spikes[i] = true;
            }
        });

        spikes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuron_update() {
        let size = 10;
        let mut pop = IzhikevichPopulation::new(size);
        let input = Array1::from_elem(size, 10.0); // Constant input current

        // Run a few steps
        for _ in 0..100 {
            pop.update(0.5, &input); // dt = 0.5ms
        }

        // Check that values have changed
        assert_ne!(pop.v[0], -65.0);
    }
}
