use crossbeam_channel::bounded;
use macroquad::prelude::*;
use resonance_audio::audio::{AudioCommand, AudioModel};
use locus::Vec2;
use rayon::prelude::*;

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[cfg(feature = "audio")]
use crossbeam_channel::Sender;

const GRID_WIDTH: usize = 128;
const GRID_HEIGHT: usize = 128;
const SAMPLE_RATE: f32 = 44100.0;
const NUM_BOIDS: usize = 400;

#[derive(Clone, Copy)]
struct Boid {
    position: Vec2,
    velocity: Vec2,
    color: Color,
}

impl Default for Boid {
    fn default() -> Self {
        Self {
            position: Vec2::new(
                ::macroquad::rand::gen_range(0.0, GRID_WIDTH as f64),
                ::macroquad::rand::gen_range(0.0, GRID_HEIGHT as f64),
            ),
            velocity: Vec2::new(
                ::macroquad::rand::gen_range(-1.0, 1.0),
                ::macroquad::rand::gen_range(-1.0, 1.0),
            ).normalize() * 0.5,
            color: Color::new(
                ::macroquad::rand::gen_range(0.5, 1.0),
                ::macroquad::rand::gen_range(0.5, 1.0),
                ::macroquad::rand::gen_range(0.8, 1.0),
                1.0,
            ),
        }
    }
}

fn update_boids(boids: &mut [Boid], pressure_grid: &[f32], _dt: f32) {
    let cloned_boids = boids.to_vec();
    let width = GRID_WIDTH as f64;
    let height = GRID_HEIGHT as f64;

    boids.par_iter_mut().enumerate().for_each(|(i, boid)| {
        let mut separation = Vec2::new(0.0, 0.0);
        let mut alignment = Vec2::new(0.0, 0.0);
        let mut cohesion = Vec2::new(0.0, 0.0);
        let mut total = 0;

        let perception_radius = 5.0;

        for (j, other) in cloned_boids.iter().enumerate() {
            if i == j {
                continue;
            }
            let diff = boid.position - other.position;
            let dist = diff.magnitude();

            if dist < perception_radius {
                separation += diff.normalize() / dist.max(0.1);
                alignment += other.velocity;
                cohesion += other.position;
                total += 1;
            }
        }

        if total > 0 {
            alignment /= total as f64;
            alignment = (alignment - boid.velocity).normalize() * 0.05;

            cohesion /= total as f64;
            cohesion = (cohesion - boid.position).normalize() * 0.05;

            separation /= total as f64;
            separation = separation.normalize() * 0.08;
        }

        // Acoustic advection / pressure force
        // The boids are pushed by gradients in the pressure field
        let px = boid.position.x as isize;
        let py = boid.position.y as isize;
        let mut pressure_force = Vec2::new(0.0, 0.0);

        if px > 0 && px < (GRID_WIDTH - 1) as isize && py > 0 && py < (GRID_HEIGHT - 1) as isize {
            let px = px as usize;
            let py = py as usize;
            let p_left = pressure_grid[py * GRID_WIDTH + px - 1];
            let p_right = pressure_grid[py * GRID_WIDTH + px + 1];
            let p_up = pressure_grid[(py - 1) * GRID_WIDTH + px];
            let p_down = pressure_grid[(py + 1) * GRID_WIDTH + px];

            let grad_x = (p_right - p_left) as f64;
            let grad_y = (p_down - p_up) as f64;

            // Repelled by high pressure
            pressure_force = Vec2::new(-grad_x, -grad_y) * 20.0;
        }

        boid.velocity += alignment + cohesion + separation + pressure_force;

        // Max speed
        let speed = boid.velocity.magnitude();
        if speed > 1.5 {
            boid.velocity = (boid.velocity / speed) * 1.5;
        }

        boid.position += boid.velocity;

        // Plane bounce
        if boid.position.x < 1.0 {
            boid.position.x = 1.0;
            boid.velocity.x *= -1.0;
        } else if boid.position.x >= width - 1.0 {
            boid.position.x = width - 1.1;
            boid.velocity.x *= -1.0;
        }

        if boid.position.y < 1.0 {
            boid.position.y = 1.0;
            boid.velocity.y *= -1.0;
        } else if boid.position.y >= height - 1.0 {
            boid.position.y = height - 1.1;
            boid.velocity.y *= -1.0;
        }
    });
}

