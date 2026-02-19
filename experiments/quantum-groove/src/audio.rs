use std::f32::consts::PI;
use std::sync::mpsc::Receiver;
use hound;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub enum Waveform {
    Sine,
    Square,
    Sawtooth,
    Noise,
}

#[derive(Debug, Clone, Copy)]
pub struct AudioEvent {
    pub waveform: Waveform,
    pub frequency: f32,
    pub duration: f32,
    pub volume: f32,
    pub start_time: f32, // Set by the writer when received
}

pub struct AudioWriter {
    writer: hound::WavWriter<std::io::BufWriter<std::fs::File>>,
    sample_rate: u32,
    current_time: f32,
    events: Vec<AudioEvent>,
    event_rx: Receiver<AudioEvent>,
}

impl AudioWriter {
    pub fn new(filename: &str, sample_rate: u32, event_rx: Receiver<AudioEvent>) -> anyhow::Result<Self> {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let writer = hound::WavWriter::create(filename, spec)?;
        Ok(Self {
            writer,
            sample_rate,
            current_time: 0.0,
            events: Vec::new(),
            event_rx,
        })
    }

    pub fn process_events(&mut self) -> anyhow::Result<()> {
        while let Ok(mut event) = self.event_rx.try_recv() {
            event.start_time = self.current_time;
            self.events.push(event);
        }
        Ok(())
    }

    pub fn generate_chunk(&mut self, duration: f32) -> anyhow::Result<()> {
        let num_samples = (duration * self.sample_rate as f32) as usize;
        let dt = 1.0 / self.sample_rate as f32;

        for _ in 0..num_samples {
            let mut sample = 0.0;
            let time = self.current_time;

            for event in &self.events {
                let local_time = time - event.start_time;
                if local_time < 0.0 || local_time > event.duration {
                    continue;
                }

                // ADSR Envelope (Simple AR)
                let attack = 0.01;
                let release = 0.05;
                let envelope = if local_time < attack {
                    local_time / attack
                } else if local_time > event.duration - release {
                    (event.duration - local_time) / release
                } else {
                    1.0
                };

                let signal = match event.waveform {
                    Waveform::Sine => (2.0 * PI * event.frequency * local_time).sin(),
                    Waveform::Square => if (2.0 * PI * event.frequency * local_time).sin() > 0.0 { 1.0 } else { -1.0 },
                    Waveform::Sawtooth => {
                         let period = 1.0 / event.frequency;
                         let cycle_pos = (local_time % period) / period;
                         2.0 * cycle_pos - 1.0
                    },
                    Waveform::Noise => {
                        let mut rng = rand::thread_rng();
                        (rng.gen::<f32>() * 2.0) - 1.0
                    },
                };

                sample += signal * event.volume * envelope;
            }

            // Hard clipper / Limiter
            if sample > 0.95 { sample = 0.95; }
            if sample < -0.95 { sample = -0.95; }

            let amplitude = i16::MAX as f32;
            self.writer.write_sample((sample * amplitude) as i16)?;

            self.current_time += dt;
        }

        // Cleanup finished events
        let now = self.current_time;
        self.events.retain(|e| now < e.start_time + e.duration);

        Ok(())
    }
}
