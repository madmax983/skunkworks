#[derive(Clone, Copy, Debug, Default)]
pub struct Spectrum {
    pub low: f32,
    pub mid: f32,
    pub high: f32,
}

pub struct AudioReactor {
    pub time: f32,
    pub spectrum: Spectrum,
    pub beat_phase: f32,
}

impl AudioReactor {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            spectrum: Spectrum::default(),
            beat_phase: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;
        self.beat_phase += dt * 3.0; // ~170 BPM for energy

        // Low freq: Big Sine wave + Noise (Kick drum)
        // Using powf(10.0) makes it very sharp (impulse-like)
        let beat = (self.beat_phase.sin() * 0.5 + 0.5).powf(10.0);
        self.spectrum.low = beat * 0.9 + rand::random::<f32>() * 0.1;

        // Mid freq: Slower undulation (Synth pads/Vocals)
        let mid_osc = (self.time * 2.0).sin() * 0.5 + 0.5;
        self.spectrum.mid = mid_osc * 0.5 + rand::random::<f32>() * 0.3;

        // High freq: Fast jitter (Hi-hats/Noise)
        let high_osc = (self.time * 15.0).sin() * 0.5 + 0.5;
        self.spectrum.high = high_osc * 0.3 + rand::random::<f32>() * 0.7;
    }

    pub fn trigger_beat(&mut self) {
        // Manually align phase to peak for interactivity
        self.beat_phase = std::f32::consts::PI / 2.0;
    }
}
