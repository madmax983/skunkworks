use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;
use hound;

pub const SAMPLE_RATE: f32 = 44100.0;

pub fn save_recording(recorder: &AudioRecorder) -> Result<(), anyhow::Error> {
    if recorder.samples.is_empty() { return Ok(()); }
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut writer = hound::WavWriter::create("recording.wav", spec)?;
    for sample in &recorder.samples {
        writer.write_sample(*sample)?;
    }
    writer.finalize()?;
    println!("Saved recording.wav");
    Ok(())
}

#[derive(Component)]
pub struct AudioString {
    pub buffer: Vec<f32>,
    pub cursor: usize,
    pub damping: f32,
    pub frequency: f32,
}

impl AudioString {
    pub fn new(frequency: f32, damping: f32) -> Self {
        let period = (SAMPLE_RATE / frequency).max(2.0);
        let len = period as usize;
        Self {
            buffer: vec![0.0; len],
            cursor: 0,
            damping,
            frequency,
        }
    }

    pub fn pluck(&mut self, strength: f32) {
        let mut rng = rand::thread_rng();
        for sample in self.buffer.iter_mut() {
            *sample += (rng.gen::<f32>() * 2.0 - 1.0) * strength;
        }
    }

    pub fn tick(&mut self) -> f32 {
        if self.buffer.is_empty() {
            return 0.0;
        }

        let current_val = self.buffer[self.cursor];
        let next_idx = (self.cursor + 1) % self.buffer.len();
        let next_val = self.buffer[next_idx];

        // Low-pass filter
        let avg = (current_val + next_val) * 0.5 * self.damping;

        self.buffer[self.cursor] = avg;
        self.cursor = next_idx;

        current_val
    }
}

// Resource to accumulate audio samples for export (optional)
#[derive(Resource, Default)]
pub struct AudioRecorder {
    pub samples: Vec<f32>,
    pub recording: bool,
}

// System to advance physics
pub fn advance_audio_physics(
    mut query: Query<&mut AudioString>,
    mut recorder: ResMut<AudioRecorder>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    let steps = (dt * SAMPLE_RATE) as usize;

    // Safety clamp
    let steps = steps.min(SAMPLE_RATE as usize / 10);

    let mut frame_samples = vec![0.0; steps];

    for mut string in query.iter_mut() {
        for i in 0..steps {
            let val = string.tick();
            frame_samples[i] += val;
        }
    }

    // Soft clip
    for s in frame_samples.iter_mut() {
        *s = (*s * 0.5).tanh();
    }

    if recorder.recording {
        recorder.samples.extend_from_slice(&frame_samples);
    }
}
