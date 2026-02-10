mod bee;
mod neuron;

use ::rand::Rng;
use bee::{Bee, BeeState, StimulusSource};
use macroquad::prelude::*;

#[macroquad::main("Synaptic Waggle")]
async fn main() {
    let mut bees = Vec::new();
    let num_bees = 300;
    let mut rng = ::rand::thread_rng();

    // Initial spawning
    for _ in 0..num_bees {
        let x = rng.gen_range(0.0..screen_width());
        let y = rng.gen_range(0.0..screen_height());
        bees.push(Bee::new(vec2(x, y), &mut rng));
    }

    let mut sources = vec![StimulusSource {
        position: vec2(screen_width() / 2.0, screen_height() / 2.0),
        strength: 15.0,
        radius: 100.0,
    }];

    let mut tick: u64 = 0;

    loop {
        // Input Handling
        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            if let Some(source) = sources.first_mut() {
                source.position = vec2(x, y);
            }
        }

        if is_key_pressed(KeyCode::Space) {
            let (x, y) = mouse_position();
            sources.push(StimulusSource {
                position: vec2(x, y),
                strength: 15.0,
                radius: 100.0,
            });
        }

        if is_key_pressed(KeyCode::C) {
            sources.clear();
        }

        if is_key_pressed(KeyCode::R) {
            bees.clear();
            for _ in 0..num_bees {
                let x = rng.gen_range(0.0..screen_width());
                let y = rng.gen_range(0.0..screen_height());
                bees.push(Bee::new(vec2(x, y), &mut rng));
            }
        }

        // Simulation Update
        let bounds = vec2(screen_width(), screen_height());

        // Interaction Logic (O(N^2) but optimized by filtering)
        // We need to count how many *dancing* bees are near each bee.
        let interaction_radius = 60.0;
        let mut nearby_counts = vec![0; bees.len()];

        // Only iterate over dancing bees as "senders"
        let dancing_indices: Vec<usize> = bees
            .iter()
            .enumerate()
            .filter(|(_, b)| b.state == BeeState::Dancing)
            .map(|(i, _)| i)
            .collect();

        for &dancer_idx in &dancing_indices {
            let dancer_pos = bees[dancer_idx].position;
            for (receiver_idx, receiver) in bees.iter().enumerate() {
                if dancer_idx != receiver_idx {
                    if receiver.position.distance_squared(dancer_pos)
                        < interaction_radius * interaction_radius
                    {
                        nearby_counts[receiver_idx] += 1;
                    }
                }
            }
        }

        for (i, bee) in bees.iter_mut().enumerate() {
            // Pass the count of active synapses (dances) targeting this bee
            bee.update(&sources, nearby_counts[i], bounds, tick);
        }

        tick += 1;

        // Drawing
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0)); // Dark Blue/Black background

        // Draw Sources (Stimulus)
        for source in &sources {
            // Pulsing effect
            let pulse = (get_time() as f32 * 2.0).sin() * 5.0;
            draw_circle(
                source.position.x,
                source.position.y,
                source.radius + pulse,
                Color::new(0.0, 1.0, 0.0, 0.1),
            );
            draw_circle_lines(
                source.position.x,
                source.position.y,
                source.radius + pulse,
                2.0,
                GREEN,
            );
            draw_text(
                "Stimulus",
                source.position.x - 30.0,
                source.position.y,
                20.0,
                GREEN,
            );
        }

        // Draw Connections (Synapses)
        // Draw lines from dancers to receivers within range? Too messy.
        // Draw lines from dancers to nearby bees?
        for &dancer_idx in &dancing_indices {
            let dancer_pos = bees[dancer_idx].position;
            // Draw broadcast ring
            draw_circle_lines(
                dancer_pos.x,
                dancer_pos.y,
                interaction_radius,
                1.0,
                Color::new(1.0, 0.8, 0.0, 0.1),
            );
        }

        // Draw Bees
        for bee in &bees {
            let (color, radius) = match bee.state {
                BeeState::Dancing => (GOLD, 5.0),
                BeeState::Refractory => (Color::new(0.8, 0.2, 0.2, 0.8), 3.0),
                BeeState::Wandering => {
                    // Voltage visualization: Blue (-65) to Cyan (-40) to White (-30)
                    // v ranges from -80 to 30 usually.
                    let v = bee.neuron.v;
                    let t = ((v + 80.0) / 110.0).clamp(0.0, 1.0);
                    (Color::new(t * 0.5, t * 0.8, 1.0, 0.8), 3.0)
                }
            };

            draw_circle(bee.position.x, bee.position.y, radius, color);

            if bee.state == BeeState::Dancing {
                // Draw Waggle Vector
                let dance_vec = vec2(bee.dance_angle.cos(), bee.dance_angle.sin()) * 15.0;
                draw_line(
                    bee.position.x,
                    bee.position.y,
                    bee.position.x + dance_vec.x,
                    bee.position.y + dance_vec.y,
                    2.0,
                    ORANGE,
                );
            }
        }

        // UI Overlay
        draw_text("Synaptic Waggle", 20.0, 30.0, 40.0, WHITE);
        draw_text(
            "Beurons (Bee Neurons) form a mobile neural network.",
            20.0,
            60.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            &format!("Active Spikes (Dances): {}", dancing_indices.len()),
            20.0,
            90.0,
            20.0,
            GOLD,
        );
        draw_text(
            "Left Click: Move Stimulus | Space: Add Stimulus | C: Clear | R: Reset",
            20.0,
            screen_height() - 20.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
