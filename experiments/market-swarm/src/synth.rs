use rand::Rng;

pub struct SynthState {
    pub price_frequency: f32,
    pub trade_intensity: f32,
    pub phase: f32,
}

impl SynthState {
    pub fn new() -> Self {
        Self {
            price_frequency: 440.0,
            trade_intensity: 0.0,
            phase: 0.0,
        }
    }

    pub fn update(&mut self, price: f32, trades: usize) {
        // Higher y (lower in grid) -> Lower Price -> Lower Freq?
        // Let's assume Price = Height - y.
        // If y is small (Top), Price is High.
        // Let's map y=0 -> 880Hz, y=H -> 220Hz.

        // This logic is simplistic but fine for Moonshot.
        self.price_frequency = (880.0 - price * 10.0).max(110.0);

        // Trades add noise/intensity.
        // Decay existing intensity.
        if trades > 0 {
            self.trade_intensity += (trades as f32) * 0.2;
        }
        self.trade_intensity *= 0.9; // Decay
        self.trade_intensity = self.trade_intensity.min(1.0);
    }

    /// Generates a waveform buffer for visualization.
    /// `width` is the number of horizontal pixels.
    pub fn get_waveform(&mut self, width: usize) -> Vec<f64> {
        let mut buffer = Vec::with_capacity(width);
        // We want to visualize a few cycles.
        let time_window = 0.02; // 20ms
        let dt = time_window / width as f32;

        for _ in 0..width {
            self.phase += self.price_frequency * dt * std::f32::consts::TAU;
            if self.phase > std::f32::consts::TAU {
                self.phase -= std::f32::consts::TAU;
            }

            let signal = self.phase.sin();

            // Add noise based on trade_intensity
            let mut rng = rand::thread_rng();
            let noise = (rng.r#gen::<f32>() * 2.0 - 1.0) * self.trade_intensity;

            buffer.push((signal * 0.8 + noise * 0.2) as f64);
        }

        buffer
    }
}
