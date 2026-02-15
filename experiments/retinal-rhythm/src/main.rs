mod neuron;
mod retina;
mod audio;

use macroquad::prelude::*;
use retina::Retina;
use audio::AudioEngine;

struct Ball {
    pos: Vec2,
    vel: Vec2,
    radius: f32,
    color: Color,
}

impl Ball {
    fn new(x: f32, y: f32) -> Self {
        let mut rng = ::rand::thread_rng();
        use ::rand::Rng;
        Self {
            pos: vec2(x, y),
            vel: vec2(rng.gen_range(-2.0..2.0), rng.gen_range(-2.0..2.0)),
            radius: rng.gen_range(2.0..8.0),
            color: Color::from_rgba(rng.gen(), rng.gen(), rng.gen(), 255),
        }
    }

    fn update(&mut self, dt: f32, bounds: Vec2) {
        self.pos += self.vel * dt * 60.0;
        if self.pos.x < self.radius || self.pos.x > bounds.x - self.radius {
            self.vel.x *= -1.0;
            self.pos.x = self.pos.x.clamp(self.radius, bounds.x - self.radius);
        }
        if self.pos.y < self.radius || self.pos.y > bounds.y - self.radius {
            self.vel.y *= -1.0;
            self.pos.y = self.pos.y.clamp(self.radius, bounds.y - self.radius);
        }
    }
}

#[macroquad::main("Retinal Rhythm")]
async fn main() {
    let mut retina = Retina::new(64, 64);
    let audio = AudioEngine::new().await;

    let mut balls: Vec<Ball> = (0..20)
        .map(|_| Ball::new(32.0, 32.0))
        .collect();

    let render_target = render_target(retina.width as u32, retina.height as u32);
    render_target.texture.set_filter(FilterMode::Nearest);

    // Visualization textures
    let retina_texture = Texture2D::from_image(&Image::gen_image_color(
        retina.width as u16,
        retina.height as u16,
        BLACK
    ));
    retina_texture.set_filter(FilterMode::Nearest);

    let mut retina_image = Image::gen_image_color(retina.width as u16, retina.height as u16, BLACK);

    loop {
        let dt = get_frame_time();

        // 1. Update World
        // Render World to Texture
        let camera = Camera2D {
            zoom: vec2(2.0 / retina.width as f32, 2.0 / retina.height as f32), // Map 0..width to -1..1
            target: vec2(retina.width as f32 / 2.0, retina.height as f32 / 2.0),
            render_target: Some(render_target.clone()),
            ..Default::default()
        };

        set_camera(&camera);
        clear_background(BLACK);

        for ball in &mut balls {
            ball.update(dt, vec2(retina.width as f32, retina.height as f32));
            draw_circle(ball.pos.x, ball.pos.y, ball.radius, ball.color);
        }

        set_default_camera();

        // 2. Read Texture -> Input Buffer
        let texture_data = render_target.texture.get_texture_data();
        let mut input_buffer = vec![0.0; retina.width * retina.height];

        // texture_data.bytes is RGBA
        for (i, pixel) in texture_data.bytes.chunks(4).enumerate() {
            if i < input_buffer.len() {
                let r = pixel[0] as f32 / 255.0;
                let g = pixel[1] as f32 / 255.0;
                let b = pixel[2] as f32 / 255.0;
                input_buffer[i] = (r + g + b) / 3.0;
            }
        }

        // 3. Update Retina
        let sub_steps = 4;
        let sub_dt = (dt * 1000.0) / sub_steps as f32; // dt in ms

        let mut spikes = Vec::new();

        for _ in 0..sub_steps {
             let new_spikes = retina.update(&input_buffer, sub_dt);
             spikes.extend(new_spikes);
        }

        // 4. Audio Trigger
        // Deduplicate X coords to avoid phasing issues?
        // Actually, phase issues are cool (Moonshot).

        let mut played_count = 0;
        for (x, _y) in &spikes {
             if played_count > 10 { break; } // Limit polyphony

             let freq = 100.0 + (*x as f32 / retina.width as f32) * 800.0;
             audio.play_closest(freq);
             played_count += 1;
        }

        // 5. Visualization
        clear_background(DARKGRAY);

        let sw = screen_width();
        let sh = screen_height();

        let scale = (sh * 0.8) / retina.height as f32;
        let offset_x = (sw - (retina.width as f32 * scale)) / 2.0;
        let offset_y = (sh - (retina.height as f32 * scale)) / 2.0;

        // Fade out old spikes
        for pixel in retina_image.bytes.chunks_mut(4) {
             if pixel[3] > 10 {
                 pixel[3] -= 10;
             } else {
                 pixel[3] = 0;
             }
        }

        // Draw new spikes
        for (x, y) in &spikes {
             retina_image.set_pixel(*x as u32, *y as u32, YELLOW);
        }

        retina_texture.update(&retina_image);

        // Draw Input (World)
        draw_texture_ex(&render_target.texture, offset_x, offset_y, WHITE, DrawTextureParams {
            dest_size: Some(vec2(retina.width as f32 * scale, retina.height as f32 * scale)),
            flip_y: true, // RenderTargets are flipped
            ..Default::default()
        });

        // Draw Spikes Overlay
        draw_texture_ex(&retina_texture, offset_x, offset_y, WHITE, DrawTextureParams {
            dest_size: Some(vec2(retina.width as f32 * scale, retina.height as f32 * scale)),
            ..Default::default()
        });

        draw_text("Retinal Rhythm", 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Spikes: {}", spikes.len()), 20.0, 60.0, 20.0, GREEN);
        draw_text("Visual Input -> DoG Filter -> Izhikevich Neurons -> Audio", 20.0, sh - 20.0, 20.0, LIGHTGRAY);

        next_frame().await;
    }
}
