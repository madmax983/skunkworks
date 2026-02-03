use crate::dsp::KarplusStrong;
use anyhow::Result;

pub trait AudioSystem {
    fn pluck(&self, index: usize);
    fn set_strings(&self, strings: Vec<KarplusStrong>);
    fn get_visual_state(&self) -> Vec<f32>;
    fn update(&self); // Used for dummy system simulation step
}

#[cfg(feature = "audio")]
pub use self::real::RealAudioSystem as AudioEngine;

#[cfg(not(feature = "audio"))]
pub use self::dummy::DummyAudioSystem as AudioEngine;

#[cfg(feature = "audio")]
mod real {
    use super::*;
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use std::sync::mpsc::{channel, Sender};
    use std::sync::{Arc, Mutex};

    enum AudioCommand {
        Pluck(usize),
        SetStrings(Vec<KarplusStrong>),
    }

    pub struct RealAudioSystem {
        command_tx: Sender<AudioCommand>,
        visual_state: Arc<Mutex<Vec<f32>>>,
        _stream: cpal::Stream,
    }

    impl RealAudioSystem {
        pub fn new() -> Result<Self> {
            let host = cpal::default_host();
            let device = host.default_output_device()
                .ok_or_else(|| anyhow::anyhow!("No output device available"))?;
            let config = device.default_output_config()?;
            // let sample_rate = config.sample_rate().0 as f32; // Unused
            let channels = config.channels() as usize;

            let (tx, rx) = channel::<AudioCommand>();
            let visual_state = Arc::new(Mutex::new(Vec::new()));
            let visual_state_clone = visual_state.clone();

            let mut strings: Vec<KarplusStrong> = Vec::new();

            // Pre-allocate buffer for visualization state to avoid allocation in loop
            let mut current_amps_buffer: Vec<f32> = Vec::new();

            let stream = device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    // Process commands
                    while let Ok(cmd) = rx.try_recv() {
                        match cmd {
                            AudioCommand::Pluck(idx) => {
                                if idx < strings.len() {
                                    strings[idx].pluck();
                                }
                            }
                            AudioCommand::SetStrings(new_strings) => {
                                strings = new_strings;
                                // Resize buffers
                                current_amps_buffer.resize(strings.len(), 0.0);
                                if let Ok(mut state) = visual_state_clone.lock() {
                                    *state = vec![0.0; strings.len()];
                                }
                            }
                        }
                    }

                    // Reset amps buffer
                    current_amps_buffer.fill(0.0);
                    // Ensure buffer size matches strings (in case SetStrings wasn't called but strings changed - unlikely here but safe)
                    if current_amps_buffer.len() != strings.len() {
                        current_amps_buffer.resize(strings.len(), 0.0);
                    }

                    for frame in data.chunks_mut(channels) {
                        let mut mix = 0.0;
                        for (i, string) in strings.iter_mut().enumerate() {
                            let val = string.tick();
                            mix += val;
                            // Track max amplitude for visualization
                            let abs_val = val.abs();
                            if abs_val > current_amps_buffer[i] {
                                current_amps_buffer[i] = abs_val;
                            }
                        }

                        // Soft clip
                        mix = mix.tanh();

                        for sample in frame.iter_mut() {
                            *sample = mix;
                        }
                    }

                    // Update shared visual state
                    if let Ok(mut state) = visual_state_clone.try_lock() {
                        if state.len() == current_amps_buffer.len() {
                            *state.as_mut_slice().copy_from_slice(&current_amps_buffer);
                        }
                    }
                },
                |err| eprintln!("Audio stream error: {}", err),
                None,
            )?;

            stream.play()?;

            Ok(Self {
                command_tx: tx,
                visual_state,
                _stream: stream,
            })
        }
    }

    impl AudioSystem for RealAudioSystem {
        fn pluck(&self, index: usize) {
            let _ = self.command_tx.send(AudioCommand::Pluck(index));
        }

        fn set_strings(&self, strings: Vec<KarplusStrong>) {
            let _ = self.command_tx.send(AudioCommand::SetStrings(strings));
        }

        fn get_visual_state(&self) -> Vec<f32> {
            self.visual_state.lock().unwrap_or_else(|e| e.into_inner()).clone()
        }

        fn update(&self) {
            // Real audio system updates automatically
        }
    }
}

#[cfg(not(feature = "audio"))]
mod dummy {
    use super::*;
    use std::sync::Mutex;

    pub struct DummyAudioSystem {
        strings: Mutex<Vec<KarplusStrong>>,
    }

    impl DummyAudioSystem {
        pub fn new() -> Result<Self> {
            Ok(Self {
                strings: Mutex::new(Vec::new()),
            })
        }
    }

    impl AudioSystem for DummyAudioSystem {
        fn pluck(&self, index: usize) {
            let mut strings = self.strings.lock().unwrap();
            if index < strings.len() {
                strings[index].pluck();
            }
        }

        fn set_strings(&self, new_strings: Vec<KarplusStrong>) {
            *self.strings.lock().unwrap() = new_strings;
        }

        fn get_visual_state(&self) -> Vec<f32> {
            let strings = self.strings.lock().unwrap();
            strings.iter().map(|s| s.current_value().abs()).collect()
        }

        fn update(&self) {
            // Manually tick physics since there's no audio callback
            let mut strings = self.strings.lock().unwrap();
            for s in strings.iter_mut() {
                s.tick();
            }
        }
    }
}
