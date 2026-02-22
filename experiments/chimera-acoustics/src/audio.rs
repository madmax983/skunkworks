use crossbeam_channel::Receiver;
use anyhow::Result;

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[cfg(feature = "audio")]
pub type Stream = cpal::Stream;

#[cfg(not(feature = "audio"))]
pub type Stream = ();

#[cfg(feature = "audio")]
pub fn init_audio(rx: Receiver<f32>) -> Result<Stream> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::anyhow!("No output device available"))?;

    let config: cpal::StreamConfig = device.default_output_config()?.into();

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                *sample = match rx.try_recv() {
                    Ok(s) => s,
                    Err(_) => 0.0,
                };
            }
        },
        err_fn,
        None,
    )?;

    stream.play()?;

    Ok(stream)
}

#[cfg(not(feature = "audio"))]
pub fn init_audio(rx: Receiver<f32>) -> Result<Stream> {
    // Spawn a dummy thread to drain the channel so the main loop doesn't block/overflow
    std::thread::spawn(move || {
        // Simulate consuming samples at audio rate?
        // Or just consume as fast as possible.
        // If we consume instantly, main loop will generate samples as fast as it can per frame.
        loop {
            if let Ok(_) = rx.recv() {
                // Consumed
            } else {
                break; // Channel closed
            }
        }
    });
    Ok(())
}
