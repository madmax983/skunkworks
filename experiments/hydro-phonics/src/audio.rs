use std::sync::{Arc, atomic::{AtomicU32, Ordering}};

pub struct Probe {
    pub x: usize,
    pub y: usize,
    pub value: Arc<AtomicU32>,
}

impl Probe {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            value: Arc::new(AtomicU32::new(0u32)),
        }
    }

    pub fn update(&self, val: f32) {
        self.value.store(val.to_bits(), Ordering::Relaxed);
    }
}

#[cfg(feature = "audio")]
pub mod engine {
    use super::Probe;
    use rodio::Source;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::time::Duration;

    pub struct ProbeSource {
        pub value: Arc<AtomicU32>,
        pub phase: f32,
        pub base_freq: f32,
        pub sample_rate: u32,
    }

    impl ProbeSource {
        pub fn new(probe: &Probe, base_freq: f32) -> Self {
            Self {
                value: probe.value.clone(),
                phase: 0.0,
                base_freq,
                sample_rate: 44100,
            }
        }
    }

    impl Iterator for ProbeSource {
        type Item = f32;

        fn next(&mut self) -> Option<Self::Item> {
            let bits = self.value.load(Ordering::Relaxed);
            let val = f32::from_bits(bits);

            // FM Synthesis
            // Base Freq + (Value * Modulation Index)
            // Value is roughly -5.0 to 5.0 in strong waves
            let freq = self.base_freq + (val * 50.0);
            let freq = freq.max(10.0).min(2000.0); // Safety clamp

            let phase_inc = freq * 2.0 * std::f32::consts::PI / self.sample_rate as f32;
            self.phase = (self.phase + phase_inc) % (2.0 * std::f32::consts::PI);

            Some(self.phase.sin() * 0.2) // 0.2 Gain to avoid clipping with multiple probes
        }
    }

    impl Source for ProbeSource {
        fn current_frame_len(&self) -> Option<usize> {
            None
        }

        fn channels(&self) -> u16 {
            1
        }

        fn sample_rate(&self) -> u32 {
            self.sample_rate
        }

        fn total_duration(&self) -> Option<Duration> {
            None
        }
    }
}
