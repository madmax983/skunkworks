use macroquad::prelude::*;
use ::rand::Rng; // Explicit import

mod physics;
mod neuron;
mod audio;

use physics::PendulumSystem;
use neuron::HodgkinHuxley;
use audio::init_audio;

const NODE_COUNT: usize = 10;

#[macroquad::main("Neuro-Pendulum")]
async fn main() -> anyhow::Result<()> {
    let mut audio_sys = init_audio()?;

    // Initialize Physics
    let mut sys = PendulumSystem::new();
    let center = Vec2::new(screen_width() / 2.0, 100.0);

    // Create a chain
    let root = sys.add_node(center, 10.0, true);
    let mut prev_node = root;

    for i in 0..NODE_COUNT {
        let pos = center + Vec2::new(0.0, (i + 1) as f32 * 30.0);
        let node = sys.add_node(pos, 1.0, false);
        sys.add_link(prev_node, node, 30.0);
        prev_node = node;
    }

    // Initialize Neurons (One per moving node)
    // Note: Node 0 is fixed root, so we skip it or give it a dummy neuron?
    // Let's verify indexes. Root is 0. Moving nodes are 1..=NODE_COUNT.
    // Let's create neurons for indices 1..=NODE_COUNT.
    let mut neurons: Vec<HodgkinHuxley> = (0..NODE_COUNT).map(|_| HodgkinHuxley::new()).collect();

    let mut physics_accumulator = 0.0;
    let physics_dt = 1.0 / 60.0;

    // Audio rate in ms for HH
    let audio_dt_ms = 1000.0 / audio_sys.sample_rate;

    let mut zoom = 1.0;
    let mut offset = Vec2::ZERO;

    loop {
        // Controls
        if is_key_down(KeyCode::Up) { offset.y += 5.0; }
        if is_key_down(KeyCode::Down) { offset.y -= 5.0; }
        if is_key_down(KeyCode::Left) { offset.x += 5.0; }
        if is_key_down(KeyCode::Right) { offset.x -= 5.0; }
        if is_key_down(KeyCode::Equal) { zoom *= 1.05; }
        if is_key_down(KeyCode::Minus) { zoom *= 0.95; }

        if is_mouse_button_down(MouseButton::Left) {
             let mouse = mouse_position();
             let world_mouse = (Vec2::new(mouse.0, mouse.1) - Vec2::new(screen_width()/2.0, screen_height()/2.0)) / zoom + center + offset;

             // Drag last node
             let last = sys.nodes.len() - 1;
             sys.nodes[last].pos = world_mouse;
             sys.nodes[last].prev_pos = world_mouse;
             sys.nodes[last].force_accumulator = Vec2::ZERO; // Cancel forces while dragging
        }

        let frame_time = get_frame_time();
        physics_accumulator += frame_time;

        // 1. Physics Steps
        while physics_accumulator >= physics_dt {
            sys.step(physics_dt);
            physics_accumulator -= physics_dt;
        }

        // 2. Audio/Neuron Steps
        // Generate enough samples to cover the frame time
        // Safety cap to prevent death spiral if frame time is huge
        let samples_needed = ((frame_time * audio_sys.sample_rate) as usize).min(2000);

        for _ in 0..samples_needed {
            let mut total_v = 0.0;

            for (i, neuron) in neurons.iter_mut().enumerate() {
                let node_idx = i + 1; // Skip root
                let node = &mut sys.nodes[node_idx];

                // Input: Kinetic Energy
                // Velocity approximation
                let vel_vec = node.pos - node.prev_pos;
                let speed = vel_vec.length();

                // Map speed (approx 0.0 - 10.0 per frame?) to current (0 - 100)
                // Need to tune this gain
                neuron.i_inj = speed * 1000.0;

                neuron.step(audio_dt_ms);

                total_v += neuron.v;

                // Feedback: Spike Force
                if neuron.v > 0.0 {
                    // Kick in direction of velocity (resonance) or tangent?
                    // Let's try kicking perpendicular to velocity for swirl, or just random jitter?
                    // Or simply "Expansion" force?

                    // Let's add a "twitch" force
                    let mut rng = ::rand::thread_rng();
                    let twitch = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));

                    // Strength proportional to voltage
                    let strength = neuron.v * 0.5;
                    node.force_accumulator += twitch * strength;
                }
            }

            // Mix and Push
            let sample = (total_v / neurons.len() as f32) * 0.05; // Gain down
            let _ = audio_sys.buffer_tx.push(sample);
        }

        // 3. Render
        clear_background(BLACK);

        // Camera
        let cam_center = center + offset;
        set_camera(&Camera2D {
            target: cam_center,
            zoom: Vec2::new(zoom / screen_width() * 2.0, -zoom / screen_height() * 2.0),
            ..Default::default()
        });

        // Draw Links
        for link in &sys.links {
            let a = sys.nodes[link.a].pos;
            let b = sys.nodes[link.b].pos;
            draw_line(a.x, a.y, b.x, b.y, 2.0, GRAY);
        }

        // Draw Nodes
        for (i, node) in sys.nodes.iter().enumerate() {
            let color = if node.fixed { RED } else {
                // Color based on neuron voltage if available
                if i > 0 {
                    let v = neurons[i-1].v; // -65 to +50
                    let t = (v + 70.0) / 120.0; // Norm 0-1
                    Color::new(t, 0.2, 1.0 - t, 1.0)
                } else {
                    WHITE
                }
            };
            draw_circle(node.pos.x, node.pos.y, 5.0, color);
        }

        set_default_camera();
        draw_text("NEURO-PENDULUM", 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Neurons: {}", neurons.len()), 20.0, 60.0, 20.0, GRAY);
        draw_text("Drag bottom node to excite.", 20.0, screen_height() - 20.0, 20.0, GRAY);

        next_frame().await
    }
}
