mod simulation;
use ::rand::Rng;
use macroquad::prelude::*;
use simulation::{Heap, ObjectState};

const CYAN: Color = Color::new(0.0, 1.0, 1.0, 1.0);

fn conf() -> Conf {
    Conf {
        window_title: "Heap Fungus".to_string(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut heap = Heap::new();
    let mut rng = ::rand::thread_rng();

    // Initial population
    let root = heap.allocate(vec2(400.0, 300.0));
    heap.add_root(root);

    let mut timer = 0.0;
    let mut phase = "GROW"; // GROW, MARK, DECAY, SWEEP

    loop {
        clear_background(BLACK);

        let dt = get_frame_time();
        timer += dt;

        // Logic
        match phase {
            "GROW" => {
                // Allocation rate
                if rng.gen_bool(0.1) {
                    let pos = vec2(rng.gen_range(50.0..750.0), rng.gen_range(50.0..550.0));
                    let node = heap.allocate(pos);

                    // Link to existing node
                    if heap.graph.node_count() > 1 {
                        // Pick a random existing node
                        let count = heap.graph.node_count();
                        // This is O(N), slow but okay for small N
                        if let Some(other) = heap.graph.node_indices().nth(rng.gen_range(0..count))
                        {
                            if other != node {
                                // 30% chance to link FROM root/existing TO new (creating reachable garbage)
                                // 70% chance to link FROM new TO existing (creating unreachable garbage if new is not linked to root)
                                // Actually, to make interesting structures:
                                if rng.gen_bool(0.3) {
                                    heap.link(other, node);
                                } else {
                                    heap.link(node, other);
                                }
                            }
                        }
                    }
                }

                if timer > 3.0 {
                    phase = "MARK";
                    timer = 0.0;
                    heap.mark();
                }
            }
            "MARK" => {
                // Just visualization delay
                if timer > 1.0 {
                    phase = "DECAY";
                    timer = 0.0;
                    heap.decay();
                }
            }
            "DECAY" => {
                // Visualization delay
                if timer > 2.0 {
                    phase = "SWEEP";
                    timer = 0.0;
                    heap.sweep();
                }
            }
            "SWEEP" => {
                // Visualization delay
                if timer > 1.0 {
                    phase = "GROW";
                    timer = 0.0;
                }
            }
            _ => {}
        }

        // Draw
        draw_text(&format!("Phase: {}", phase), 20.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Nodes: {}", heap.graph.node_count()),
            20.0,
            60.0,
            30.0,
            WHITE,
        );

        // Draw edges first
        for edge in heap.graph.edge_indices() {
            if let Some((start, end)) = heap.graph.edge_endpoints(edge) {
                if let (Some(n1), Some(n2)) =
                    (heap.graph.node_weight(start), heap.graph.node_weight(end))
                {
                    // Hyphae color depends on phase
                    let color = if phase == "MARK"
                        && heap.graph[start].state == ObjectState::Marked
                        && heap.graph[end].state == ObjectState::Marked
                    {
                        YELLOW // Active signal
                    } else {
                        Color::new(0.5, 0.5, 0.5, 0.5) // Gray hyphae
                    };
                    draw_line(n1.pos.x, n1.pos.y, n2.pos.x, n2.pos.y, 1.0, color);
                }
            }
        }

        // Draw nodes
        for node_idx in heap.graph.node_indices() {
            let obj = &heap.graph[node_idx];
            let color = match obj.state {
                ObjectState::Allocated => GREEN,
                ObjectState::Marked => CYAN,   // Glowing
                ObjectState::Rotting => BROWN, // Rot
            };

            let size = if heap.roots.contains(&node_idx) {
                15.0
            } else {
                10.0
            };

            draw_rectangle(
                obj.pos.x - size / 2.0,
                obj.pos.y - size / 2.0,
                size,
                size,
                color,
            );

            // Roots indicator
            if heap.roots.contains(&node_idx) {
                draw_circle_lines(obj.pos.x, obj.pos.y, size, 2.0, BLUE);
            }

            // Decomposer visualization (Red dots on rotting nodes)
            if obj.state == ObjectState::Rotting {
                draw_circle(obj.pos.x, obj.pos.y, 3.0, RED);
            }
        }

        // Manual interactions
        if is_key_pressed(KeyCode::Space) {
            // Force jump to next phase
            timer = 100.0;
        }

        next_frame().await
    }
}
