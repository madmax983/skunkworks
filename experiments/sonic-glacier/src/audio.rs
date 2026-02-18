use crossbeam_channel::Sender;
use rustfft::{num_complex::Complex, FftPlanner};
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};

pub const SAMPLE_RATE: u32 = 44100;
pub const FFT_SIZE: usize = 1024;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Spectrum {
    pub low: f32,
    pub mid: f32,
    pub high: f32,
    pub raw: Vec<f32>,
}

pub struct SonicEngine {
    sample_clock: u64,
    buffer: Vec<f32>,
    fft_planner: Arc<Mutex<FftPlanner<f32>>>,
    sender: Sender<Spectrum>,
    // Synth params
    freq: f32,
    mod_freq: f32,
    mod_index: f32,
}

impl SonicEngine {
    pub fn new(sender: Sender<Spectrum>) -> Self {
        Self {
            sample_clock: 0,
            buffer: Vec::with_capacity(FFT_SIZE),
            fft_planner: Arc::new(Mutex::new(FftPlanner::new())),
            sender,
            freq: 110.0,
            mod_freq: 55.0,
            mod_index: 2.0,
        }
    }

    pub fn generate_sample(&mut self) -> f32 {
        let t = self.sample_clock as f32 / SAMPLE_RATE as f32;

        // Slowly evolve parameters
        // LFO for carrier freq
        self.freq = 110.0 + (t * 0.1).sin() * 20.0;
        // LFO for modulation index
        self.mod_index = 3.0 + (t * 0.23).cos() * 2.0;

        // FM Synthesis
        let modulator = (2.0 * PI * self.mod_freq * t).sin() * self.mod_index;
        let carrier = (2.0 * PI * (self.freq + modulator * self.freq) * t).sin();

        let sample = carrier * 0.5;

        // Buffer for FFT
        self.buffer.push(sample);
        if self.buffer.len() >= FFT_SIZE {
            self.perform_fft();
            self.buffer.clear();
        }

        self.sample_clock += 1;
        sample
    }

    fn perform_fft(&mut self) {
        let mut planner = self.fft_planner.lock().unwrap();
        let fft = planner.plan_fft_forward(FFT_SIZE);

        let mut input: Vec<Complex<f32>> = self
            .buffer
            .iter()
            .map(|&x| Complex { re: x, im: 0.0 })
            .collect();

        // Apply Hann window
        for (i, val) in input.iter_mut().enumerate() {
            let window = 0.5 * (1.0 - (2.0 * PI * i as f32 / (FFT_SIZE - 1) as f32).cos());
            val.re *= window;
        }

        fft.process(&mut input);

        // Calculate magnitude
        let magnitudes: Vec<f32> = input.iter().take(FFT_SIZE / 2).map(|c| c.norm()).collect();

        // Binning
        // Bin width = 44100 / 1024 = ~43 Hz
        // Low: 0-200Hz -> bins 0-4
        // Mid: 200-2000Hz -> bins 4-46
        // High: 2000Hz+ -> bins 46+

        let bin_width = SAMPLE_RATE as f32 / FFT_SIZE as f32;
        let low_idx = (200.0 / bin_width) as usize;
        let mid_idx = (2000.0 / bin_width) as usize;
        let end_idx = magnitudes.len();

        let low_e: f32 = magnitudes.iter().take(low_idx).sum::<f32>() / low_idx.max(1) as f32;
        let mid_e: f32 = magnitudes
            .iter()
            .skip(low_idx)
            .take(mid_idx - low_idx)
            .sum::<f32>()
            / (mid_idx - low_idx).max(1) as f32;
        let high_e: f32 =
            magnitudes.iter().skip(mid_idx).sum::<f32>() / (end_idx - mid_idx).max(1) as f32;

        let _ = self.sender.try_send(Spectrum {
            low: low_e,
            mid: mid_e,
            high: high_e,
            raw: magnitudes,
        });
    }
}

impl Iterator for SonicEngine {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.generate_sample())
    }
}

#[cfg(feature = "audio")]
use rodio::Source;
#[cfg(feature = "audio")]
use std::time::Duration;

#[cfg(feature = "audio")]
impl Source for SonicEngine {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossbeam_channel::unbounded;

    #[test]
    fn test_fft_sine_wave() {
        let (tx, rx) = unbounded();
        let mut engine = SonicEngine::new(tx);

        // Override generate_sample to produce pure sine at ~430Hz (approx bin 10)
        // Bin width = 44100 / 1024 = 43.06 Hz
        // 430 Hz = Bin 10 approx.

        // We bypass the internal oscillator logic for this test and just push to buffer
        engine.buffer.clear();
        for i in 0..FFT_SIZE {
            let t = i as f32 / SAMPLE_RATE as f32;
            let sample = (2.0 * PI * 430.0 * t).sin();
            engine.buffer.push(sample);
        }

        engine.perform_fft();

        let spectrum = rx.try_recv().expect("Should receive spectrum");

        // Check if mid energy is dominant (430Hz is in mid range: 200-2000)
        println!(
            "Low: {}, Mid: {}, High: {}",
            spectrum.low, spectrum.mid, spectrum.high
        );
        assert!(spectrum.mid > spectrum.low);
        assert!(spectrum.mid > spectrum.high);
    }
}
