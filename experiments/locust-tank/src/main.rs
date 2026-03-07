use ::rand::prelude::*;
use crossbeam_channel::bounded;
use macroquad::prelude::*;
#[cfg(feature = "audio")]
use resonance_audio::audio::AudioSnapshot;
use resonance_audio::audio::{AudioCommand, AudioModel};

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

const GRID_WIDTH: usize = 128;
const GRID_HEIGHT: usize = 128;
const SAMPLE_RATE: f32 = 44100.0;
const AGENT_COUNT: usize = 10_000;
const WORLD_SIZE: f32 = 128.0; // Map 1:1 with grid for simplicity
const SPEED: f32 = 1.0;

#[derive(Clone, Copy)]
pub struct Locust {
    pub pos: Vec2,
    pub vel: Vec2,
}

pub struct World {
    pub agents: Vec<Locust>,
    pub grid_w: usize,
    pub grid_h: usize,
    pub target: Vec2,
}

impl World {
    pub fn new() -> Self {
        let mut rng = ::rand::thread_rng();
        let mut agents = Vec::with_capacity(AGENT_COUNT);

        for _ in 0..AGENT_COUNT {
            let x = rng.gen_range(0.0..WORLD_SIZE);
            let y = rng.gen_range(0.0..WORLD_SIZE);

            agents.push(Locust {
                pos: vec2(x, y),
                vel: vec2(0.0, 0.0),
            });
        }

        World {
            agents,
            grid_w: GRID_WIDTH,
            grid_h: GRID_HEIGHT,
            target: vec2(WORLD_SIZE / 2.0, WORLD_SIZE / 2.0),
        }
    }

    pub fn update(&mut self, pressure: &[f32], cmd_tx: &crossbeam_channel::Sender<AudioCommand>) {
        let target = self.target;
        let w = self.grid_w;
        let h = self.grid_h;

        // Collect plucks to send to audio thread
        let mut plucks = Vec::new();

        for agent in &mut self.agents {
            // Seek Target roughly
            let to_target = target - agent.pos;
            let dist_target = to_target.length();

            let mut desire = if dist_target > 0.0 {
                to_target.normalize() * SPEED
            } else {
                vec2(0.0, 0.0)
            };

            // Read wave pressure at agent position
            let gx = agent.pos.x.floor() as isize;
            let gy = agent.pos.y.floor() as isize;

            if gx >= 0 && gx < w as isize && gy >= 0 && gy < h as isize {
                let p = pressure[(gy as usize) * w + (gx as usize)];
                // Pressure acts as a repulsive force
                if p.abs() > 0.1 {
                    // Try to flow down the gradient (away from high pressure)
                    // We approximate by just being pushed randomly or outward
                    let mut rng = ::rand::thread_rng();
                    let push_dir = vec2(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0))
                        .normalize_or_zero();
                    desire += push_dir * p * 5.0; // Repel based on pressure magnitude
                }
            }

            // Simple steering
            agent.vel = agent.vel.lerp(desire, 0.1);
            let next_pos = agent.pos + agent.vel;

            // Boundary collision
            let mut hit_wall = false;
            if next_pos.x <= 0.0 || next_pos.x >= WORLD_SIZE - 1.0 {
                agent.vel.x *= -1.0;
                hit_wall = true;
            }
            if next_pos.y <= 0.0 || next_pos.y >= WORLD_SIZE - 1.0 {
                agent.vel.y *= -1.0;
                hit_wall = true;
            }

            agent.pos += agent.vel;
            agent.pos.x = agent.pos.x.clamp(0.0, WORLD_SIZE - 1.1);
            agent.pos.y = agent.pos.y.clamp(0.0, WORLD_SIZE - 1.1);

            if hit_wall {
                plucks.push((agent.pos.x as usize, agent.pos.y as usize, 0.5));
            }
        }

        // Send a limited number of plucks to avoid overloading the channel
        let mut rng = ::rand::thread_rng();
        plucks.shuffle(&mut rng);
        for (px, py, strength) in plucks.into_iter().take(10) {
            let _ = cmd_tx.send(AudioCommand::Pluck {
                x: px,
                y: py,
                strength,
            });
        }
    }
}

