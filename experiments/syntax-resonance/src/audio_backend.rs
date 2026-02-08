use anyhow::Result;
use crossbeam_channel::{Receiver, Sender};
use resonance_audio::audio::{AudioCommand, AudioModel};

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub struct AudioSystem {
    #[cfg(feature = "audio")]
    pub stream: Option<cpal::Stream>,
    pub model: Option<AudioModel>,
}

impl AudioSystem {
    pub fn new(
        width: usize,
        height: usize,
        cmd_rx: Receiver<AudioCommand>,
        snap_tx: Sender<Vec<f32>>,
    ) -> Result<Self> {
        #[cfg(feature = "audio")]
        {
            let host = cpal::default_host();
            // Try to initialize audio
            if let Some(device) = host.default_output_device() {
                if let Ok(config) = device.default_output_config() {
                    let mut model = AudioModel::new(width, height, cmd_rx, snap_tx);
                    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

                    let stream_res = match config.sample_format() {
                        cpal::SampleFormat::F32 => device.build_output_stream(
                            &config.into(),
                            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                                model.process(data);
                            },
                            err_fn,
                            None,
                        ),
                        _ => Err(cpal::BuildStreamError::StreamConfigNotSupported),
                    };

                    if let Ok(stream) = stream_res {
                        stream.play()?;
                        return Ok(Self {
                            stream: Some(stream),
                            model: None,
                        });
                    }
                }
            }
            eprintln!("Audio initialization failed. Falling back to silent simulation.");
        }

        // Fallback
        let model = AudioModel::new(width, height, cmd_rx, snap_tx);
        Ok(Self {
            #[cfg(feature = "audio")]
            stream: None,
            model: Some(model),
        })
    }
}
