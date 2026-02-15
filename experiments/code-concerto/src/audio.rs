use crate::model::{Instrument, MusicalEvent};
use anyhow::Result;

#[cfg(feature = "audio")]
use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
#[cfg(feature = "audio")]
use std::time::Duration;

pub struct AudioEngine {
    #[cfg(feature = "audio")]
    _stream: OutputStream,
    #[cfg(feature = "audio")]
    stream_handle: OutputStreamHandle,
}

impl AudioEngine {
    pub fn new() -> Result<Self> {
        #[cfg(feature = "audio")]
        {
            let (stream, stream_handle) = OutputStream::try_default()?;
            Ok(Self {
                _stream: stream,
                stream_handle,
            })
        }
        #[cfg(not(feature = "audio"))]
        {
            Ok(Self {})
        }
    }

    pub fn play_event(&mut self, event: &MusicalEvent) {
        #[cfg(feature = "audio")]
        {
            match event {
                MusicalEvent::NoteOn {
                    instrument,
                    pitch,
                    volume,
                    duration,
                } => {
                    if let Ok(sink) = Sink::try_new(&self.stream_handle) {
                        let vol = *volume;
                        let dur = *duration;
                        let freq = *pitch;

                        match *instrument {
                            Instrument::Pad => {
                                let source = rodio::source::SineWave::new(freq)
                                    .take_duration(dur)
                                    .amplify(vol)
                                    .fade_in(Duration::from_millis(100))
                                    .fade_out(Duration::from_millis(100));
                                sink.append(source);
                            }
                            Instrument::Bass => {
                                let source = rodio::source::SineWave::new(freq)
                                    .take_duration(dur)
                                    .amplify(vol)
                                    .fade_in(Duration::from_millis(10))
                                    .fade_out(Duration::from_millis(50));
                                sink.append(source);
                            }
                            Instrument::Lead => {
                                let source = rodio::source::SineWave::new(freq)
                                    .take_duration(dur)
                                    .amplify(vol)
                                    .fade_in(Duration::from_millis(5))
                                    .fade_out(Duration::from_millis(10));
                                sink.append(source);
                            }
                            _ => {
                                let source = rodio::source::SineWave::new(freq)
                                    .take_duration(dur)
                                    .amplify(vol)
                                    .fade_in(Duration::from_millis(10))
                                    .fade_out(Duration::from_millis(10));
                                sink.append(source);
                            }
                        }

                        sink.detach();
                    }
                }
                MusicalEvent::Wait(_) => {}
            }
        }
        #[cfg(not(feature = "audio"))]
        {
            let _ = event;
        }
    }
}