#[macroquad::main("Locust Tank")]
async fn main() {
    let (cmd_tx, cmd_rx) = bounded(1024);
    let (snap_tx, snap_rx) = bounded(2);

    #[cfg(feature = "audio")]
    let (mut _stream, mut fallback_model) = match init_audio(cmd_rx.clone(), snap_tx.clone()) {
        Ok((s, _)) => (Some(s), None),
        Err(e) => {
            eprintln!("Audio init failed: {}. Running in silent mode.", e);
            (
                None,
                Some(AudioModel::new(
                    GRID_WIDTH,
                    GRID_HEIGHT,
                    cmd_rx,
                    snap_tx.clone(),
                    None,
                )),
            )
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
        while let Ok(snap) = snap_rx.try_recv() {
            current_snapshot = snap.pressure;
        }

        #[cfg(not(feature = "audio"))]
        {
            let dt = get_frame_time();
            let steps = (dt * SAMPLE_RATE) as usize;
            let steps = steps.min(2000);
            let mut dummy_buffer = vec![0.0; steps];
            model.process(&mut dummy_buffer);
        }

        #[cfg(feature = "audio")]
        if let Some(ref mut fallback) = fallback_model {
            let dt = get_frame_time();
            let steps = (dt * SAMPLE_RATE) as usize;
            let steps = steps.min(2000);
            let mut dummy_buffer = vec![0.0; steps];
            fallback.process(&mut dummy_buffer);
        }

        // Update swarm physics, passing the current pressure field
        world.update(&current_snapshot, &cmd_tx);

        // Update texture with waves
        for (i, &val) in current_snapshot.iter().enumerate() {
            let x = (i % GRID_WIDTH) as u32;
            let y = (i / GRID_WIDTH) as u32;

            let color = if val > 0.0 {
                Color::new(val.min(1.0), 0.0, 0.0, 1.0)
            } else {
                Color::new(0.0, 0.0, (-val).min(1.0), 1.0)
            };
            image.set_pixel(x, y, color);
        }

        // Overlay agents
        for agent in &world.agents {
            let x = agent.pos.x as u32;
            let y = agent.pos.y as u32;
            if x < GRID_WIDTH as u32 && y < GRID_HEIGHT as u32 {
                image.set_pixel(x, y, GREEN);
            }
        }

        texture.update(&image);

        clear_background(BLACK);

        let scale = (screen_width() / GRID_WIDTH as f32).min(screen_height() / GRID_HEIGHT as f32);
        let draw_width = GRID_WIDTH as f32 * scale;
        let draw_height = GRID_HEIGHT as f32 * scale;
        let offset_x = (screen_width() - draw_width) / 2.0;
        let offset_y = (screen_height() - draw_height) / 2.0;

        draw_texture_ex(
            &texture,
            offset_x,
            offset_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(draw_width, draw_height)),
                ..Default::default()
            },
        );

        let mouse_pos = mouse_position();
        if mouse_pos.0 >= offset_x
            && mouse_pos.0 < offset_x + draw_width
            && mouse_pos.1 >= offset_y
            && mouse_pos.1 < offset_y + draw_height
        {
            let grid_x = ((mouse_pos.0 - offset_x) / scale) as usize;
            let grid_y = ((mouse_pos.1 - offset_y) / scale) as usize;

            if is_mouse_button_down(MouseButton::Left) {
                let _ = cmd_tx.send(AudioCommand::Pluck {
                    x: grid_x,
                    y: grid_y,
                    strength: 0.5,
                });
                world.target = vec2(grid_x as f32, grid_y as f32);
            }
        }

        if is_key_pressed(KeyCode::Space) {
            let _ = cmd_tx.send(AudioCommand::ClearWaves);
        }
        if is_key_pressed(KeyCode::C) {
            let _ = cmd_tx.send(AudioCommand::ClearWalls);
        }
        if is_key_pressed(KeyCode::R) {
            world = World::new();
        }

        draw_text(
            "L-Click: Pluck & Attract Swarm | Space: Clear Waves | C: Clear Walls | R: Reset",
            10.0,
            20.0,
            20.0,
            WHITE,
        );

        next_frame().await
    }
}

#[cfg(feature = "audio")]
fn init_audio(
    cmd_rx: crossbeam_channel::Receiver<AudioCommand>,
    snap_tx: crossbeam_channel::Sender<AudioSnapshot>,
) -> anyhow::Result<(cpal::Stream, AudioModel)> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::anyhow!("No output device available"))?;

    let config = device.default_output_config()?;

    let mut model = AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx, None);

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    // To prevent the model from being moved entirely and causing problems on fallback,
    // we would ideally run it in a separate thread. Since `resonance-audio`'s AudioModel
    // expects to be driven directly via `process`, we handle it in the audio thread.
    // However, if we fail to initialize audio, we need to return an error *before* moving model.

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                model.process(data);
            },
            err_fn,
            None,
        )?,
        _ => return Err(anyhow::anyhow!("Only F32 sample format supported")),
    };

    stream.play()?;
    // We can't return model here since it's moved into the closure, but if the closure
    // is successfully created, we don't need the model for manual fallback anyway.
    Ok((
        stream,
        AudioModel::new(
            0,
            0,
            crossbeam_channel::bounded(1).1,
            crossbeam_channel::bounded(1).0,
            None,
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_init() {
        let world = World::new();
        assert_eq!(world.agents.len(), AGENT_COUNT);
    }
}
