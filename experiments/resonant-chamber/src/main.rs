use crossbeam_channel::{bounded, unbounded};
use macroquad::prelude::*;
use resonance_audio::audio::{AudioCommand, AudioSnapshot};
use resonance_audio::physics::Material;
use rustfft::{num_complex::Complex, FftPlanner};

mod audio;
use audio::init_audio;

const GRID_W: usize = 80;
const GRID_H: usize = 60;
const FFT_SIZE: usize = 4096;

#[derive(PartialEq, Clone, Copy)]
enum Tool {
    Wall,
    Air,
    Source,
    Listener,
}

#[derive(PartialEq)]
enum AppState {
    Interactive,
    Analyzing,
}

struct Analyzer {
    planner: FftPlanner<f32>,
    buffer: Vec<f32>,
    spectrum: Vec<f32>,
    recording: bool,
    samples_needed: usize,
}

impl Analyzer {
    fn new() -> Self {
        Self {
            planner: FftPlanner::new(),
            buffer: Vec::with_capacity(FFT_SIZE),
            spectrum: Vec::new(),
            recording: false,
            samples_needed: 0,
        }
    }

    fn start_recording(&mut self, count: usize) {
        self.buffer.clear();
        self.recording = true;
        self.samples_needed = count;
    }

    fn push_samples(&mut self, samples: &[f32]) -> bool {
        if !self.recording {
            return false;
        }

        self.buffer.extend_from_slice(samples);

        if self.buffer.len() >= self.samples_needed {
            self.recording = false;
            self.compute_fft();
            return true; // Finished
        }
        false
    }

    fn compute_fft(&mut self) {
        let fft = self.planner.plan_fft_forward(self.buffer.len());
        let mut input: Vec<Complex<f32>> = self
            .buffer
            .iter()
            .map(|&x| Complex::new(x, 0.0))
            .collect();

        // Apply Hanning window
        let len = input.len();
        for (i, val) in input.iter_mut().enumerate() {
            let window = 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / (len - 1) as f32).cos());
            *val = *val * window;
        }

        fft.process(&mut input);

        // Compute magnitude
        self.spectrum = input
            .iter()
            .take(len / 2) // Only first half (Nyquist)
            .map(|c| c.norm())
            .collect();

        // Normalize
        let max_val = self.spectrum.iter().cloned().fold(0.0f32, f32::max);
        if max_val > 0.0 {
            for v in &mut self.spectrum {
                *v /= max_val;
            }
        }
    }
}

