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

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_integrator_constant() {
        // Integrate y = 2 with respect to x.
        // z = int(2 dx) = 2x.
        let mut integrator = Integrator::new();
        integrator.carriage_pos = 2.0;

        // Rotate input by PI
        let output_delta = integrator.update(PI);

        assert!((output_delta - 2.0 * PI).abs() < 1e-5);
        assert!((integrator.output_angle - 2.0 * PI).abs() < 1e-5);
    }

    #[test]
    fn test_integrator_linear() {
        // Integrate y = x. z = x^2/2.
        // We step x by small amounts and update y (carriage) accordingly.
        let mut integrator = Integrator::new();
        let dt = 0.01;
        let steps = 100;
        let mut x = 0.0;

        for _ in 0..steps {
            integrator.carriage_pos = x;
            integrator.update(dt);
            x += dt;
        }

        // Expected: z = x^2 / 2 = (1.0)^2 / 2 = 0.5
        // Approximation errors will exist, so we use a larger epsilon or better integration method (trapezoidal).
        // Since our update is rectangular (Euler), error is proportional to step size.
        // With dt=0.01, error should be small.

        let expected = 0.5;
        assert!(
            (integrator.output_angle - expected).abs() < 0.01,
            "Got {}, expected {}",
            integrator.output_angle,
            expected
        );
    }

    #[test]
    fn test_differential() {
        let mut diff = Differential::new();
        // A rotates 10, B rotates 20. Output should be (10+20)/2 = 15.
        let out = diff.update(10.0, 20.0);
        assert!((out - 15.0).abs() < 1e-5);
        assert!((diff.output_angle - 15.0).abs() < 1e-5);
    }
}
