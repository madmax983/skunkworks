use rodio::{source::Source, OutputStream, OutputStreamHandle, Sink};
use rodio::source::{SineWave, WhiteNoise};
use std::time::Duration;

pub struct Synth {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
}

impl Synth {
    pub fn new() -> Option<Self> {
        match OutputStream::try_default() {
            Ok((_stream, stream_handle)) => Some(Self {
                _stream,
                stream_handle,
            }),
            Err(_) => None,
        }
    }

    pub fn play_kick(&self) {
        let source = SineWave::new(60.0)
            .take_duration(Duration::from_millis(150))
            .amplify(0.8);

        let sink = Sink::try_new(&self.stream_handle).unwrap();
        sink.append(source);
        sink.detach();
    }

    pub fn play_snare(&self) {
        let source = WhiteNoise::new()
            .take_duration(Duration::from_millis(100))
            .amplify(0.5);

        let sink = Sink::try_new(&self.stream_handle).unwrap();
        sink.append(source);
        sink.detach();
    }

    pub fn play_hihat(&self) {
        let source = WhiteNoise::new()
            .take_duration(Duration::from_millis(50))
            .amplify(0.3);

        let sink = Sink::try_new(&self.stream_handle).unwrap();
        sink.append(source);
        sink.detach();
    }

    pub fn play_bass(&self, freq: f32) {
         let source = SineWave::new(freq)
            .take_duration(Duration::from_millis(300))
            .amplify(0.6);

        let sink = Sink::try_new(&self.stream_handle).unwrap();
        sink.append(source);
        sink.detach();
    }
}