#[macroquad::main("Resonant Chamber")]
async fn main() {
    let (cmd_tx, cmd_rx) = bounded(1024);
    let (snap_tx, snap_rx) = bounded(2);
    // Use unbounded for recording to avoid dropping samples during bursts,
    // though in practice bounded(100) is fine if we consume fast enough.
    let (rec_tx, rec_rx) = unbounded();

    // Init Audio
    let _audio_sys = match init_audio(GRID_W, GRID_H, cmd_rx, snap_tx, rec_tx) {
        Ok(sys) => sys,
        Err(e) => {
            eprintln!("Audio init failed: {}", e);
            return;
        }
    };

    // State
    let mut current_tool = Tool::Wall;
    let mut state = AppState::Interactive;
    let mut analyzer = Analyzer::new();

    // Visuals
    let mut snapshot: Option<AudioSnapshot> = None;
    let mut source_pos = (GRID_W / 4, GRID_H / 2);
    let mut listener_pos = (3 * GRID_W / 4, GRID_H / 2);

    // Initial setup
    let _ = cmd_tx.send(AudioCommand::MoveListener {
        x: listener_pos.0,
        y: listener_pos.1,
    });
    // Walls around the border
    for x in 0..GRID_W {
        let _ = cmd_tx.send(AudioCommand::AddWall { x, y: 0 });
        let _ = cmd_tx.send(AudioCommand::AddWall { x, y: GRID_H - 1 });
    }
    for y in 0..GRID_H {
        let _ = cmd_tx.send(AudioCommand::AddWall { x: 0, y });
        let _ = cmd_tx.send(AudioCommand::AddWall { x: GRID_W - 1, y });
    }

    loop {
        // 1. Handle Recording Data
        while let Ok(samples) = rec_rx.try_recv() {
            if analyzer.push_samples(&samples) {
                // Analysis finished
                state = AppState::Interactive;
                println!("Analysis complete. Spectrum size: {}", analyzer.spectrum.len());
            }
        }

        // 2. Handle Snapshots
        while let Ok(snap) = snap_rx.try_recv() {
            snapshot = Some(snap);
        }

        // 3. Layout
        let total_w = screen_width();
        let total_h = screen_height();

        // Split screen: Left 70% for Grid, Right 30% for Controls/Graph?
        // Or Top 70% Grid, Bottom 30% Graph.
        let split_y = total_h * 0.7;
        let control_h = total_h - split_y;

        // 4. Draw Grid
        let cell_w = total_w / GRID_W as f32;
        let cell_h = split_y / GRID_H as f32; // Use top part for grid

        clear_background(BLACK);

        if let Some(snap) = &snapshot {
            for y in 0..GRID_H {
                for x in 0..GRID_W {
                    let idx = y * GRID_W + x;
                    let mat = snap.materials[idx];
                    let pressure = snap.pressure[idx];

                    let rx = x as f32 * cell_w;
                    let ry = y as f32 * cell_h;

                    let color = match mat {
                        Material::Wall => GRAY,
                        Material::Slow => DARKBLUE,
                        Material::Void => BLACK,
                        _ => {
                            let p = pressure.clamp(-1.0, 1.0);
                            if p > 0.0 {
                                Color::new(p, 0.0, 0.0, 1.0)
                            } else {
                                Color::new(0.0, 0.0, -p, 1.0)
                            }
                        }
                    };
                    draw_rectangle(rx, ry, cell_w, cell_h, color);
                }
            }
        }

        // Draw Source and Listener
        let sx = source_pos.0 as f32 * cell_w + cell_w / 2.0;
        let sy = source_pos.1 as f32 * cell_h + cell_h / 2.0;
        draw_circle(sx, sy, cell_w * 0.8, YELLOW);
        draw_text("S", sx - 5.0, sy + 5.0, 15.0, BLACK);

        let lx = listener_pos.0 as f32 * cell_w + cell_w / 2.0;
        let ly = listener_pos.1 as f32 * cell_h + cell_h / 2.0;
        draw_circle(lx, ly, cell_w * 0.8, GREEN);
        draw_text("L", lx - 5.0, ly + 5.0, 15.0, BLACK);

        // 5. Interaction
        let (mx, my) = mouse_position();
        if my < split_y {
            let gx = (mx / cell_w) as usize;
            let gy = (my / cell_h) as usize;

            if gx < GRID_W && gy < GRID_H {
                if is_mouse_button_down(MouseButton::Left) {
                    match current_tool {
                        Tool::Wall => {
                            let _ = cmd_tx.send(AudioCommand::AddWall { x: gx, y: gy });
                        }
                        Tool::Air => {
                            let _ = cmd_tx.send(AudioCommand::RemoveWall { x: gx, y: gy });
                        }
                        Tool::Source => {
                            source_pos = (gx, gy);
                        }
                        Tool::Listener => {
                            listener_pos = (gx, gy);
                            let _ = cmd_tx.send(AudioCommand::MoveListener { x: gx, y: gy });
                        }
                    }
                }

                // Right click triggers simple pluck for testing
                if is_mouse_button_pressed(MouseButton::Right) {
                     let _ = cmd_tx.send(AudioCommand::Pluck {
                        x: gx,
                        y: gy,
                        strength: 0.8,
                    });
                }
            }
        }

        // 6. UI Panel
        draw_rectangle(0.0, split_y, total_w, control_h, DARKGRAY);

        // Buttons
        let btn_w = 100.0;
        let btn_h = 30.0;
        let start_x = 10.0;
        let start_y = split_y + 10.0;

        // Tool Selection
        let tools = [
            (Tool::Wall, "Wall"),
            (Tool::Air, "Eraser"),
            (Tool::Source, "Source"),
            (Tool::Listener, "Listener"),
        ];

        for (i, (tool, label)) in tools.iter().enumerate() {
            let x = start_x + i as f32 * (btn_w + 10.0);
            let y = start_y;
            let color = if current_tool == *tool { WHITE } else { LIGHTGRAY };

            draw_rectangle(x, y, btn_w, btn_h, color);
            draw_text(label, x + 10.0, y + 20.0, 20.0, BLACK);

            if mx >= x && mx <= x + btn_w && my >= y && my <= y + btn_h && is_mouse_button_pressed(MouseButton::Left) {
                current_tool = *tool;
            }
        }

        // Analyze Button
        let analyze_x = start_x;
        let analyze_y = start_y + btn_h + 10.0;
        let analyze_color = if state == AppState::Analyzing { RED } else { BLUE };
        draw_rectangle(analyze_x, analyze_y, btn_w, btn_h, analyze_color);
        draw_text("Analyze", analyze_x + 10.0, analyze_y + 20.0, 20.0, WHITE);

        if mx >= analyze_x && mx <= analyze_x + btn_w && my >= analyze_y && my <= analyze_y + btn_h && is_mouse_button_pressed(MouseButton::Left) {
            if state == AppState::Interactive {
                state = AppState::Analyzing;
                // Trigger analysis
                let _ = cmd_tx.send(AudioCommand::ClearWaves);
                // Give it a moment to clear (this is async but fast enough)
                // Actually, ClearWaves is instant in audio thread.
                // Trigger Pluck
                let _ = cmd_tx.send(AudioCommand::Pluck {
                    x: source_pos.0,
                    y: source_pos.1,
                    strength: 10.0, // Strong impulse
                });
                analyzer.start_recording(FFT_SIZE);
            }
        }

        // Clear Walls Button
        let clear_x = analyze_x + btn_w + 10.0;
        let clear_y = analyze_y;
        draw_rectangle(clear_x, clear_y, btn_w, btn_h, RED);
        draw_text("Clear All", clear_x + 10.0, clear_y + 20.0, 20.0, WHITE);

        if mx >= clear_x && mx <= clear_x + btn_w && my >= clear_y && my <= clear_y + btn_h && is_mouse_button_pressed(MouseButton::Left) {
            let _ = cmd_tx.send(AudioCommand::ClearWalls);
            // Rebuild border
            for x in 0..GRID_W {
                let _ = cmd_tx.send(AudioCommand::AddWall { x, y: 0 });
                let _ = cmd_tx.send(AudioCommand::AddWall { x, y: GRID_H - 1 });
            }
            for y in 0..GRID_H {
                let _ = cmd_tx.send(AudioCommand::AddWall { x: 0, y });
                let _ = cmd_tx.send(AudioCommand::AddWall { x: GRID_W - 1, y });
            }
        }

        // 7. Graph
        let graph_x = start_x + 4.0 * (btn_w + 10.0) + 20.0; // Right of buttons
        let graph_y = split_y + 10.0;
        let graph_w = total_w - graph_x - 10.0;
        let graph_h = control_h - 20.0;

        draw_rectangle(graph_x, graph_y, graph_w, graph_h, BLACK);
        draw_rectangle_lines(graph_x, graph_y, graph_w, graph_h, 1.0, WHITE);

        if !analyzer.spectrum.is_empty() {
            let step = graph_w / analyzer.spectrum.len() as f32;
            for (i, &val) in analyzer.spectrum.iter().enumerate() {
                // Logarithmic scale for magnitude helps
                let mag = val.sqrt();
                let h = mag * graph_h;
                draw_line(
                    graph_x + i as f32 * step,
                    graph_y + graph_h,
                    graph_x + i as f32 * step,
                    graph_y + graph_h - h,
                    1.0,
                    GREEN
                );
            }
            draw_text("Frequency Response", graph_x + 5.0, graph_y + 15.0, 15.0, WHITE);
        } else {
             draw_text("No Data", graph_x + graph_w/2.0 - 20.0, graph_y + graph_h/2.0, 20.0, GRAY);
        }

        next_frame().await;
    }
}
