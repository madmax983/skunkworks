use bevy::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Resource, Default)]
pub struct AudioSpectrum {
    pub data: Vec<f32>,
}

#[cfg(feature = "audio")]
#[derive(Resource)]
struct AudioStreamHandle {
    _stream: cpal::Stream,
    buffer: Arc<Mutex<Vec<f32>>>,
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioSpectrum>()
           .add_systems(Startup, setup_audio)
           .add_systems(Update, update_spectrum);
    }
}

fn setup_audio(mut commands: Commands) {
    #[cfg(feature = "audio")]
    {
        use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
        use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit, scaling::divide_by_N};

        let host = cpal::default_host();
        let device = host.default_input_device();

        let buffer = Arc::new(Mutex::new(Vec::new()));
        let buffer_clone = buffer.clone();

        if let Some(device) = device {
            info!("Audio input device found: {}", device.name().unwrap_or("Unknown".into()));
            if let Ok(config) = device.default_input_config() {
                let sample_rate = config.sample_rate().0;
                let stream_config: cpal::StreamConfig = config.clone().into();

                let stream = device.build_input_stream(
                    &stream_config,
                    move |data: &[f32], _: &_| {
                        // FFT needs power of 2 length
                        let n = 1024;
                        if data.len() >= n {
                            // Only use the first n samples
                            let spectrum = samples_fft_to_spectrum(
                                &data[0..n],
                                sample_rate,
                                FrequencyLimit::All,
                                Some(&divide_by_N),
                            );
                            if let Ok(spectrum) = spectrum {
                                if let Ok(mut guard) = buffer_clone.lock() {
                                    *guard = spectrum.data().iter().map(|(_, val)| val.val()).collect();
                                }
                            }
                        }
                    },
                    |err| error!("Stream error: {}", err),
                    None
                );

                match stream {
                    Ok(s) => {
                        if let Ok(_) = s.play() {
                            commands.insert_resource(AudioStreamHandle {
                                _stream: s,
                                buffer,
                            });
                            return;
                        }
                    },
                    Err(e) => error!("Failed to build stream: {}", e),
                }
            }
        } else {
            warn!("No input device found.");
        }
        // If we reach here, initialization failed. Ghost mode will be active because handle resource is missing.
    }

    #[cfg(not(feature = "audio"))]
    {
        info!("Audio feature disabled. Ghost Mode active.");
    }
}

fn update_spectrum(
    mut spectrum: ResMut<AudioSpectrum>,
    time: Res<Time>,
    #[cfg(feature = "audio")]
    handle: Option<Res<AudioStreamHandle>>,
) {
    let mut use_ghost_mode = true;

    #[cfg(feature = "audio")]
    if let Some(handle) = handle {
        if let Ok(guard) = handle.buffer.lock() {
            if !guard.is_empty() {
                spectrum.data = guard.clone();
                use_ghost_mode = false;
            }
        }
    }

    if use_ghost_mode {
        // Ghost mode: Generate synthetic spectrum
        let t = time.elapsed_seconds();
        spectrum.data = (0..64).map(|i| {
            let x = i as f32;
            // Create some moving peaks simulating a "breathing" pattern
            let p1 = (((x - 10.0 - (t * 2.0).sin() * 5.0).powi(2)) / -20.0).exp();
            let p2 = (((x - 40.0 - (t * 1.5).cos() * 10.0).powi(2)) / -30.0).exp();
            // Add some noise
            let noise = (x * t * 13.0).sin().abs() * 0.1;
            (p1 + p2 * 0.8 + noise).clamp(0.0, 1.0)
        }).collect();
    }
}
