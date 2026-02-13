use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[cfg(feature = "real_audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[cfg(feature = "real_audio")]
use spectrum_analyzer::scaling::divide_by_N;
#[cfg(feature = "real_audio")]
use spectrum_analyzer::windows::hann_window;
#[cfg(feature = "real_audio")]
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};

#[derive(Debug, Clone, Copy)]
pub struct AudioFeatures {
    pub bass: f32,   // 20Hz - 250Hz
    pub mids: f32,   // 250Hz - 2000Hz
    pub treble: f32, // 2000Hz - 20000Hz
}

impl Default for AudioFeatures {
    fn default() -> Self {
        Self {
            bass: 0.0,
            mids: 0.0,
            treble: 0.0,
        }
    }
}

pub struct AudioEngine {
    features: Arc<Mutex<AudioFeatures>>,
    #[cfg(feature = "real_audio")]
    _stream: Option<cpal::Stream>,
    #[cfg(not(feature = "real_audio"))]
    _stream: (), // Placeholder
}

impl AudioEngine {
    pub fn new() -> Self {
        Self {
            features: Arc::new(Mutex::new(AudioFeatures::default())),
            #[cfg(feature = "real_audio")]
            _stream: None,
            #[cfg(not(feature = "real_audio"))]
            _stream: (),
        }
    }

    pub fn get_features(&self) -> AudioFeatures {
        *self.features.lock().unwrap()
    }

    pub fn start(&mut self) {
        #[cfg(feature = "real_audio")]
        self.start_real_audio();

        #[cfg(not(feature = "real_audio"))]
        {
            log::warn!("Real Audio feature disabled. Starting Ghost Mode.");
            self.start_ghost_mode();
        }
    }

    #[cfg(feature = "real_audio")]
    fn start_real_audio(&mut self) {
        let host = cpal::default_host();
        // Attempt to find default input device
        let device = match host.default_input_device() {
            Some(d) => d,
            None => {
                log::warn!("No audio input device found. Engaging Ghost Mode.");
                self.start_ghost_mode();
                return;
            }
        };

        log::info!(
            "Using audio device: {}",
            device.name().unwrap_or("Unknown".into())
        );

        let config = match device.default_input_config() {
            Ok(c) => c,
            Err(e) => {
                log::error!(
                    "Failed to get default input config: {:?}. Engaging Ghost Mode.",
                    e
                );
                self.start_ghost_mode();
                return;
            }
        };

        // We only support F32 for simplicity in this moonshot.
        if config.sample_format() != cpal::SampleFormat::F32 {
            log::warn!(
                "Device does not support F32 natively. Engaging Ghost Mode (Converter TODO)."
            );
            self.start_ghost_mode();
            return;
        }

        let config: cpal::StreamConfig = config.into();
        let features_handle = self.features.clone();
        let sample_rate = config.sample_rate.0 as f32;
        let channels = config.channels as usize;

        // Buffer for FFT
        let fft_size = 2048;
        let mut buffer = Vec::with_capacity(fft_size);

        let err_fn = move |err| {
            log::error!("an error occurred on stream: {}", err);
        };

        let stream = device.build_input_stream(
            &config,
            move |data: &[f32], _: &_| {
                for frame in data.chunks(channels) {
                    let sample = frame.iter().sum::<f32>() / channels as f32;
                    buffer.push(sample);

                    if buffer.len() >= fft_size {
                        let window = hann_window(&buffer);
                        let spectrum = samples_fft_to_spectrum(
                            &window,
                            sample_rate as u32,
                            FrequencyLimit::All,
                            Some(&divide_by_N),
                        );

                        if let Ok(spec) = spectrum {
                            let mut bass = 0.0;
                            let mut mids = 0.0;
                            let mut treble = 0.0;

                            for (freq, val) in spec.data() {
                                let f = freq.val();
                                let v = val.val();
                                if f < 250.0 {
                                    bass += v;
                                } else if f < 2000.0 {
                                    mids += v;
                                } else if f < 20000.0 {
                                    treble += v;
                                }
                            }

                            let mut f = features_handle.lock().unwrap();
                            f.bass = bass * 10.0;
                            f.mids = mids * 10.0;
                            f.treble = treble * 10.0;
                        }

                        buffer.clear();
                    }
                }
            },
            err_fn,
            None,
        );

        match stream {
            Ok(s) => {
                s.play().unwrap();
                self._stream = Some(s);
            }
            Err(e) => {
                log::error!("Failed to build stream: {:?}. Engaging Ghost Mode.", e);
                self.start_ghost_mode();
            }
        }
    }

    fn start_ghost_mode(&self) {
        let features = self.features.clone();
        thread::spawn(move || {
            let start = Instant::now();
            loop {
                thread::sleep(Duration::from_millis(16));
                let t = start.elapsed().as_secs_f32();

                // Simulate a beat: 120 BPM = 2 Hz
                // Bass kick every 0.5s
                let kick = (t * 2.0 * std::f32::consts::PI).sin().max(0.0).powf(10.0);

                // Hi-hats every 0.125s
                let hats = (t * 8.0 * std::f32::consts::PI).sin().max(0.0).powf(20.0);

                // Mids are slow swelling pad
                let pads = (t * 0.5).sin() * 0.5 + 0.5;

                let mut f = features.lock().unwrap();
                f.bass = kick * 0.8;
                f.mids = pads * 0.4;
                f.treble = hats * 0.6;
            }
        });
    }
}
