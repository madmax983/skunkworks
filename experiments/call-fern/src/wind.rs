#[derive(Debug, Clone)]
pub struct Wind {
    /// How fast the wind animation plays
    pub speed: f64,
    /// The maximum angle (in radians) of sway
    pub strength: f64,
    /// How fast the wave propagates through the branches (spatial frequency)
    pub frequency: f64,
}

impl Default for Wind {
    fn default() -> Self {
        Self {
            speed: 2.0,
            strength: 0.15,
            frequency: 0.5,
        }
    }
}

impl Wind {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn calculate_sway(&self, time: f64, depth: usize) -> f64 {
        // Calculate the phase of the wave based on time and depth.
        // As depth increases, the phase shifts, creating a wave effect.
        let phase = time * self.speed + (depth as f64) * self.frequency;

        // Use sine wave for periodic motion
        phase.sin() * self.strength
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_wind_sway_bounds() {
        let wind = Wind {
            strength: 0.5,
            ..Wind::default()
        };

        for t in 0..100 {
            let time = t as f64 * 0.1;
            let sway = wind.calculate_sway(time, 0);
            assert!(sway.abs() <= 0.50001);
        }
    }

    #[test]
    fn test_wind_periodicity() {
        let wind = Wind {
            speed: 1.0,
            strength: 1.0,
            frequency: 0.0,
        };

        // At time 0
        let s0 = wind.calculate_sway(0.0, 0);
        // At time 2PI
        let s2pi = wind.calculate_sway(2.0 * PI, 0);

        assert!((s0 - s2pi).abs() < 1e-6);
    }

    #[test]
    fn test_depth_variance() {
        let wind = Wind {
            speed: 0.0,
            strength: 1.0,
            frequency: 1.0,
        };

        // At time 0, depth 0 -> sin(0) = 0
        let d0 = wind.calculate_sway(0.0, 0);
        assert!(d0.abs() < 1e-6);

        // At time 0, depth PI/2 (approx 1.57) -> sin(1.57) ~= 1
        // Since depth is usize, we can't pass PI/2 strictly, but we can check if different depths give different values.
        let d1 = wind.calculate_sway(0.0, 1);

        assert!((d0 - d1).abs() > 1e-6);
    }
}
