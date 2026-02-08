use macroquad::prelude::*;
use std::env;
use spectral_loading::codec::{decode, SAMPLE_RATE, WINDOW_SIZE};
use rustfft::{FftPlanner, num_complex::Complex};
use mlua::{Lua, Function};

#[cfg(feature = "audio_playback")]
use rodio::{OutputStream, Sink, buffer::SamplesBuffer};

struct Player {
    samples: Vec<f32>,
    position: usize,
    playing: bool,
    #[cfg(feature = "audio_playback")]
    _stream: Option<(OutputStream, rodio::OutputStreamHandle)>,
    #[cfg(feature = "audio_playback")]
    sink: Option<Sink>,
}

impl Player {
    fn new(samples: Vec<f32>) -> Self {
        Self {
            samples,
            position: 0,
            playing: false,
            #[cfg(feature = "audio_playback")]
            _stream: None,
            #[cfg(feature = "audio_playback")]
            sink: None,
        }
    }

    fn play(&mut self) {
        self.playing = true;
        self.position = 0;

        #[cfg(feature = "audio_playback")]
        {
            if let Ok((stream, handle)) = OutputStream::try_default() {
                let sink = Sink::try_new(&handle).ok();
                if let Some(sink) = sink {
                    let buffer = SamplesBuffer::new(1, SAMPLE_RATE, self.samples.clone());
                    sink.append(buffer);
                    sink.play();
                    self.sink = Some(sink);
                }
                self._stream = Some((stream, handle));
            } else {
                eprintln!("Failed to init audio output");
            }
        }
    }

    fn update(&mut self, dt: f32) {
        if self.playing {
            self.position += (SAMPLE_RATE as f32 * dt) as usize;
            if self.position >= self.samples.len() {
                self.position = 0; // Loop? Or Stop?
                // self.playing = false;

                // Restart playback for audio backend
                #[cfg(feature = "audio_playback")]
                if let Some(sink) = &self.sink {
                     if sink.empty() {
                         // Re-append
                         let buffer = SamplesBuffer::new(1, SAMPLE_RATE, self.samples.clone());
                         sink.append(buffer);
                     }
                }
            }
        }
    }

    fn get_current_window(&self) -> Vec<f32> {
        let start = self.position;
        if start + WINDOW_SIZE <= self.samples.len() {
            self.samples[start..start + WINDOW_SIZE].to_vec()
        } else {
            // Pad with zeros
            let mut window = Vec::with_capacity(WINDOW_SIZE);
            if start < self.samples.len() {
                window.extend_from_slice(&self.samples[start..]);
            }
            window.resize(WINDOW_SIZE, 0.0);
            window
        }
    }
}

