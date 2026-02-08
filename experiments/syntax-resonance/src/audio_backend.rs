#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[cfg(feature = "audio")]
use crossbeam_channel::{Receiver, Sender};
#[cfg(feature = "audio")]
use resonance_audio::audio::{AudioCommand, AudioModel};

#[cfg(feature = "audio")]
pub fn init_audio(
    cmd_rx: Receiver<AudioCommand>,
    snap_tx: Sender<Vec<f32>>,
    width: usize,
    height: usize,
) -> anyhow::Result<cpal::Stream> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::anyhow!("No output device available"))?;

    let config = device.default_output_config()?;

    // Create the model inside the stream builder closure?
    // No, model needs to be created and moved into the closure.
    let mut model = AudioModel::new(width, height, cmd_rx, snap_tx);

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                model.process(data);
            },
            err_fn,
            None,
        )?,
        _ => return Err(anyhow::anyhow!("Only F32 sample format supported")),
    };

    stream.play()?;
    Ok(stream)
}
