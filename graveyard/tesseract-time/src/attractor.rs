use crate::math4d::Vec4;

pub struct Lissajous4D {
    pub time: f32,
    pub freqs: Vec4,
    pub phases: Vec4,
}

impl Lissajous4D {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            freqs: Vec4::new(1.3, 1.7, 2.3, 2.9), // Primes for less repetition
            phases: Vec4::new(0.0, 0.5, 1.0, 1.5),
        }
    }

    pub fn update(&mut self, dt: f32) -> Vec4 {
        self.time += dt;

        let t = self.time;

        // Add some "Time Series" chaos by modulating frequency slightly
        let modulation = (t * 0.1).sin() * 0.2 + 1.0;

        let x = (t * self.freqs.x * modulation + self.phases.x).sin();
        let y = (t * self.freqs.y * modulation + self.phases.y).cos(); // Mix sin/cos
        let z = (t * self.freqs.z * modulation + self.phases.z).sin();
        let w = (t * self.freqs.w * modulation + self.phases.w).cos();

        Vec4::new(x, y, z, w)
    }
}
