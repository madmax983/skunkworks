use ::rand::prelude::*;
use crossbeam_channel::bounded;
use macroquad::prelude::*;
use std::f32::consts::PI;

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
const AGENT_COUNT: usize = 3_000;
const WORLD_SIZE: f32 = 128.0;

#[derive(Clone, Copy)]
pub struct Agent {
    pub pos: Vec2,
    pub angle: f32,
    pub speed: f32,
}

impl Agent {
    pub fn new() -> Self {
        let mut rng = ::rand::thread_rng();
        Self {
            pos: Vec2::new(
                rng.gen_range(0.0..WORLD_SIZE),
                rng.gen_range(0.0..WORLD_SIZE),
            ),
            angle: rng.gen_range(0.0..2.0 * PI),
            speed: rng.gen_range(0.5..1.5),
        }
    }

    pub fn sense(&self, wave_heights: &[f32], angle_offset: f32, sensor_dist: f32) -> f32 {
        let sensor_angle = self.angle + angle_offset;
        let sensor_x = self.pos.x + sensor_angle.cos() * sensor_dist;
        let sensor_y = self.pos.y + sensor_angle.sin() * sensor_dist;

        let w = GRID_WIDTH as f32;
        let h = GRID_HEIGHT as f32;

        let sx = (sensor_x.rem_euclid(w)) as usize;
        let sy = (sensor_y.rem_euclid(h)) as usize;

        if sx < GRID_WIDTH && sy < GRID_HEIGHT {
            wave_heights[sy * GRID_WIDTH + sx]
        } else {
            0.0
        }
    }
}

pub struct World {
    pub agents: Vec<Agent>,
    pub wave_heights: Vec<f32>,
}

impl World {
    pub fn new() -> Self {
        let mut agents = Vec::with_capacity(AGENT_COUNT);
        for _ in 0..AGENT_COUNT {
            agents.push(Agent::new());
        }

        Self {
            agents,
            wave_heights: vec![0.0; GRID_WIDTH * GRID_HEIGHT],
        }
    }

    pub fn update(&mut self, audio_cmd_tx: &crossbeam_channel::Sender<AudioCommand>) {
        let mut rng = ::rand::thread_rng();
        let sensor_angle = PI / 4.0;
        let sensor_dist = 4.0;
        let turn_angle = PI / 8.0;

        // Collect new positions and deposits
        let mut updates = Vec::with_capacity(self.agents.len());

        for agent in self.agents.iter_mut() {
            // Sense the wave tank heights (gradient climbing)
            let left = agent.sense(&self.wave_heights, -sensor_angle, sensor_dist);
            let center = agent.sense(&self.wave_heights, 0.0, sensor_dist);
            let right = agent.sense(&self.wave_heights, sensor_angle, sensor_dist);

            // Myco-transit logic: move towards higher concentration (or in this case, wave pressure)
            if center > left && center > right {
                // Keep going
            } else if center < left && center < right {
                // Random turn
                if rng.gen_bool(0.5) {
                    agent.angle += turn_angle;
                } else {
                    agent.angle -= turn_angle;
                }
            } else if left > right {
                agent.angle -= turn_angle;
            } else if right > left {
                agent.angle += turn_angle;
            }

            // Move
            agent.pos.x += agent.angle.cos() * agent.speed;
            agent.pos.y += agent.angle.sin() * agent.speed;

            // Wrap
            agent.pos.x = agent.pos.x.rem_euclid(WORLD_SIZE);
            agent.pos.y = agent.pos.y.rem_euclid(WORLD_SIZE);

            // Deposit "pheromone" which is a wave pluck
            let ix = agent.pos.x as usize;
            let iy = agent.pos.y as usize;

            if ix < GRID_WIDTH && iy < GRID_HEIGHT {
                updates.push((ix, iy, 0.5)); // Deposit 0.5 pressure
            }
        }

        // Send plucks to audio thread
        for (x, y, force) in updates {
            let _ = audio_cmd_tx.try_send(AudioCommand::Pluck {
                x,
                y,
                strength: force,
            });
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

#[macroquad::main("Myco-Tank")]
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

    let mut world = World::new();

    let mut image = Image::gen_image_color(GRID_WIDTH as u16, GRID_HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut current_snapshot: Vec<f32> = vec![0.0; GRID_WIDTH * GRID_HEIGHT];

    loop {
        #[cfg(not(feature = "audio"))]
        {
            // If no audio, advance model manually
            let steps = (SAMPLE_RATE / 60.0) as usize;
            let mut dummy_output = [0.0; 2];
            for _ in 0..steps {
                model.process(&mut dummy_output);
            }
        }

        // Try to get latest snapshot from audio thread
        while let Ok(snap) = snap_rx.try_recv() {
            current_snapshot = snap.pressure;
        }

        // Update wave_heights in world for agent sensing
        world.wave_heights.copy_from_slice(&current_snapshot);

        // Input
        let mouse_pos = mouse_position();
        let world_mouse = vec2(
            mouse_pos.0 / screen_width() * WORLD_SIZE,
            mouse_pos.1 / screen_height() * WORLD_SIZE,
        );

        if is_mouse_button_down(MouseButton::Left) {
            let x = world_mouse.x as usize;
            let y = world_mouse.y as usize;
            let _ = cmd_tx.try_send(AudioCommand::Pluck {
                x,
                y,
                strength: 100.0,
            });
        }

        // Update agents
        world.update(&cmd_tx);

        // Render Wave Tank
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

        // Render Agents
        for agent in &world.agents {
            let screen_x = agent.pos.x / WORLD_SIZE * screen_width();
            let screen_y = agent.pos.y / WORLD_SIZE * screen_height();
            draw_circle(screen_x, screen_y, 2.0, YELLOW);
        }

        draw_text("Myco-Tank", 10.0, 20.0, 30.0, WHITE);
        draw_text(
            &format!("Agents: {}", AGENT_COUNT),
            10.0,
            50.0,
            20.0,
            WHITE,
        );
        draw_text(
            &format!("FPS: {}", get_fps()),
            10.0,
            80.0,
            20.0,
            WHITE,
        );

        next_frame().await;
    }
}