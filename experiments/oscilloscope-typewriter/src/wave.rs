use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WaveType {
    Sine,
    Square,
    Saw,
    Triangle,
    Noise,
}

#[derive(Clone, Debug)]
pub struct Voice {
    pub frequency: f32,
    pub amplitude: f32,
    pub phase: f32,
    pub wave_type: WaveType,
}

impl Voice {
    pub fn new(frequency: f32, amplitude: f32, wave_type: WaveType) -> Self {
        Self {
            frequency,
            amplitude,
            phase: 0.0,
            wave_type,
        }
    }

    pub fn synthesize(&self, time: f32, spatial_offset: f32) -> f32 {
        let t = time * self.frequency + self.phase + spatial_offset;
        let pi = std::f32::consts::PI;

        let raw = match self.wave_type {
            WaveType::Sine => t.sin(),
            WaveType::Square => if t.sin() >= 0.0 { 1.0 } else { -1.0 },
            WaveType::Saw => {
                let period = 2.0 * pi;
                let normalized = (t % period + period) % period; // handle negative
                (normalized / period) * 2.0 - 1.0
            },
            WaveType::Triangle => {
                 t.sin().asin() / (pi / 2.0)
            },
            WaveType::Noise => {
                rand::gen_range(-1.0f32, 1.0f32)
            },
        };
        raw * self.amplitude
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sine_wave() {
        let voice = Voice::new(1.0, 1.0, WaveType::Sine);
        let val = voice.synthesize(0.0, 0.0);
        assert!((val - 0.0).abs() < 0.001);

        let val_pi_half = voice.synthesize(std::f32::consts::PI / 2.0, 0.0);
        assert!((val_pi_half - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_square_wave() {
        let voice = Voice::new(1.0, 1.0, WaveType::Square);
        let val = voice.synthesize(0.1, 0.0);
        assert_eq!(val, 1.0);

        let val_neg = voice.synthesize(std::f32::consts::PI + 0.1, 0.0);
        assert_eq!(val_neg, -1.0);
    }
}
