use macroquad::prelude::*;
use neuro_sim::Network;
use poincare_disk::{hyperbolic_dist, Point};
use ::rand::{thread_rng, Rng};

const NUM_NEURONS: usize = 100;
const DISK_RADIUS: f32 = 300.0; // Visual radius

#[macroquad::main("Hyperbolic Neural Networks")]
async fn main() {
    let mut rng = thread_rng();
    let mut network = Network::new();
    let mut positions = Vec::with_capacity(NUM_NEURONS);

    // Initialize neurons with random positions within the unit disk
    for _ in 0..NUM_NEURONS {
        network.add_neuron();

        let r = rng.gen_range(0.0..0.9_f64).sqrt(); // sqrt for uniform distribution
        let theta = rng.gen_range(0.0..std::f64::consts::TAU);

        positions.push(Point::new(r * theta.cos(), r * theta.sin()));
    }

    // Connect neurons based on hyperbolic distance
    for i in 0..NUM_NEURONS {
        for j in 0..NUM_NEURONS {
            if i != j {
                let dist = hyperbolic_dist(positions[i], positions[j]);

                // Only connect neurons within a certain hyperbolic radius
                if dist < 2.0 {
                    // Weight is inversely proportional to distance, with some randomness
                    let weight = (2.0 - dist as f32) * rng.gen_range(5.0..15.0);

                    // Delay is proportional to hyperbolic distance!
                    // This is the novel trait: signals traveling near the edge take exponentially longer
                    let delay = (dist * 10.0).round() as usize;

                    // 10% chance to be inhibitory
                    let final_weight = if rng.gen_bool(0.1) { -weight * 1.5 } else { weight };

                    network.add_synapse_with_delay(i, j, final_weight, delay);
                }
            }
        }
    }

    // Buffer for external inputs to the network
    let mut inputs = vec![0.0; NUM_NEURONS];

    loop {
        clear_background(Color::new(0.05, 0.05, 0.08, 1.0));

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Draw the boundary of the Poincaré disk
        draw_circle_lines(center_x, center_y, DISK_RADIUS, 2.0, GRAY);

        inputs.fill(0.0);

        // Inject current into a random neuron occasionally to keep the network active
        if rng.gen_bool(0.05) {
            let target = rng.gen_range(0..NUM_NEURONS);
            inputs[target] = 50.0; // Strong stimulus
        }

        // Allow user to stimulate by clicking near a neuron
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            for i in 0..NUM_NEURONS {
                let nx = center_x + (positions[i].re as f32 * DISK_RADIUS);
                let ny = center_y + (positions[i].im as f32 * DISK_RADIUS);

                // Euclidean distance for interaction
                let dist_sq = (mx - nx).powi(2) + (my - ny).powi(2);
                if dist_sq < 400.0 { // 20px radius
                    inputs[i] += 100.0;
                }
            }
        }

        // Step the neural simulation
        network.step(&inputs);

        // Render synapses (only active ones)
        // Network doesn't directly expose synapses array safely for visualization easily without indexing
        // But we can visualize the spikes traveling if we had access to in-transit spikes.
        // For now, we'll draw lines between recently active neurons and their neighbors.

        for i in 0..NUM_NEURONS {
            if network.is_spiking(i) {
                let p1 = positions[i];
                let nx1 = center_x + (p1.re as f32 * DISK_RADIUS);
                let ny1 = center_y + (p1.im as f32 * DISK_RADIUS);

                // Draw an expanding ring for the spike
                draw_circle_lines(nx1, ny1, 10.0, 2.0, Color::new(1.0, 0.8, 0.0, 0.8));

                // Draw lines to other neurons it might be communicating with (visual abstraction)
                for j in 0..NUM_NEURONS {
                    if i != j {
                        let dist = hyperbolic_dist(positions[i], positions[j]);
                        if dist < 2.0 {
                            let p2 = positions[j];
                            let nx2 = center_x + (p2.re as f32 * DISK_RADIUS);
                            let ny2 = center_y + (p2.im as f32 * DISK_RADIUS);
                            draw_line(nx1, ny1, nx2, ny2, 1.0, Color::new(1.0, 0.8, 0.0, 0.2));
                        }
                    }
                }
            }
        }

        // Render neurons
        for i in 0..NUM_NEURONS {
            let p = positions[i];
            let nx = center_x + (p.re as f32 * DISK_RADIUS);
            let ny = center_y + (p.im as f32 * DISK_RADIUS);

            // Map voltage (-90 to +30 roughly) to color
            // v is private, so we use is_spiking for color
            let color = if network.is_spiking(i) {
                Color::new(1.0, 1.0, 1.0, 1.0)
            } else {
                // Color gets darker near the edge to represent hyperbolic depth
                let h_dist = hyperbolic_dist(Point::new(0.0, 0.0), p) as f32;
                let intensity = (1.0 / (1.0 + h_dist * 0.5)).clamp(0.2, 0.8);
                Color::new(0.0, intensity, intensity * 0.8, 1.0)
            };

            // Size is also affected by hyperbolic perspective (smaller near edge)
            let size = 4.0 * (1.0 - p.norm() as f32).clamp(0.2, 1.0);

            draw_circle(nx, ny, size, color);
        }

        draw_text("Hyperbolic Neural Networks", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            "Synaptic delay is proportional to non-Euclidean distance.",
            10.0,
            60.0,
            20.0,
            GRAY,
        );
        draw_text(
            "Click near neurons to inject current.",
            10.0,
            80.0,
            20.0,
            GRAY,
        );

        next_frame().await;
    }
}
