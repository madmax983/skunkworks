use ::rand::prelude::*;
use crossbeam_channel::bounded;
use ferrous_core::Platter;
use locus::Vec2;
use macroquad::prelude::*;
#[cfg(feature = "audio")]
use resonance_audio::audio::AudioSnapshot;
use resonance_audio::audio::{AudioCommand, AudioModel};

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[cfg(feature = "audio")]
use crossbeam_channel::Sender;

const GRID_WIDTH: usize = 128;
const GRID_HEIGHT: usize = 128;
const SAMPLE_RATE: f32 = 44100.0;
const PARTICLE_COUNT: usize = 2_000;
const WORLD_WIDTH: f64 = 128.0;
const WORLD_HEIGHT: f64 = 128.0;

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
}

impl Particle {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::zero(),
            acc: Vec2::zero(),
        }
    }
}

#[cfg(feature = "audio")]
fn init_audio(
    cmd_rx: crossbeam_channel::Receiver<AudioCommand>,
    snap_tx: crossbeam_channel::Sender<AudioSnapshot>,
) -> anyhow::Result<cpal::Stream> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::anyhow!("No output device found"))?;

    let config = device.default_output_config()?;
    let sample_rate = config.sample_rate().0 as f32;
    let mut model = AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx, Some(sample_rate));

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            model.process(data);
        },
        move |err| {
            eprintln!("Audio stream error: {}", err);
        },
        None,
    )?;

    stream.play()?;
    Ok(stream)
}

#[macroquad::main("Ferrous Tank")]
async fn main() {
    let (cmd_tx, cmd_rx) = bounded(1024);
    let (snap_tx, snap_rx) = bounded(2);

    #[cfg(feature = "audio")]
    let _stream = match init_audio(cmd_rx, snap_tx.clone()) {
        Ok(s) => Some(s),
        Err(e) => {
            eprintln!("Audio init failed: {}. Running in silent mode.", e);
            None
        }
    };

    #[cfg(not(feature = "audio"))]
    let mut model = AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx, None);

    let mut world = World::new(WORLD_WIDTH, WORLD_HEIGHT);

    let mut image = Image::gen_image_color(GRID_WIDTH as u16, GRID_HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut current_snapshot: Vec<f32> = vec![0.0; GRID_WIDTH * GRID_HEIGHT];

    loop {
        #[cfg(not(feature = "audio"))]
        {
            let steps = (SAMPLE_RATE / 60.0) as usize;
            let mut dummy_output = [0.0; 2];
            for _ in 0..steps {
                model.process(&mut dummy_output);
            }
        }

        while let Ok(snap) = snap_rx.try_recv() {
            current_snapshot = snap.pressure;
        }

        world.wave_heights.copy_from_slice(&current_snapshot);

        let dt = 0.05;
        world.update(dt, &cmd_tx);

        let mouse_pos = mouse_position();
        let world_mouse = vec2(
            mouse_pos.0 / screen_width() * WORLD_WIDTH as f32,
            mouse_pos.1 / screen_height() * WORLD_HEIGHT as f32,
        );

        if is_mouse_button_down(MouseButton::Left) {
            let x = world_mouse.x as usize;
            let y = world_mouse.y as usize;
            if x < GRID_WIDTH && y < GRID_HEIGHT {
                let _ = cmd_tx.try_send(AudioCommand::Pluck {
                    x,
                    y,
                    strength: 100.0,
                });
            }
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            world.add_magnet(world_mouse.x as f64, world_mouse.y as f64, true);
        }
        if is_mouse_button_pressed(MouseButton::Middle) {
            world.add_magnet(world_mouse.x as f64, world_mouse.y as f64, false);
        }

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let h = current_snapshot[y * GRID_WIDTH + x];
                let color_val = ((h + 1.0) * 0.5 * 255.0).clamp(0.0, 255.0) as u8;
                image.set_pixel(
                    x as u32,
                    y as u32,
                    Color::from_rgba(color_val, color_val, 255, 255),
                );
            }
        }
        texture.update(&image);

        clear_background(BLACK);

        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        for p in &world.particles {
            let screen_x = p.pos.x as f32 / WORLD_WIDTH as f32 * screen_width();
            let screen_y = p.pos.y as f32 / WORLD_HEIGHT as f32 * screen_height();
            draw_circle(screen_x, screen_y, 2.0, Color::new(0.0, 1.0, 1.0, 1.0));
            // CYAN
        }

        for mag in &world.magnets {
            let screen_x = mag.pos.x as f32 / WORLD_WIDTH as f32 * screen_width();
            let screen_y = mag.pos.y as f32 / WORLD_HEIGHT as f32 * screen_height();
            let color = if mag.polarity { RED } else { BLUE };
            draw_circle(screen_x, screen_y, 5.0, color);
        }

        draw_text(
            "Ferrous Tank: Magnetic Acoustic Resonance",
            10.0,
            20.0,
            30.0,
            WHITE,
        );
        draw_text(
            &format!(
                "Particles: {} | Magnets: {}",
                PARTICLE_COUNT,
                world.magnets.len()
            ),
            10.0,
            50.0,
            20.0,
            WHITE,
        );
        draw_text(
            "Left Click: Pluck | Right Click: Add N Magnet | Middle Click: Add S Magnet",
            10.0,
            80.0,
            20.0,
            WHITE,
        );
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 110.0, 20.0, WHITE);

        next_frame().await;
    }
}

