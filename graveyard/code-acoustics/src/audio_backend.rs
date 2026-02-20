use anyhow::Result;
use crossbeam_channel::{Receiver, Sender};
use resonance_audio::audio::{AudioCommand, AudioModel, AudioSnapshot};

#[cfg(not(feature = "audio"))]
use std::{
    thread,
    time::{Duration, Instant},
};

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub struct AudioSystem {
    #[cfg(feature = "audio")]
    _stream: cpal::Stream,
    #[cfg(not(feature = "audio"))]
    _thread: thread::JoinHandle<()>,
}

pub fn init_audio(
    width: usize,
    height: usize,
    cmd_rx: Receiver<AudioCommand>,
    snap_tx: Sender<AudioSnapshot>,
) -> Result<AudioSystem> {
    #[cfg(feature = "audio")]
    {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow::anyhow!("No audio device available"))?;
        let config = device.default_output_config()?;
        let stream_config: cpal::StreamConfig = config.into();

        let mut model = AudioModel::new(width, height, cmd_rx, snap_tx, None);

        let stream = device.build_output_stream(
            &stream_config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                model.process(data);
            },
            move |err| eprintln!("Audio error: {}", err),
            None,
        )?;

        stream.play()?;
        Ok(AudioSystem { _stream: stream })
    }

    #[cfg(not(feature = "audio"))]
    {
        println!("Audio feature disabled. Running in simulation mode.");
        let handle = thread::spawn(move || {
            let mut model = AudioModel::new(width, height, cmd_rx, snap_tx, None);
            // Simulate 44.1kHz processing in chunks
            let chunk_size = 1024;
            let mut buffer = vec![0.0; chunk_size];
            let sample_rate = 44100.0;
            let chunk_duration = Duration::from_secs_f64(chunk_size as f64 / sample_rate);

            loop {
                let start = Instant::now();
                model.process(&mut buffer);

                // Sleep to maintain real-time speed approximately
                let elapsed = start.elapsed();
                if elapsed < chunk_duration {
                    thread::sleep(chunk_duration - elapsed);
                }
            }
        });
        Ok(AudioSystem { _thread: handle })
    }
}
