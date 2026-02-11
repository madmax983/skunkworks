use std::f64::consts::PI;
use rand::prelude::*;

pub const SAMPLE_RATE: f32 = 44100.0;

pub struct SignalGenerator {
    time: f64,
    rng: ThreadRng,
}

impl SignalGenerator {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            rng: rand::thread_rng(),
        }
    }

    fn kick(t: f64) -> f32 {
        let decay = 0.3;
        let trigger_period = 0.5; // Every 0.5s
        let local_t = t % trigger_period;

        if local_t > decay {
            return 0.0;
        }

        // Pitch sweep: 150Hz -> 50Hz
        let freq_start = 150.0;
        let freq_end = 50.0;

        let phase = freq_start * local_t + (freq_end - freq_start) / (2.0 * decay) * local_t * local_t;
        let amp = (1.0 - local_t / decay).powi(2);

        ((phase * 2.0 * PI).sin() * amp) as f32
    }

    fn hihat(&mut self, t: f64) -> f32 {
        let trigger_period = 0.25; // 8th notes
        let local_t = t % trigger_period;
        let decay = 0.05;

        if local_t > decay {
            return 0.0;
        }

        let noise: f64 = self.rng.gen_range(-1.0..1.0);
        let amp = (1.0 - local_t / decay).powi(4);

        (noise * amp * 0.3) as f32
    }

    fn pad(t: f64) -> f32 {
        let freq = 200.0 + (t * 0.5).sin() * 50.0;
        ((t * freq * 2.0 * PI).sin() * 0.1) as f32
    }
}

impl Iterator for SignalGenerator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let t = self.time;
        self.time += 1.0 / SAMPLE_RATE as f64;

        let k = Self::kick(t);
        let h = self.hihat(t);
        let p = Self::pad(t);

        Some((k + h + p).tanh())
    }
}