pub struct Magnet {
    pub pos: Vec2,
    pub strength: f64,
    pub polarity: bool, // true = North (Pull), false = South (Push)
}

pub struct World {
    pub particles: Vec<Particle>,
    pub magnets: Vec<Magnet>,
    pub width: f64,
    pub height: f64,
    pub platter: Platter,
    pub wave_heights: Vec<f32>,
}

impl World {
    pub fn new(width: f64, height: f64) -> Self {
        let mut particles = Vec::with_capacity(PARTICLE_COUNT);
        let mut rng = ::rand::thread_rng();

        for _ in 0..PARTICLE_COUNT {
            particles.push(Particle::new(
                rng.gen_range(width * 0.1..width * 0.9),
                rng.gen_range(height * 0.1..height * 0.9),
            ));
        }

        let grid_w = width as usize + 1;
        let grid_h = height as usize + 1;

        Self {
            particles,
            magnets: Vec::new(),
            width,
            height,
            platter: Platter::new(grid_w, grid_h),
            wave_heights: vec![0.0; GRID_WIDTH * GRID_HEIGHT],
        }
    }

    pub fn add_magnet(&mut self, x: f64, y: f64, polarity: bool) {
        self.magnets.push(Magnet {
            pos: Vec2::new(x, y),
            strength: 2000.0,
            polarity,
        });
    }

    pub fn update(&mut self, dt: f64, audio_cmd_tx: &crossbeam_channel::Sender<AudioCommand>) {
        let gravity = Vec2::new(0.0, 0.0); // No gravity in the wave tank, it's top-down
        let damping = 0.96;

        self.platter.clear();

        for p in &self.particles {
            let gx = p.pos.x.round() as usize;
            let gy = p.pos.y.round() as usize;
            if gx < self.platter.width() && gy < self.platter.height() {
                self.platter.accumulate(gx, gy, 1.0);
            }
        }

        let mut updates = Vec::new();

        for i in 0..self.particles.len() {
            let mut force = gravity;

            for mag in &self.magnets {
                let delta = mag.pos - self.particles[i].pos;
                let dist_sq = delta.magnitude_squared();

                if dist_sq > 1.0 {
                    let dir = delta.normalize();
                    let mag_force = (mag.strength / dist_sq).min(200.0);

                    if mag.polarity {
                        force = force + dir * mag_force;
                    } else {
                        force = force - dir * mag_force;
                    }
                }
            }

            let p_pos = self.particles[i].pos;
            let gx = p_pos.x.round() as usize;
            let gy = p_pos.y.round() as usize;

            if gx > 0
                && gx < (self.platter.width() - 1)
                && gy > 0
                && gy < (self.platter.height() - 1)
            {
                let left = self.platter.get_magnetism(gx - 1, gy);
                let right = self.platter.get_magnetism(gx + 1, gy);
                let down = self.platter.get_magnetism(gx, gy - 1);
                let up = self.platter.get_magnetism(gx, gy + 1);

                let dx = right - left;
                let dy = up - down;

                // Push away from high density of particles
                let pressure_force = Vec2::new(-dx, -dy) * 50.0;
                force = force + pressure_force;
            }

            // --- Wave Interaction (The Hybrid Trait) ---
            if gx < GRID_WIDTH && gy < GRID_HEIGHT {
                // Acoustic wave pushes particles based on wave gradient
                let left_w = if gx > 0 {
                    self.wave_heights[gy * GRID_WIDTH + (gx - 1)]
                } else {
                    0.0
                };
                let right_w = if gx < GRID_WIDTH - 1 {
                    self.wave_heights[gy * GRID_WIDTH + (gx + 1)]
                } else {
                    0.0
                };
                let up_w = if gy > 0 {
                    self.wave_heights[(gy - 1) * GRID_WIDTH + gx]
                } else {
                    0.0
                };
                let down_w = if gy < GRID_HEIGHT - 1 {
                    self.wave_heights[(gy + 1) * GRID_WIDTH + gx]
                } else {
                    0.0
                };

                let wave_dx = right_w - left_w;
                let wave_dy = down_w - up_w; // In macroquad, down is +y.

                let acoustic_force = Vec2::new(-wave_dx as f64, -wave_dy as f64) * 500.0;
                force = force + acoustic_force;

                // Fast particles create acoustic plucks
                if self.particles[i].vel.magnitude_squared() > 100.0
                    && ::rand::thread_rng().gen_bool(0.01)
                {
                    updates.push((gx, gy, 0.2));
                }
            }

            self.particles[i].acc = force;
        }

        for p in &mut self.particles {
            p.vel = p.vel + p.acc * dt;
            p.vel = p.vel * damping;
            p.pos = p.pos + p.vel * dt;

            if p.pos.y < 0.0 {
                p.pos.y = 0.0;
                p.vel.y = -p.vel.y * 0.6;
            }
            if p.pos.x < 0.0 {
                p.pos.x = 0.0;
                p.vel.x = -p.vel.x * 0.6;
            }
            if p.pos.x >= self.width {
                p.pos.x = self.width - 0.1;
                p.vel.x = -p.vel.x * 0.6;
            }
            if p.pos.y >= self.height {
                p.pos.y = self.height - 0.1;
                p.vel.y = -p.vel.y * 0.6;
            }
        }

        for (x, y, force) in updates {
            let _ = audio_cmd_tx.try_send(AudioCommand::Pluck {
                x,
                y,
                strength: force,
            });
        }
    }
}
