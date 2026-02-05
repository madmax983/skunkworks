use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::{HeapRb, Producer};
use anyhow::Result;
use std::sync::Arc;

pub struct AudioSystem {
    #[allow(dead_code)]
    pub stream: cpal::Stream,
    pub buffer_tx: Producer<f32, Arc<ringbuf::SharedRb<f32, Vec<std::mem::MaybeUninit<f32>>>>>,
    pub sample_rate: f32,
}

pub fn init_audio() -> Result<AudioSystem> {
    let host = cpal::default_host();
    let device = host.default_output_device().ok_or_else(|| anyhow::anyhow!("No output device"))?;
    let config = device.default_output_config()?;
    let sample_rate = config.sample_rate().0 as f32;
    let channels = config.channels() as usize;

    // Buffer size: Enough for ~100ms of audio to prevent underruns if main loop jitters
    // 44100 * 0.1 = 4410
    let ring_buffer = HeapRb::<f32>::new(8192);
    let (mut prod, mut cons) = ring_buffer.split();

    // Fill buffer with silence initially to avoid immediate underrun
    for _ in 0..4096 {
        let _ = prod.push(0.0);
    }

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(channels) {
                let sample = cons.pop().unwrap_or(0.0);
                for out in frame.iter_mut() {
                    *out = sample;
                }
            }
        },
        |err| eprintln!("Audio error: {}", err),
        None,
    )?;

    stream.play()?;

    Ok(AudioSystem {
        stream,
        buffer_tx: prod,
        sample_rate,
    })
}
