#[cfg(feature = "audio")]
use crate::cpu::InstructionTriggered;
use bevy::prelude::*;
#[cfg(feature = "audio")]
use std::f32::consts::PI;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {
        #[cfg(feature = "audio")]
        {
            app.add_systems(Update, play_instruction_sound);
        }
    }
}

#[cfg(feature = "audio")]
fn play_instruction_sound(
    mut events: EventReader<InstructionTriggered>,
    mut commands: Commands,
    mut audio_assets: ResMut<Assets<AudioSource>>,
) {
    for event in events.read() {
        let frequency = match event.note_index {
            0 => 261.63, // C4
            1 => 293.66, // D4
            2 => 329.63, // E4
            3 => 392.00, // G4
            4 => 440.00, // A4
            _ => 440.0,
        };

        let sample_rate = 44100;
        let duration_secs = 0.2;
        let num_samples = (sample_rate as f32 * duration_secs) as usize;
        let mut samples = Vec::with_capacity(num_samples);

        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            let sample = (t * frequency * 2.0 * PI).sin();
            let envelope = if t < 0.01 {
                t / 0.01
            } else {
                1.0 - (t - 0.01) / (duration_secs - 0.01)
            };
            let val = sample * envelope * 0.3;
            samples.push(val);
        }

        let wav_data = generate_wav(samples, sample_rate);

        // Convert Vec<u8> to Arc<[u8]>
        let bytes: std::sync::Arc<[u8]> = std::sync::Arc::from(wav_data);

        let source = AudioSource { bytes };

        let handle = audio_assets.add(source);

        commands.spawn(AudioBundle {
            source: handle,
            settings: PlaybackSettings::DESPAWN,
        });
    }
}

#[cfg(feature = "audio")]
fn generate_wav(samples: Vec<f32>, sample_rate: u32) -> Vec<u8> {
    let mut buffer = Vec::new();

    let num_channels: u16 = 1;
    let bits_per_sample: u16 = 16;
    let byte_rate = sample_rate * num_channels as u32 * bits_per_sample as u32 / 8;
    let block_align = num_channels * bits_per_sample / 8;
    let subchunk2_size = samples.len() as u32 * num_channels as u32 * bits_per_sample as u32 / 8;
    let chunk_size = 36 + subchunk2_size;

    buffer.extend_from_slice(b"RIFF");
    buffer.extend_from_slice(&chunk_size.to_le_bytes());
    buffer.extend_from_slice(b"WAVE");

    buffer.extend_from_slice(b"fmt ");
    buffer.extend_from_slice(&16u32.to_le_bytes());
    buffer.extend_from_slice(&1u16.to_le_bytes());
    buffer.extend_from_slice(&num_channels.to_le_bytes());
    buffer.extend_from_slice(&sample_rate.to_le_bytes());
    buffer.extend_from_slice(&byte_rate.to_le_bytes());
    buffer.extend_from_slice(&block_align.to_le_bytes());
    buffer.extend_from_slice(&bits_per_sample.to_le_bytes());

    buffer.extend_from_slice(b"data");
    buffer.extend_from_slice(&subchunk2_size.to_le_bytes());

    for sample in samples {
        let val = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
        buffer.extend_from_slice(&val.to_le_bytes());
    }

    buffer
}
