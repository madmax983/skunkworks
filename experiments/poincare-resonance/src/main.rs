use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_channel::bounded;
use macroquad::prelude::*;
use poincare_disk::{hyperbolic_dist, Point};
use resonance_audio::{AudioCommand, AudioModel};

const GRID_W: usize = 120;
const GRID_H: usize = 120;

fn window_conf() -> macroquad::window::Conf {
    macroquad::window::Conf {
        window_title: "Poincaré Resonance".to_owned(),
        ..Default::default()
    }
}

fn main() {
    if std::env::args().any(|arg| arg == "--headless") {
        return;
    }
    macroquad::Window::from_config(window_conf(), async_main());
}

async fn async_main() {
    let (cmd_tx, cmd_rx) = bounded(1024);
    let (snap_tx, snap_rx) = bounded(2);

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device available");
    let config = device.default_output_config().unwrap();

    let mut model = AudioModel::new(GRID_W, GRID_H, cmd_rx, snap_tx, None);

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                model.process(data);
            },
            |err| eprintln!("Audio stream error: {}", err),
            None,
        ),
        _ => panic!("Unsupported sample format"),
    }
    .unwrap();

    stream.play().unwrap();

    let mut image = Image::gen_image_color(GRID_W as u16, GRID_H as u16, BLANK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut time = 0.0_f32;

    loop {
        clear_background(Color::new(0.05, 0.05, 0.08, 1.0));
        time += get_frame_time();

        let cx = (time * 0.5).cos() * 0.5;
        let cy = (time * 0.3).sin() * 0.5;
        let center_pt = Point::new(cx as f64, cy as f64);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let scale = (screen_height() / GRID_H as f32).min(screen_width() / GRID_W as f32);
            let w = GRID_W as f32 * scale;
            let h = GRID_H as f32 * scale;
            let offset_x = (screen_width() - w) / 2.0;
            let offset_y = (screen_height() - h) / 2.0;

            let grid_x = ((mx - offset_x) / scale).round() as usize;
            let grid_y = ((my - offset_y) / scale).round() as usize;

            if grid_x < GRID_W && grid_y < GRID_H {
                // Determine hyperbolic properties based on click position relative to center_pt
                let normalized_x = (grid_x as f64 / GRID_W as f64) * 2.0 - 1.0;
                let normalized_y = (grid_y as f64 / GRID_H as f64) * 2.0 - 1.0;
                let click_pt = Point::new(normalized_x, normalized_y);

                // Only pluck if within the unit disk (r < 1.0)
                if normalized_x * normalized_x + normalized_y * normalized_y < 1.0 {
                    let h_dist = hyperbolic_dist(click_pt, center_pt);
                    let strength = (1.0 / (1.0 + h_dist as f32)).clamp(0.1, 1.0);

                    cmd_tx
                        .send(AudioCommand::Pluck {
                            x: grid_x,
                            y: grid_y,
                            strength,
                        })
                        .unwrap();
                }
            }
        }

        if let Ok(snapshot) = snap_rx.try_recv() {
            let pixels = image.get_image_data_mut();
            for y in 0..GRID_H {
                for x in 0..GRID_W {
                    let normalized_x = (x as f64 / GRID_W as f64) * 2.0 - 1.0;
                    let normalized_y = (y as f64 / GRID_H as f64) * 2.0 - 1.0;
                    let pt = Point::new(normalized_x, normalized_y);

                    let r2 = normalized_x * normalized_x + normalized_y * normalized_y;
                    let p_idx = y * GRID_W + x;

                    if r2 >= 1.0 {
                        pixels[p_idx] = [0, 0, 0, 0]; // Outside disk
                    } else {
                        let h_dist = hyperbolic_dist(pt, center_pt);

                        let pressure = snapshot.pressure[p_idx];

                        // Warp the pressure visualization by hyperbolic geometry
                        let warped_pressure = pressure * (1.0 + (h_dist as f32 * 0.5));
                        let c = ((warped_pressure + 1.0) * 0.5 * 255.0).clamp(0.0, 255.0) as u8;

                        // Colorize based on hyperbolic depth
                        let depth_color = (h_dist * 20.0).min(255.0) as u8;

                        pixels[p_idx] = [c, c.saturating_sub(depth_color), 255 - depth_color, 255];
                    }
                }
            }
            texture.update(&image);
        }

        let scale = (screen_height() / GRID_H as f32).min(screen_width() / GRID_W as f32);
        let w = GRID_W as f32 * scale;
        let h = GRID_H as f32 * scale;
        let x = (screen_width() - w) / 2.0;
        let y = (screen_height() - h) / 2.0;

        draw_texture_ex(
            &texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(w, h)),
                ..Default::default()
            },
        );

        let screen_cx = x + (cx as f32 + 1.0) * 0.5 * w;
        let screen_cy = y + (cy as f32 + 1.0) * 0.5 * h;
        draw_circle_lines(screen_cx, screen_cy, 5.0, 2.0, RED);
        draw_circle_lines(x + w / 2.0, y + h / 2.0, w / 2.0, 2.0, GRAY); // Unit disk boundary

        draw_text("Poincaré Resonance", 10.0, 20.0, 30.0, WHITE);
        draw_text(
            "Hyperbolic acoustic wave propagation",
            10.0,
            50.0,
            20.0,
            GRAY,
        );
        draw_text(
            "Click to pluck. Distances warp near the edge.",
            10.0,
            70.0,
            20.0,
            GRAY,
        );

        next_frame().await;
    }
}
