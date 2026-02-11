mod physics;
use physics::{VocalTract, Glottis};
use macroquad::prelude::*;
use std::collections::VecDeque;
use std::io::BufWriter;
use std::fs::File;

const SAMPLE_RATE: f32 = 44100.0;
const BUFFER_SIZE: usize = 1024; // For visualization
const SAMPLES_PER_FRAME: usize = 735; // 44100 / 60

struct AudioEngine {
    tract: VocalTract,
    glottis: Glottis,
    buffer: VecDeque<f32>,
    writer: Option<hound::WavWriter<BufWriter<File>>>,
    recording: bool,
}

impl AudioEngine {
    fn new() -> Self {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: SAMPLE_RATE as u32,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };

        let writer = hound::WavWriter::create("output.wav", spec).ok();

        Self {
            tract: VocalTract::new(44), // ~17cm tract
            glottis: Glottis::new(120.0, SAMPLE_RATE), // 120Hz fundamental
            buffer: VecDeque::with_capacity(BUFFER_SIZE),
            writer,
            recording: true,
        }
    }

    fn update(&mut self, samples: usize) {
        for _ in 0..samples {
            // Glottal source (sawtooth/pulse)
            let glottal_out = self.glottis.step();

            // Physics step
            self.tract.step(glottal_out);

            // Output
            let sample = self.tract.output_sample;

            // Limit amplitude (soft clip)
            let sample = sample.clamp(-1.0, 1.0);

            // Write to WAV
            if self.recording {
                if let Some(writer) = &mut self.writer {
                    // Ignore write errors to keep running
                    let _ = writer.write_sample(sample);
                }
            }

            // Visualization Buffer
            if self.buffer.len() >= BUFFER_SIZE {
                self.buffer.pop_front();
            }
            self.buffer.push_back(sample);
        }
    }
}

#[macroquad::main("Silicon Larynx")]
async fn main() {
    let mut engine = AudioEngine::new();

    // Hardcoded source text for now (the physics file)
    let source_code = include_str!("physics.rs");
    let mut cursor = 0;

    loop {
        clear_background(BLACK);

        // 1. Update Physics
        engine.update(SAMPLES_PER_FRAME);

        // 2. Read Source Code & Modulate Tract
        if cursor < source_code.len() {
            let char = source_code.chars().nth(cursor).unwrap_or(' ');

            // Map char to tract shape
            // Vowels -> Open area (larger A)
            // Consonants -> Constrictions (smaller A)
            // Code structure (indents, brackets) -> Resonances?

            let target_area = match char {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => 3.0,
                ' ' | '\n' | '\t' => 1.5, // Neutral
                '{' | '}' | '(' | ')' => 4.0, // Resonant cavity
                _ => 0.5, // Consonant constriction
            };

            // Apply to specific regions based on char hash/type?
            // For now, simple uniform interpolation toward target
            // Maybe vary the *location* of constriction based on char value?
            // char as u8 % n_tubes

            let constriction_loc = (char as usize) % engine.tract.n_tubes;

            for (i, area) in engine.tract.areas.iter_mut().enumerate() {
                let mut target = 1.5; // Default open

                // Formant logic:
                // Vowels open the front
                // Consonants close specific spots

                if i == constriction_loc {
                    target = target_area;
                } else {
                    // Neighbor smoothing
                    if (i as isize - constriction_loc as isize).abs() < 3 {
                         target = (target_area + 1.5) / 2.0;
                    }
                }

                *area = *area * 0.9 + target * 0.1;
            }

            // Draw current char being read
            draw_text(&format!("Reading: '{}'", char), 20.0, 50.0, 40.0, WHITE);

            cursor += 1;
        } else {
            cursor = 0; // Loop
        }

        // 3. Draw Tract (Visualization)
        let center_y = screen_height() / 2.0;
        let scale_x = screen_width() / engine.tract.n_tubes as f32;
        let scale_y = 50.0;

        for (i, area) in engine.tract.areas.iter().enumerate() {
            let x = i as f32 * scale_x;
            let h = area * scale_y;
            let y = center_y - h / 2.0;

            // Draw Tube Segment
            draw_rectangle(x, y, scale_x, h, Color::new(0.8, 0.4, 0.4, 1.0)); // Flesh color
            draw_rectangle_lines(x, y, scale_x, h, 2.0, RED);

            // Visualize Pressure (Standing Waves)
            let pressure = engine.tract.forward[i] + engine.tract.backward[i];
            let intensity = (pressure.abs() * 2.0).clamp(0.0, 1.0);
            if intensity > 0.01 {
                draw_rectangle(x, y, scale_x, h, Color::new(0.0, 0.0, 1.0, intensity * 0.5));
            }
        }

        // 4. Draw Waveform (Oscilloscope)
        let waveform_y = screen_height() - 100.0;
        let mut prev_x = 0.0;
        let mut prev_y = waveform_y;

        for (i, sample) in engine.buffer.iter().enumerate() {
            let x = i as f32 / BUFFER_SIZE as f32 * screen_width();
            let y = waveform_y + sample * 50.0;
            draw_line(prev_x, prev_y, x, y, 1.0, GREEN);
            prev_x = x;
            prev_y = y;
        }

        draw_text("Silicon Larynx - Reading `physics.rs`", 20.0, 20.0, 20.0, GRAY);
        draw_text("Output: output.wav", 20.0, screen_height() - 20.0, 20.0, GRAY);

        next_frame().await
    }
}
