use anyhow::Result;
use crossbeam_channel::{Receiver, Sender};
use resonance_audio::audio::{AudioCommand, AudioModel, AudioSnapshot};
use std::thread;
use std::time::Duration;

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub struct AudioHandle {
    #[cfg(feature = "audio")]
    _stream: Option<cpal::Stream>,
    _simulation_thread: Option<thread::JoinHandle<()>>,
}

pub fn init_audio(
    cmd_rx: Receiver<AudioCommand>,
    snap_tx: Sender<AudioSnapshot>,
) -> Result<AudioHandle> {
    #[cfg(feature = "audio")]
    {
        match setup_cpal(cmd_rx.clone(), snap_tx.clone()) {
            Ok(stream) => return Ok(AudioHandle {
                _stream: Some(stream),
                _simulation_thread: None,
            }),
            Err(e) => {
                eprintln!("Audio init failed: {}. Falling back to simulation.", e);
            }
        }
    }

    // Fallback or non-audio mode
    // We consume cmd_rx, so we must be the only one.
    // If cpal setup failed, cmd_rx is still valid (cloned? no, it's Receiver).
    // Wait, Receiver is not Clone.
    // So we can't clone Receiver easily unless we use `crossbeam_channel::unbounded`?
    // `bounded` returns `Receiver` which is Clone.
    // `std::sync::mpsc::Receiver` is NOT Clone.
    // `crossbeam_channel::Receiver` IS Clone.

    let sim_thread = thread::spawn(move || {
        let mut model = AudioModel::new(100, 100, cmd_rx, snap_tx);
        let mut dummy_buffer = vec![0.0; 1024];
        loop {
            model.process(&mut dummy_buffer);
            // Throttle simulation to avoid 100% CPU
            // 1024 samples at 44100Hz is ~23ms
            thread::sleep(Duration::from_millis(20));
        }
    });

    Ok(AudioHandle {
        #[cfg(feature = "audio")]
        _stream: None,
        _simulation_thread: Some(sim_thread),
    })
}

#[cfg(feature = "audio")]
fn setup_cpal(
    cmd_rx: Receiver<AudioCommand>,
    snap_tx: Sender<AudioSnapshot>,
) -> Result<cpal::Stream> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::anyhow!("No output device available"))?;

    let config = device.default_output_config()?;

    // We assume f32 samples for simplicity, as Resonance Audio outputs f32.
    // If device doesn't support f32, we'd need conversion.
    // For this moonshot, we'll error out if not f32.

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => {
            let mut model = AudioModel::new(100, 100, cmd_rx, snap_tx);
            device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    model.process(data);
                },
                err_fn,
                None,
            )?
        },
        _ => return Err(anyhow::anyhow!("Only F32 sample format supported")),
    };

    stream.play()?;
    Ok(stream)
}
