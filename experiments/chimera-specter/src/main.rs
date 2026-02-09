use macroquad::prelude::*;
use chimera_lang::prelude::*;
use chimera_lang::vm::{ChimeraVM, Value};
use hound::WavSpec;
use std::collections::VecDeque;

mod synth;
use synth::SpecterSynth;

const SAMPLE_RATE: u32 = 44100;
const FFT_SIZE: usize = 1024;
const GRID_SIZE: usize = 16;

#[macroquad::main("Chimera Specter")]
async fn main() {
    // 1. Initialize DNA (The Musician)
    // A simple program that:
    // - Reads current position
    // - Adds 1 to grid cell
    // - Moves randomly
    // - Spawns offspring (Mitosis) if energy high
    // - Photosynthesizes

    let genes = vec![
        Gene { op: OpCode::Photosynthesize, args: vec![] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] },
        Gene { op: OpCode::Consume, args: vec![] }, // Eat stack value

        // Write to grid (Sound)
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(50)] },
        Gene { op: OpCode::Locate, args: vec![] }, // [50, y, x]
        Gene { op: OpCode::GWrite, args: vec![] },

        // Move randomly (Simple walk)
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // dy
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // dx
        Gene { op: OpCode::Migrate, args: vec![] },

        Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
    ];

    let dna = Dna { helix: Helix { strands: vec![Strand { genes }] } };
    let mut vm = ChimeraVM::new(dna);

    // 2. Initialize Synth
    let mut synth = SpecterSynth::new(FFT_SIZE);

    // 3. Audio Buffer
    let mut recording_buffer: Vec<f32> = Vec::new();
    let mut is_recording = false;
    let mut saved_message_timer = 0;

    // 4. Spectrogram History for Visualization
    let mut spectrogram_history: VecDeque<Vec<f32>> = VecDeque::new();
    let max_history = 100;

    loop {
        clear_background(BLACK);

        // --- Simulation Step ---
        // Run VM multiple times per frame to create evolving patterns
        for _ in 0..5 {
            vm.step();
        }

        // --- Audio Generation ---
        let samples = synth.grid_to_audio(&vm.grid);

        // Append to recording buffer if active
        if is_recording {
            recording_buffer.extend_from_slice(samples);
        }

        // Store for visualization (frequency magnitudes)
        let mut spectrum_slice = vec![0.0; GRID_SIZE];
        for (i, row) in vm.grid.iter().enumerate() {
            let mut amp = 0.0;
            for cell in row {
                 match cell {
                    Value::Int(v) => amp += (*v as f32).abs(),
                    _ => {},
                }
            }
            spectrum_slice[i] = amp;
        }

        spectrogram_history.push_back(spectrum_slice);
        if spectrogram_history.len() > max_history {
            spectrogram_history.pop_front();
        }

        // --- Rendering ---

        // Draw Spectrogram (Scrolling)
        let slice_w = screen_width() / max_history as f32;
        let cell_h = screen_height() / GRID_SIZE as f32;

        for (t, slice) in spectrogram_history.iter().enumerate() {
            for (freq_idx, amp) in slice.iter().enumerate() {
                let x = t as f32 * slice_w;
                // Spectrogram usually puts low freq at bottom
                // Our Row 0 is mapped to Low Freq (Bin 10)
                // So Row 0 should be at bottom (y = height - cell_h)
                let y = screen_height() - (freq_idx as f32 * cell_h) - cell_h;
                let color_val = (amp / 50.0).clamp(0.0, 1.0);
                draw_rectangle(x, y, slice_w, cell_h, Color::new(0.0, color_val, color_val * 0.5, 1.0));
            }
        }

        // Draw Grid Overlay (The VM)
        let grid_w = 200.0;
        let grid_h = 200.0;
        let start_x = 20.0;
        let start_y = 20.0;

        draw_rectangle_lines(start_x, start_y, grid_w, grid_h, 2.0, WHITE);

        let cell_size_w = grid_w / GRID_SIZE as f32;
        let cell_size_h = grid_h / GRID_SIZE as f32;

        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let val = &vm.grid[y][x];
                let color = match val {
                    Value::Int(0) => None,
                    Value::Int(_) => Some(Color::new(1.0, 0.0, 0.0, 0.5)),
                    Value::Str(_) => Some(Color::new(0.0, 1.0, 0.0, 0.5)),
                    _ => Some(Color::new(0.0, 0.0, 1.0, 0.5)),
                };

                if let Some(c) = color {
                    draw_rectangle(
                        start_x + x as f32 * cell_size_w,
                        start_y + y as f32 * cell_size_h,
                        cell_size_w,
                        cell_size_h,
                        c
                    );
                }
            }
        }

        // Draw Agents (Context Loc)
        let (cy, cx) = vm.context_loc;
        draw_circle(
            start_x + cx as f32 * cell_size_w + cell_size_w/2.0,
            start_y + cy as f32 * cell_size_h + cell_size_h/2.0,
            5.0,
            YELLOW
        );

        // UI
        draw_text("CHIMERA SPECTER", 20.0, 240.0, 30.0, WHITE);
        draw_text(format!("Energy: {}", vm.energy).as_str(), 20.0, 270.0, 20.0, WHITE);

        if is_recording {
            draw_text("RECORDING...", 20.0, 300.0, 20.0, RED);
            if is_key_pressed(KeyCode::R) {
                is_recording = false;
            }
        } else {
            draw_text("Press 'R' to Record", 20.0, 300.0, 20.0, GRAY);
            if is_key_pressed(KeyCode::R) {
                is_recording = true;
                recording_buffer.clear();
            }
        }

        if is_key_pressed(KeyCode::S) && !recording_buffer.is_empty() {
            save_wav("output.wav", &recording_buffer);
            saved_message_timer = 60;
        }

        if saved_message_timer > 0 {
            draw_text("SAVED output.wav", 20.0, 330.0, 20.0, GREEN);
            saved_message_timer -= 1;
        }

        next_frame().await
    }
}

fn save_wav(path: &str, samples: &[f32]) {
    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(path, spec).unwrap();
    for s in samples {
        let amp = (s * i16::MAX as f32).clamp(-i16::MAX as f32, i16::MAX as f32);
        writer.write_sample(amp as i16).unwrap();
    }
    writer.finalize().unwrap();
}
