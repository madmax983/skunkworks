#[cfg(feature = "resonance")]
use resonance_audio::audio::AudioModel;
#[cfg(feature = "resonance")]
use rodio::Source;
#[cfg(feature = "resonance")]
use std::time::Duration;

#[cfg(feature = "resonance")]
pub struct RodioAudioSource {
    pub model: AudioModel,
    pub buffer: Vec<f32>,
    pub pos: usize,
}

#[cfg(feature = "resonance")]
impl RodioAudioSource {
    pub fn new(model: AudioModel) -> Self {
        Self {
            model,
            buffer: vec![0.0; 512], // Standard buffer size
            pos: 512,               // Force initial fill
        }
    }
}

#[cfg(feature = "resonance")]
impl Iterator for RodioAudioSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.buffer.len() {
            self.model.process(&mut self.buffer);
            self.pos = 0;
        }
        let val = self.buffer[self.pos];
        self.pos += 1;
        Some(val)
    }
}

#[cfg(feature = "resonance")]
impl Source for RodioAudioSource {
    fn current_frame_len(&self) -> Option<usize> {
        None // Infinite
    }

    fn channels(&self) -> u16 {
        1 // Mono
    }

    fn sample_rate(&self) -> u32 {
        44100
    }

    fn total_duration(&self) -> Option<Duration> {
        None // Infinite
    }
}
