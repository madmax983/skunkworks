#[derive(Debug, Default)]
pub struct Shaft {
    pub angle: f32,
    pub delta_angle: f32, // Rotation in this tick
}

impl Shaft {
    pub fn rotate(&mut self, radians: f32) {
        self.delta_angle = radians;
        self.angle += radians;
    }

    pub fn reset_delta(&mut self) {
        self.delta_angle = 0.0;
    }
}

pub struct Integrator {
    // A ball-and-disc integrator.
    // Inputs:
    // - Disc Rotation (from a shaft)
    // - Carriage Position (r, distance from center)
    // Output:
    // - Cylinder Rotation (result)
    pub carriage_pos: f32, // The 'y' value. 0 is center.
    pub output_angle: f32, // The accumulated integral 'z'.
}

impl Integrator {
    pub fn new() -> Self {
        Self {
            carriage_pos: 0.0,
            output_angle: 0.0,
        }
    }

    /// Updates the integrator based on the input disc rotation.
    /// Returns the rotation of the output shaft (delta).
    /// formula: d_output = d_disc * carriage_pos
    pub fn update(&mut self, disc_delta: f32) -> f32 {
        let output_delta = disc_delta * self.carriage_pos;
        self.output_angle += output_delta;
        output_delta
    }
}

pub struct Differential {
    // Inputs: A and B
    // Output: C = (A + B) / 2
    pub output_angle: f32,
}

impl Differential {
    pub fn new() -> Self {
        Self { output_angle: 0.0 }
    }

    /// Updates based on two input deltas.
    /// Returns the output delta.
    pub fn update(&mut self, delta_a: f32, delta_b: f32) -> f32 {
        let output_delta = (delta_a + delta_b) / 2.0;
        self.output_angle += output_delta;
        output_delta
    }
}
