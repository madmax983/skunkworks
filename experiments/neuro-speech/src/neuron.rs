#[derive(Debug, Clone, Copy)]
pub enum Command {
    SetCurrent(f32),
    SetGNa(f32),
    SetGK(f32),
    #[allow(dead_code)]
    SetGL(f32),
    #[allow(dead_code)]
    SetNoise(f32),
    Pluck, // Inject a short pulse
}

#[derive(Debug, Clone)]
pub struct HodgkinHuxley {
    // State variables
    pub v: f32, // Membrane potential (mV)
    pub m: f32, // Na activation
    pub h: f32, // Na inactivation
    pub n: f32, // K activation

    // Parameters (tunable)
    pub c_m: f32,  // Membrane capacitance
    pub g_na: f32, // Max Na conductance
    pub g_k: f32,  // Max K conductance
    pub g_l: f32,  // Leak conductance
    pub e_na: f32, // Na reversal potential
    pub e_k: f32,  // K reversal potential
    pub e_l: f32,  // Leak reversal potential

    // Inputs
    pub i_inj: f32, // Injected current
    pub noise_level: f32,
}

impl HodgkinHuxley {
    pub fn new() -> Self {
        Self {
            v: -65.0,
            m: 0.05,
            h: 0.6,
            n: 0.32,

            c_m: 1.0,
            g_na: 120.0,
            g_k: 36.0,
            g_l: 0.3,
            e_na: 50.0,
            e_k: -77.0,
            e_l: -54.387,

            i_inj: 0.0,
            noise_level: 0.0,
        }
    }

    pub fn apply_command(&mut self, cmd: Command) {
        match cmd {
            Command::SetCurrent(val) => self.i_inj = val,
            Command::SetGNa(val) => self.g_na = val,
            Command::SetGK(val) => self.g_k = val,
            Command::SetGL(val) => self.g_l = val,
            Command::SetNoise(val) => self.noise_level = val,
            Command::Pluck => {
                // Instantaneous charge injection (bump voltage)
                self.v += 10.0;
            }
        }
    }

    pub fn step(&mut self, dt: f32) {
        let v = self.v;

        // Rate functions
        let alpha_n = 0.01 * (v + 55.0) / (1.0 - (-(v + 55.0) / 10.0).exp());
        let beta_n = 0.125 * (-(v + 65.0) / 80.0).exp();

        let alpha_m = 0.1 * (v + 40.0) / (1.0 - (-(v + 40.0) / 10.0).exp());
        let beta_m = 4.0 * (-(v + 65.0) / 18.0).exp();

        let alpha_h = 0.07 * (-(v + 65.0) / 20.0).exp();
        let beta_h = 1.0 / (1.0 + (-(v + 35.0) / 10.0).exp());

        // Handle NaN cases (div by zero in alpha functions at specific voltages)
        // Usually implementation checks if denominator is close to 0, but f32 might handle it or we ignore for now.
        // Actually, let's make it robust.
        // If v is exactly -55, alpha_n has 0/0. L'Hopital's rule gives 0.1.
        // If v is exactly -40, alpha_m has 0/0. L'Hopital gives 1.0.

        let alpha_n = if (v + 55.0).abs() < 1e-5 {
            0.1
        } else {
            alpha_n
        };
        let alpha_m = if (v + 40.0).abs() < 1e-5 {
            1.0
        } else {
            alpha_m
        };

        // Derivatives
        let dn = alpha_n * (1.0 - self.n) - beta_n * self.n;
        let dm = alpha_m * (1.0 - self.m) - beta_m * self.m;
        let dh = alpha_h * (1.0 - self.h) - beta_h * self.h;

        // Currents
        let i_na = self.g_na * self.m.powi(3) * self.h * (v - self.e_na);
        let i_k = self.g_k * self.n.powi(4) * (v - self.e_k);
        let i_l = self.g_l * (v - self.e_l);

        // Noise
        let noise = if self.noise_level > 0.0 {
            (rand::random::<f32>() - 0.5) * self.noise_level
        } else {
            0.0
        };

        // Membrane equation
        let dv = (self.i_inj + noise - i_na - i_k - i_l) / self.c_m;

        // Euler Integration
        self.v += dv * dt;
        self.n += dn * dt;
        self.m += dm * dt;
        self.h += dh * dt;
    }
}

#[cfg(test)]
#[path = "neuron_test.rs"]
mod neuron_test;
