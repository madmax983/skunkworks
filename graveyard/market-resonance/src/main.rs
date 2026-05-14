use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_channel::bounded;
use macroquad::prelude::*;
use market_sim::{Grid as MarketGrid, Particle};
use resonance_audio::audio::{AudioCommand, AudioModel};

const GRID_W: usize = 120;
const GRID_H: usize = 120;

#[macroquad::main("Market Resonance")]
async fn main() {
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

    let mut market = MarketGrid::new(GRID_W, GRID_H);

    loop {
        clear_background(Color::new(0.05, 0.05, 0.08, 1.0));

        // Inject orders
        if macroquad::rand::gen_range(0.0, 1.0) < 0.3 {
            let x = macroquad::rand::gen_range(0, GRID_W);
            market.set(x, 0, Particle::Ask(macroquad::rand::gen_range(0, 1000000)));
        }
        if macroquad::rand::gen_range(0.0, 1.0) < 0.3 {
            let x = macroquad::rand::gen_range(0, GRID_W);
            market.set(
                x,
                GRID_H - 1,
                Particle::Bid(macroquad::rand::gen_range(0, 1000000)),
            );
        }

        // Add some random noise trades to keep it lively if we want, or just wait for collisions.

        let trades = market.update();
        for trade in trades {
            // A trade happens at a given price (Y axis) and some X axis (which we don't directly know from TradeEvent,
            // but we can just pluck randomly or along the price line).
            // Actually, we can approximate the position. TradeEvent gives price. Price = height - 1 - Y.
            // So Y = height - 1 - price.
            let y = (GRID_H as f32 - 1.0 - trade.price) as usize;
            let x = macroquad::rand::gen_range(0, GRID_W);

            // The strength could be proportional to price or just fixed.
            let strength = (trade.price / GRID_H as f32).clamp(0.1, 1.0);

            cmd_tx.send(AudioCommand::Pluck { x, y, strength }).unwrap();
        }

        if let Ok(snapshot) = snap_rx.try_recv() {
            let pixels = image.get_image_data_mut();
            for y in 0..GRID_H {
                for x in 0..GRID_W {
                    let p_idx = y * GRID_W + x;
                    let pressure = snapshot.pressure[p_idx];

                    // Base color from acoustic pressure
                    let c = ((pressure + 1.0) * 0.5 * 255.0).clamp(0.0, 255.0) as u8;
                    let mut col = [c, c, 255, 255]; // blueish wave

                    // Overlay market particles
                    let particle = market.get(x, y);
                    match particle {
                        Particle::Bid(_) => col = [0, 255, 0, 255], // Green for Bids
                        Particle::Ask(_) => col = [255, 0, 0, 255], // Red for Asks
                        Particle::Trade { .. } => col = [255, 255, 0, 255], // Yellow for Trades
                        Particle::Wall => col = [100, 100, 100, 255],
                        Particle::Empty => {}
                    }

                    pixels[p_idx] = col;
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

        draw_text("Market Resonance", 10.0, 20.0, 30.0, WHITE);
        draw_text("Acoustic Market Sonification", 10.0, 50.0, 20.0, GRAY);
        draw_text("Bids (Green) vs Asks (Red)", 10.0, 70.0, 20.0, GRAY);
        draw_text(
            format!("Trades: {}", market.trade_count).as_str(),
            10.0,
            90.0,
            20.0,
            YELLOW,
        );

        next_frame().await;
    }
}
