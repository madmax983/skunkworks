
use std::thread;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Voice {
    Speaker(usize), // Speaker ID determines pitch
    Clash,
}

#[derive(Debug, Clone)]
pub struct AudioEvent {
    pub voice: Voice,
    pub distortion: f32, // 0.0 to 1.0+
}

pub struct AudioEngine {
    #[allow(dead_code)]
    tx: crossbeam_channel::Sender<AudioEvent>,
}

impl AudioEngine {
    pub fn new() -> anyhow::Result<Self> {
        let (tx, rx) = crossbeam_channel::unbounded();

        #[cfg(feature = "audio")]
        thread::spawn(move || {
            if let Err(e) = audio_loop(rx) {
                eprintln!("Audio loop error: {}", e);
            }
        });

        #[cfg(not(feature = "audio"))]
        {
            // Just drain the channel
            thread::spawn(move || {
                while let Ok(_) = rx.recv() {}
            });
        }

        Ok(Self { tx })
    }

    pub fn play(&self, voice: Voice, distortion: f32) {
        let _ = self.tx.send(AudioEvent { voice, distortion });
    }
}

#[cfg(feature = "audio")]
fn audio_loop(rx: Receiver<AudioEvent>) -> anyhow::Result<()> {
    use rodio::{source::SineWave, OutputStream, Sink, Source};
    use std::time::Duration;

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    while let Ok(event) = rx.recv() {
        let freq = match event.voice {
            Voice::Speaker(id) => 220.0 + (id as f32 * 55.0), // A3, C#4, E4, etc.
            Voice::Clash => 110.0, // A2 (Low rumble)
        };

        // Distortion adds noise or changes waveform?
        // Rodio supports basic sources. Let's just use SineWave and mix it.

        let duration = Duration::from_millis(100);
        let source = SineWave::new(freq)
            .take_duration(duration)
            .amplify(0.2);

        // If high distortion, maybe play a second dissonant tone?
        if event.distortion > 0.5 {
             let noise_freq = freq * (1.0 + event.distortion);
             let noise = SineWave::new(noise_freq)
                .take_duration(duration)
                .amplify(0.1);
             sink.append(source.mix(noise));
        } else {
             sink.append(source);
        }
    }

    Ok(())
}
