use crossbeam_channel::bounded;
use gray_scott::GrayScott;
use macroquad::prelude::*;
use resonance_audio::audio::{AudioCommand, AudioModel};

const GRID_SIZE: usize = 100;
const CELL_SIZE: f32 = 6.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Gray-Resonance".to_owned(),
        window_width: (GRID_SIZE as f32 * CELL_SIZE) as i32,
        window_height: (GRID_SIZE as f32 * CELL_SIZE) as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> anyhow::Result<()> {
    let (cmd_tx, cmd_rx) = bounded(1024);
    let (snap_tx, _snap_rx) = bounded(2);

    let mut audio_model = AudioModel::new(GRID_SIZE, GRID_SIZE, cmd_rx, snap_tx, None);

    let mut gs = GrayScott::new(GRID_SIZE, GRID_SIZE);

    // Initial seed in center
    for y in GRID_SIZE / 2 - 5..GRID_SIZE / 2 + 5 {
        for x in GRID_SIZE / 2 - 5..GRID_SIZE / 2 + 5 {
            gs.add_chemical(x, y, 1.0);
        }
    }

    // Spots parameters
    let feed = 0.03;
    let kill = 0.062;

    // A simple buffer to drive audio locally if not using CPAL thread
    let mut audio_buf = vec![0.0; 256];

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let x = (mx / CELL_SIZE) as usize;
            let y = (my / CELL_SIZE) as usize;
            if x < GRID_SIZE && y < GRID_SIZE {
                gs.add_chemical(x, y, 1.0);
            }
        }

        // Update Gray-Scott
        for _ in 0..10 {
            gs.update(feed, kill, 1.0);
        }

        // Inject acoustic waves based on the V chemical
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let idx = gs.get_index(x, y);
                let v = gs.v()[idx];
                if v > 0.5 {
                    // Inject a pluck
                    let _ = cmd_tx.try_send(AudioCommand::Pluck {
                        x,
                        y,
                        strength: v * 0.1,
                    });
                }
            }
        }

        // Process audio locally to keep it running
        audio_model.process(&mut audio_buf);

        clear_background(BLACK);

        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let idx = gs.get_index(x, y);
                let u = gs.u()[idx];
                let v = gs.v()[idx];

                // Render chemical state
                let c = Color::new(v, 0.0, u, 1.0);
                draw_rectangle(
                    x as f32 * CELL_SIZE,
                    y as f32 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    c,
                );
            }
        }

        next_frame().await;
    }

    Ok(())
}
