use crate::rhythm::Instrument;
use hound;
use std::f32::consts::PI;

pub fn render_wav(
    filename: &str,
    pattern1: &[Instrument],
    pattern2: &[Instrument],
    bpm: u32,
    duration_beats: u32,
    speed_ratio: f32,
) -> anyhow::Result<()> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(filename, spec)?;
    let sample_rate = 44100;

    // Duration
    // Beats * SamplesPerBeat
    let samples_per_beat = sample_rate as f32 * 60.0 / bpm as f32;
    let total_samples = (samples_per_beat * duration_beats as f32) as usize;

    let mut buffer = vec![0.0; total_samples];

    // Synthesis
    let kick = synthesize_kick(sample_rate);
    let snare = synthesize_snare(sample_rate);
    let hat = synthesize_hat(sample_rate);

    // Track 1
    // 16th notes
    let samples_per_step_1 = samples_per_beat / 4.0;
    let mut cursor = 0.0;
    let mut step_count = 0;

    while cursor < total_samples as f32 {
        let step_idx = step_count % pattern1.len();
        match pattern1[step_idx] {
            Instrument::Kick => mix_sound(&mut buffer, cursor as usize, &kick),
            Instrument::Snare => mix_sound(&mut buffer, cursor as usize, &snare),
            Instrument::Hat => mix_sound(&mut buffer, cursor as usize, &hat),
            Instrument::Rest => {}
        }
        cursor += samples_per_step_1;
        step_count += 1;
    }

    // Track 2 (Phased)
    let samples_per_step_2 = samples_per_step_1 / speed_ratio;
    let mut cursor = 0.0;
    let mut step_count = 0;

    while cursor < total_samples as f32 {
        let step_idx = step_count % pattern2.len();
        match pattern2[step_idx] {
            Instrument::Kick => mix_sound(&mut buffer, cursor as usize, &kick), // Could modify pitch/pan to distinguish
            Instrument::Snare => mix_sound(&mut buffer, cursor as usize, &snare),
            Instrument::Hat => mix_sound(&mut buffer, cursor as usize, &hat),
            Instrument::Rest => {}
        }
        cursor += samples_per_step_2;
        step_count += 1;
    }

    // Write
    for sample in buffer {
        let val = (sample * 0.5).clamp(-1.0, 1.0);
        writer.write_sample((val * i16::MAX as f32) as i16)?;
    }

    writer.finalize()?;
    Ok(())
}

fn mix_sound(buffer: &mut [f32], start_idx: usize, sound: &[f32]) {
    for (i, &sample) in sound.iter().enumerate() {
        if start_idx + i < buffer.len() {
            buffer[start_idx + i] += sample;
        }
    }
}

fn synthesize_kick(sample_rate: u32) -> Vec<f32> {
    let duration = 0.3;
    let len = (sample_rate as f32 * duration) as usize;
    let mut data = Vec::with_capacity(len);

    // Sine sweep 150Hz -> 50Hz
    let start_freq = 150.0;
    let end_freq = 50.0;

    for i in 0..len {
        let t = i as f32 / sample_rate as f32;
        let progress = t / duration;
        let freq = start_freq + (end_freq - start_freq) * progress;
        let amp = (1.0 - progress).powf(2.0); // Decay

        let phase = 2.0 * PI * freq * t;
        data.push(phase.sin() * amp);
    }
    data
}

fn synthesize_snare(sample_rate: u32) -> Vec<f32> {
    let duration = 0.15;
    let len = (sample_rate as f32 * duration) as usize;
    let mut data = Vec::with_capacity(len);

    // White noise + Sine
    for i in 0..len {
        let t = i as f32 / sample_rate as f32;
        let progress = t / duration;
        let amp = (1.0 - progress).powf(4.0);

        let noise: f32 = rand::random::<f32>() * 2.0 - 1.0;
        let tone = (2.0 * PI * 200.0 * t).sin();

        data.push((noise * 0.8 + tone * 0.2) * amp);
    }
    data
}

fn synthesize_hat(sample_rate: u32) -> Vec<f32> {
    let duration = 0.05;
    let len = (sample_rate as f32 * duration) as usize;
    let mut data = Vec::with_capacity(len);

    // High pass noise (simulated by random)
    for i in 0..len {
        let t = i as f32 / sample_rate as f32;
        let progress = t / duration;
        let amp = (1.0 - progress).powf(10.0);

        let noise: f32 = rand::random::<f32>() * 2.0 - 1.0;
        data.push(noise * amp * 0.5);
    }
    data
}
