use crossbeam_channel::bounded;
use macroquad::prelude::*;
use neuro_sim::Network;
use resonance_audio::audio::{AudioCommand, AudioModel};

const GRID_WIDTH: usize = 128;
const GRID_HEIGHT: usize = 128;

#[macroquad::main("Neuro Tank")]
async fn main() {
    // Communication channels for the AudioModel
    let (cmd_tx, cmd_rx) = bounded(1024);
    let (snap_tx, snap_rx) = bounded(2);

    // Initialize the wave physics model
    let mut model = AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx, None);

    // Initialize the SNN
    let mut brain = Network::new();

    // Create a 2D grid of neurons
    let spacing = 16;
    let cols = GRID_WIDTH / spacing;
    let rows = GRID_HEIGHT / spacing;

    struct NeuronPos {
        id: usize,
        x: usize,
        y: usize,
    }

    let mut neurons_pos = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            let id = brain.add_neuron();
            let x = (col * spacing) + (spacing / 2);
            let y = (row * spacing) + (spacing / 2);
            neurons_pos.push(NeuronPos { id, x, y });

            // Randomly connect some neurons to induce network dynamics
            if row > 0 {
                let up_id = id - cols;
                brain.add_synapse(id, up_id, 2.0);
            }
            if col > 0 {
                let left_id = id - 1;
                brain.add_synapse(id, left_id, 2.0);
            }
        }
    }

    // Visual setup
    let mut image = Image::gen_image_color(GRID_WIDTH as u16, GRID_HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut current_snapshot: Vec<f32> = vec![0.0; GRID_WIDTH * GRID_HEIGHT];

    loop {
        let _dt = get_frame_time();

        // 1. Process wave tank
        // We simulate a few audio steps per visual frame
        let steps = 100;
        let mut dummy_buffer = vec![0.0; steps];
        model.process(&mut dummy_buffer);

        while let Ok(snap) = snap_rx.try_recv() {
            current_snapshot = snap.pressure;
        }

        // 2. Process Brain
        // Calculate input currents from wave tank pressure
        let mut inputs = vec![0.0; brain.neurons.len()];
        for np in &neurons_pos {
            let idx = np.y * GRID_WIDTH + np.x;
            if idx < current_snapshot.len() {
                // Wave pressure translates to input current
                let pressure = current_snapshot[idx];
                inputs[np.id] = pressure * 100.0;
            }
        }

        brain.step(&inputs);

        // 3. Inject spikes back into wave tank
        for np in &neurons_pos {
            if brain.is_spiking(np.id) {
                let _ = cmd_tx.send(AudioCommand::Pluck {
                    x: np.x,
                    y: np.y,
                    strength: 1.0,
                });
            }
        }

        // --- Rendering ---
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

        // Draw neurons
        for np in &neurons_pos {
            let lx = offset_x + np.x as f32 * scale;
            let ly = offset_y + np.y as f32 * scale;
            let color = if brain.is_spiking(np.id) {
                GREEN
            } else {
                DARKGRAY
            };
            draw_circle(lx + scale / 2.0, ly + scale / 2.0, scale, color);
        }

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
                let _ = cmd_tx.send(AudioCommand::Pluck {
                    x: grid_x,
                    y: grid_y,
                    strength: 0.5,
                });
            }
        }

        draw_text("L-Click: Stimulate Waves", 10.0, 20.0, 20.0, WHITE);

        next_frame().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        assert_eq!(GRID_WIDTH, 128);
    }
}
