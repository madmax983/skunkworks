mod heap;
use heap::Heap;
use macroquad::prelude::*;
use ::rand::Rng;

#[derive(PartialEq)]
enum GcMode {
    Manual,
    ReferenceCounting,
    MarkAndSweep,
}

#[macroquad::main("Myco-Reaper")]
async fn main() {
    let mut heap = Heap::new();
    let mut rng = ::rand::thread_rng();
    let mut gc_mode = GcMode::Manual;
    let mut last_alloc = 0.0;
    let mut auto_alloc = true;

    // Reset roots to screen center for better view if needed,
    // but the hardcoded ones in Heap::new might be okay.
    // Let's re-position roots based on actual screen size now that we have context.
    heap.nodes.clear();
    heap.roots.clear();
    heap.next_id = 0;

    let w = screen_width();
    let h = screen_height();
    for i in 0..3 {
        let pos = vec2(
            w / 2.0 + (i as f32 - 1.0) * 150.0,
            h - 100.0,
        );
        let id = heap.allocate(pos);
        // Hack: Make them roots manually since allocate() doesn't
        if let Some(node) = heap.nodes.get_mut(&id) {
            node.size = 20.0;
            node.color = GREEN;
            node.ref_count = 1;
        }
        heap.roots.push(id);
    }

    loop {
        clear_background(Color::new(0.1, 0.05, 0.0, 1.0)); // Dark soil

        // Controls
        if is_key_pressed(KeyCode::Space) {
            heap.mark_from_roots();
            heap.sweep();
        }
        if is_key_pressed(KeyCode::M) {
            heap.mark_from_roots();
        }
        if is_key_pressed(KeyCode::S) {
            heap.sweep();
        }
        if is_key_pressed(KeyCode::R) {
            gc_mode = match gc_mode {
                GcMode::Manual => GcMode::ReferenceCounting,
                GcMode::ReferenceCounting => GcMode::MarkAndSweep,
                GcMode::MarkAndSweep => GcMode::Manual,
            };
        }
        if is_key_pressed(KeyCode::A) {
            auto_alloc = !auto_alloc;
        }
        if is_key_pressed(KeyCode::C) {
            // Create a cycle
            let pos = vec2(screen_width() / 2.0, screen_height() / 2.0);
            let a = heap.allocate(pos + vec2(-20.0, 0.0));
            let b = heap.allocate(pos + vec2(20.0, 0.0));
            heap.link(a, b);
            heap.link(b, a);
        }

        // Auto Allocation
        if auto_alloc && get_time() - last_alloc > 0.1 {
            last_alloc = get_time();
            // Pick a random parent to grow from
            // Prefer live nodes
            let parents: Vec<usize> = heap.nodes.values()
                .filter(|n| n.alive)
                .map(|n| n.id)
                .collect();

            if !parents.is_empty() {
                let parent_id = parents[rng.gen_range(0..parents.len())];
                if let Some(parent_pos) = heap.nodes.get(&parent_id).map(|n| n.pos) {
                    // Random direction
                    let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
                    let dist = rng.gen_range(30.0..80.0);
                    let new_pos = parent_pos + vec2(angle.cos() * dist, angle.sin() * dist);

                    // Keep within bounds
                    let clamped_pos = vec2(
                        new_pos.x.clamp(50.0, screen_width() - 50.0),
                        new_pos.y.clamp(50.0, screen_height() - 50.0),
                    );

                    let child_id = heap.allocate(clamped_pos);
                    heap.link(parent_id, child_id);

                    // Chance to link to another random node (anastomosis)
                    if rng.gen_bool(0.1) {
                        let other_id = parents[rng.gen_range(0..parents.len())];
                        if other_id != child_id {
                            heap.link(other_id, child_id);
                        }
                    }
                }
            }
        }

        // GC Logic
        match gc_mode {
            GcMode::ReferenceCounting => {
                heap.reference_counting();
            }
            GcMode::MarkAndSweep => {
                // Auto-season: Mark every 2s, Sweep 1s later
                let time = get_time();
                let season = time % 4.0; // 4 second cycle
                if season < 2.0 {
                    // Growing season (allocation happens above)
                    // Reset marks?
                    for node in heap.nodes.values_mut() {
                        node.marked = false;
                    }
                } else if season < 3.0 {
                    // Mark season
                    heap.mark_from_roots();
                } else {
                    // Sweep season
                    heap.sweep();
                }
            }
            _ => {}
        }

        // Draw Links (Hyphae)
        // Draw dead links fainter
        for node in heap.nodes.values() {
            for &child_id in &node.children {
                if let Some(child) = heap.nodes.get(&child_id) {
                    let color = if node.alive && child.alive {
                        if node.marked && child.marked {
                            Color::new(0.0, 1.0, 1.0, 1.0)
                        } else {
                            Color::new(0.5, 0.8, 0.5, 0.5)
                        }
                    } else {
                        Color::new(0.4, 0.3, 0.2, 0.3)
                    };
                    draw_line(node.pos.x, node.pos.y, child.pos.x, child.pos.y, 2.0, color);
                }
            }
        }

        // Draw Nodes
        for node in heap.nodes.values() {
            let mut color = node.color;
            if !node.alive {
                color = BROWN;
            } else if node.marked {
                color = BLUE;
            }

            draw_circle(node.pos.x, node.pos.y, node.size, color);

            // Draw Ref Count
            draw_text(
                &format!("{}", node.ref_count),
                node.pos.x - 5.0,
                node.pos.y + 5.0,
                15.0,
                WHITE
            );
        }

        // Draw UI
        draw_text("Myco-Reaper", 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Nodes: {}", heap.nodes.len()), 20.0, 60.0, 20.0, WHITE);
        draw_text(&format!("GC Mode: {:?}", match gc_mode {
            GcMode::Manual => "Manual (Space/M/S)",
            GcMode::ReferenceCounting => "Ref Counting (Continuous)",
            GcMode::MarkAndSweep => "Mark & Sweep",
        }), 20.0, 80.0, 20.0, WHITE);
        draw_text("Space: Mark+Sweep | M: Mark | S: Sweep | R: Toggle RC | A: Toggle Alloc | C: Cycle", 20.0, screen_height() - 20.0, 20.0, WHITE);

        next_frame().await
    }
}
