use macroquad::prelude::*;
use phonology::{Phoneme, Features};
use physics::{World};
use brain::{Brain, Muscle};
use neuron::Command;

mod phonology;
mod physics;
mod brain;
mod neuron;

fn create_phoneme(c: char) -> Phoneme {
    let features = match c {
        'a' => Features { voice: 1.0, place: 0.5, manner: 1.0 },
        'e' => Features { voice: 1.0, place: 0.3, manner: 0.8 },
        'i' => Features { voice: 1.0, place: 0.2, manner: 0.9 },
        'o' => Features { voice: 1.0, place: 0.7, manner: 0.8 },
        'u' => Features { voice: 1.0, place: 0.8, manner: 0.9 },
        'p' => Features { voice: 0.0, place: 0.0, manner: 0.0 },
        't' => Features { voice: 0.0, place: 0.2, manner: 0.0 },
        'k' => Features { voice: 0.0, place: 0.8, manner: 0.0 },
        'b' => Features { voice: 1.0, place: 0.0, manner: 0.0 },
        'd' => Features { voice: 1.0, place: 0.2, manner: 0.0 },
        'g' => Features { voice: 1.0, place: 0.8, manner: 0.0 },
        'm' => Features { voice: 1.0, place: 0.0, manner: 0.2 },
        'n' => Features { voice: 1.0, place: 0.2, manner: 0.2 },
        'l' => Features { voice: 1.0, place: 0.3, manner: 0.6 },
        'r' => Features { voice: 1.0, place: 0.3, manner: 0.5 },
        _ => Features { voice: 0.5, place: 0.5, manner: 0.5 },
    };
    Phoneme { features, symbol: c }
}

#[macroquad::main("Neuro Speech")]
async fn main() {
    let mut world = World::new();
    let mut brain = Brain::new();

    // Initial Word: "pater"
    let word_str = "pater";
    let start_x = 300.0;
    let start_y = 300.0;
    let mut last_idx = None;

    // Create Phonemes and Neurons for each
    for (i, c) in word_str.chars().enumerate() {
        let p = create_phoneme(c);
        let pos = Vec2::new(start_x + i as f32 * 60.0, start_y);
        world.add_particle(p, pos);
        let particle_idx = world.particles.len() - 1;

        if let Some(prev) = last_idx {
            world.add_spring(prev, particle_idx, 60.0);
        }
        last_idx = Some(particle_idx);

        // Add a "Levitator" neuron (pulls up)
        let n_up = brain.add_neuron();
        brain.add_muscle(n_up, particle_idx, Vec2::new(0.0, -5.0));

        // Add a "Gravitator" neuron (pulls down)
        let n_down = brain.add_neuron();
        brain.add_muscle(n_down, particle_idx, Vec2::new(0.0, 5.0));
    }

    loop {
        clear_background(BLACK);

        // User Input to stimulate neurons
        // 1-5 stimulates the first 5 "Up" neurons
        // Q-T stimulates the first 5 "Down" neurons

        let keys_up = [KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4, KeyCode::Key5];
        let keys_down = [KeyCode::Q, KeyCode::W, KeyCode::E, KeyCode::R, KeyCode::T];

        for (i, &key) in keys_up.iter().enumerate() {
            let n_idx = i * 2; // Up neurons are at 0, 2, 4...
            if n_idx < brain.neurons.len() {
                let current = if is_key_down(key) { 20.0 } else { 0.0 };
                brain.neurons[n_idx].apply_command(Command::SetCurrent(current));
            }
        }

        for (i, &key) in keys_down.iter().enumerate() {
            let n_idx = i * 2 + 1; // Down neurons are at 1, 3, 5...
            if n_idx < brain.neurons.len() {
                let current = if is_key_down(key) { 20.0 } else { 0.0 };
                brain.neurons[n_idx].apply_command(Command::SetCurrent(current));
            }
        }

        if is_key_down(KeyCode::Space) {
             // Stimulate ALL neurons randomly (Babbling)
             for n in &mut brain.neurons {
                 if rand::gen_range(0, 100) < 5 {
                     n.apply_command(Command::Pluck);
                 }
             }
        }


        // Physics Loop
        let dt = get_frame_time();

        world.reset_forces();
        world.apply_internal_forces();
        brain.update(dt, &mut world);
        world.integrate(dt);

        // Render Connections (Springs)
        for &(i, j, _) in &world.springs {
             if i < world.particles.len() && j < world.particles.len() {
                draw_line(
                    world.particles[i].pos.x, world.particles[i].pos.y,
                    world.particles[j].pos.x, world.particles[j].pos.y,
                    2.0, GRAY
                );
             }
        }

        // Render Muscles (Axons)
        for muscle in &brain.muscles {
            if muscle.neuron_idx < brain.neurons.len() && muscle.particle_idx < world.particles.len() {
                 let neuron = &brain.neurons[muscle.neuron_idx];
                 let particle = &world.particles[muscle.particle_idx];

                 // Visualize Axon
                 // Neuron positions are fixed at top/bottom for visualization
                 let n_pos = if muscle.force_vector.y < 0.0 {
                     // Up neuron
                     Vec2::new(particle.pos.x, 50.0)
                 } else {
                     // Down neuron
                     Vec2::new(particle.pos.x, screen_height() - 50.0)
                 };

                 let activation = ((neuron.v + 50.0) / 20.0).clamp(0.0, 1.0);
                 let color = Color::new(1.0, 1.0, 0.0, activation * 0.8 + 0.2); // Yellow, brightness = activation

                 draw_line(n_pos.x, n_pos.y, particle.pos.x, particle.pos.y, 1.0, color);
                 draw_circle(n_pos.x, n_pos.y, 10.0, color);
            }
        }

        // Render Particles (Phonemes)
        for p in &world.particles {
            let color = Color::new(
                p.phoneme.features.place,
                p.phoneme.features.manner,
                p.phoneme.features.voice,
                1.0
            );
            draw_circle(p.pos.x, p.pos.y, 20.0, color);
            draw_text(&p.phoneme.symbol.to_string(), p.pos.x - 8.0, p.pos.y + 8.0, 30.0, WHITE);
        }

        draw_text("Neuro-Speech: [1-5] Pull Up | [Q-T] Pull Down | [SPACE] Babble", 10.0, 30.0, 20.0, WHITE);

        next_frame().await
    }
}