#[macroquad::main("Spectral Loading")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut player: Option<Player> = None;
    let mut lua: Option<Lua> = None;
    let mut error_msg: Option<String> = None;

    // Try load from args
    if args.len() > 1 {
        match load_file(&args[1]) {
            Ok((p, l)) => {
                let mut p = p;
                p.play();
                player = Some(p);
                lua = Some(l);
            }
            Err(e) => error_msg = Some(format!("Error loading: {}", e)),
        }
    } else {
        error_msg = Some("Drag and Drop a .wav file".to_string());
    }

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(WINDOW_SIZE);
    let mut scratch = vec![Complex::new(0.0, 0.0); WINDOW_SIZE];

    loop {
        clear_background(BLACK);

        if let Some(p) = &mut player {
            p.update(get_frame_time());

            // FFT
            let window_samples = p.get_current_window();
            let mut input: Vec<Complex<f32>> = window_samples.iter().map(|&s| Complex::new(s, 0.0)).collect();
            fft.process_with_scratch(&mut input, &mut scratch);

            // Prepare FFT data for Lua
            let magnitudes: Vec<f32> = input.iter().take(WINDOW_SIZE/2).map(|c| c.norm()).collect();

            if let Some(l) = &lua {
                let globals = l.globals();

                // Set FFT data
                if let Ok(fft_table) = l.create_table() {
                    for (i, &mag) in magnitudes.iter().enumerate() {
                        let _ = fft_table.set(i + 1, mag);
                    }
                    let _ = globals.set("fft", fft_table);
                }

                let _ = globals.set("time", p.position as f32 / SAMPLE_RATE as f32);
                let _ = globals.set("screen_w", screen_width());
                let _ = globals.set("screen_h", screen_height());

                // Call draw
                if let Ok(draw_fn) = globals.get::<_, Function>("draw") {
                    if let Err(e) = draw_fn.call::<_, ()>(()) {
                        error_msg = Some(format!("Lua Runtime Error: {}", e));
                    }
                }
            }
        } else {
            // Show message
            draw_text("SPECTRAL LOADING", 20.0, 30.0, 40.0, WHITE);
            if let Some(msg) = &error_msg {
                draw_text(msg, 20.0, 60.0, 20.0, RED);
            }
        }

        // Handle file drop
        // macroquad::miniquad::window::dropped_file_count is not directly exposed in macroquad prelud?
        // Wait, macroquad doesn't expose `dropped_file_count` directly in prelude.
        // It's in `macroquad::input::utils`? No.
        // It's specific to miniquad.
        // But macroquad doesn't re-export everything nicely.
        // I'll skip drag-drop for now if I can't find it easily.
        // Actually, previous memory mentions `macroquad::miniquad::window`.
        // I need `use macroquad::miniquad::window::{dropped_file_count, dropped_file_path};` if available.
        // Let's rely on args for now.

        next_frame().await
    }
}

fn load_file(path: &str) -> anyhow::Result<(Player, Lua)> {
    let reader = hound::WavReader::open(path)?;
    let spec = reader.spec();
    let samples: Vec<f32> = if spec.sample_format == hound::SampleFormat::Int {
        reader.into_samples::<i16>()
            .map(|s| s.map(|x| x as f32 / 32768.0))
            .collect::<Result<Vec<f32>, _>>()?
    } else {
        reader.into_samples::<f32>()
            .collect::<Result<Vec<f32>, _>>()?
    };

    // Decode Payload
    let payload = decode(&samples).ok_or(anyhow::anyhow!("No hidden payload found!"))?;
    let script = String::from_utf8(payload)?;

    // Init Lua
    let lua = Lua::new();

    {
        let globals = lua.globals();

        // Bindings
        globals.set("clear_background", lua.create_function(|_, c: u32| {
            let r = ((c >> 16) & 0xFF) as f32 / 255.0;
            let g = ((c >> 8) & 0xFF) as f32 / 255.0;
            let b = (c & 0xFF) as f32 / 255.0;
            clear_background(Color::new(r, g, b, 1.0));
            Ok(())
        })?)?;

        globals.set("draw_line", lua.create_function(|_, (x1, y1, x2, y2, t, c): (f32, f32, f32, f32, f32, u32)| {
            let color = hex_to_color(c);
            draw_line(x1, y1, x2, y2, t, color);
            Ok(())
        })?)?;

        globals.set("draw_circle", lua.create_function(|_, (x, y, r, c): (f32, f32, f32, u32)| {
            let color = hex_to_color(c);
            draw_circle(x, y, r, color);
            Ok(())
        })?)?;

        globals.set("draw_text", lua.create_function(|_, (text, x, y, s, c): (String, f32, f32, f32, u32)| {
            let color = hex_to_color(c);
            draw_text(&text, x, y, s, color);
            Ok(())
        })?)?;

        // Color constants
        globals.set("BLACK", 0x000000)?;
        globals.set("WHITE", 0xFFFFFF)?;
        globals.set("RED", 0xFF0000)?;
        globals.set("GREEN", 0x00FF00)?;
        globals.set("BLUE", 0x0000FF)?;

        // Load script
        lua.load(&script).exec()?;
    }

    Ok((Player::new(samples), lua))
}

fn hex_to_color(c: u32) -> Color {
    let r = ((c >> 16) & 0xFF) as f32 / 255.0;
    let g = ((c >> 8) & 0xFF) as f32 / 255.0;
    let b = (c & 0xFF) as f32 / 255.0;
    Color::new(r, g, b, 1.0)
}