#[macroquad::main("Luminous Tank")]
async fn main() {
    // 1. Audio / Simulation Setup
    let (cmd_tx, cmd_rx) = bounded(1024);
    let (snap_tx, snap_rx) = bounded(2);


    #[cfg(feature = "audio")]
    let stream = match init_audio(cmd_rx.clone(), snap_tx.clone()) {
        Ok(s) => Some(s),
        Err(e) => {
            eprintln!("Audio init failed: {}. Running in silent mode.", e);
            None
        }
    };

    #[cfg(not(feature = "audio"))]
    let stream: Option<()> = None;

    let mut model = AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx, None);


    // 2. Visual Setup
    let mut image = Image::gen_image_color(GRID_WIDTH as u16, GRID_HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut current_snapshot: Vec<f32> = vec![0.0; GRID_WIDTH * GRID_HEIGHT];
    let listener_pos = (GRID_WIDTH / 2, GRID_HEIGHT / 2);

    let mut boids: Vec<Boid> = (0..NUM_BOIDS).map(|_| Boid::default()).collect();

    loop {
        // Poll for snapshot
        while let Ok(snap) = snap_rx.try_recv() {
            current_snapshot = snap.pressure;
        }

        let dt = get_frame_time();

        // Update boids
        update_boids(&mut boids, &current_snapshot, dt);

        // Boids pluck the grid
        for boid in &boids {
            // Randomly deposit energy to simulate moving through a fluid
            if ::macroquad::rand::gen_range(0.0, 1.0) < 0.05 {
                let px = boid.position.x.round() as usize;
                let py = boid.position.y.round() as usize;
                if px < GRID_WIDTH && py < GRID_HEIGHT {
                    let force = boid.velocity.magnitude() as f32 * 0.5;
                    let _ = cmd_tx.try_send(AudioCommand::Pluck {
                        x: px,
                        y: py,
                        strength: force,
                    });
                }
            }
        }


        if stream.is_none() {
            // Simulate steps for this frame
            let steps = (dt * SAMPLE_RATE) as usize;
            let steps = steps.min(2000);
            let mut dummy_buffer = vec![0.0; steps];
            model.process(&mut dummy_buffer);
        }


        // Update texture
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
        texture.update(&image);

        // Draw
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

        // Draw Boids
        for boid in &boids {
            let bx = offset_x + boid.position.x as f32 * scale;
            let by = offset_y + boid.position.y as f32 * scale;
            // Draw a small circle for the boid
            draw_circle(bx + scale / 2.0, by + scale / 2.0, scale.max(2.0), boid.color);
        }

        // Draw Listener
        let lx = offset_x + listener_pos.0 as f32 * scale;
        let ly = offset_y + listener_pos.1 as f32 * scale;
        draw_circle(lx + scale / 2.0, ly + scale / 2.0, scale / 2.0, GREEN);

        // Input
        let mouse_pos = mouse_position();
        if mouse_pos.0 >= offset_x
            && mouse_pos.0 < offset_x + draw_width
            && mouse_pos.1 >= offset_y
            && mouse_pos.1 < offset_y + draw_height
        {
            let grid_x = ((mouse_pos.0 - offset_x) / scale) as usize;
            let grid_y = ((mouse_pos.1 - offset_y) / scale) as usize;

            if is_mouse_button_down(MouseButton::Left) {
                let _ = cmd_tx.try_send(AudioCommand::Pluck {
                    x: grid_x,
                    y: grid_y,
                    strength: 5.0,
                });
            }
        }

        if is_key_pressed(KeyCode::Space) {
            let _ = cmd_tx.try_send(AudioCommand::ClearWaves);
        }
        if is_key_pressed(KeyCode::C) {
            let _ = cmd_tx.try_send(AudioCommand::ClearWalls);
        }
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        next_frame().await;
    }
}

#[cfg(feature = "audio")]
fn init_audio(
    cmd_rx: crossbeam_channel::Receiver<AudioCommand>,
    snap_tx: crossbeam_channel::Sender<resonance_audio::audio::AudioSnapshot>,
) -> anyhow::Result<cpal::Stream> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::anyhow!("No output device found"))?;

    let config = device.default_output_config()?;
    let sample_rate = config.sample_rate().0 as f32;
    let channels = config.channels() as usize;

    let mut model = AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx, Some(sample_rate));

    let err_fn = |err| eprintln!("An error occurred on the output audio stream: {}", err);

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let mut mix_buffer = vec![0.0; data.len() / channels];
                model.process(&mut mix_buffer);
                for frame in data.chunks_mut(channels) {
                    let sample = mix_buffer.remove(0);
                    for sample_out in frame.iter_mut() {
                        *sample_out = sample;
                    }
                }
            },
            err_fn,
            None,
        )?,
        _ => return Err(anyhow::anyhow!("Unsupported sample format")),
    };

    stream.play()?;
    Ok(stream)
}
