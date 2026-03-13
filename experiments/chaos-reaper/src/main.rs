mod heap;
mod physics;

use ::rand::Rng;
use heap::Heap;
use macroquad::prelude::*;
use physics::PendulumSystem;

#[derive(PartialEq)]
enum GcMode {
    Manual,
    ReferenceCounting,
    MarkAndSweep,
}

#[macroquad::main("Chaos-Reaper")]
async fn main() {
    let mut heap = Heap::new();
    let mut rng = ::rand::thread_rng();
    let mut gc_mode = GcMode::Manual;
    let mut last_alloc = 0.0;
    let mut auto_alloc = true;

    heap.nodes.clear();
    heap.roots.clear();
    heap.next_id = 0;

    let w = screen_width();
    let h = screen_height();
    for i in 0..3 {
        let pos = vec2(w / 2.0 + (i as f32 - 1.0) * 150.0, h - 100.0);
        let id = heap.allocate(pos);
        if let Some(node) = heap.nodes.get_mut(&id) {
            node.size = 20.0;
            node.color = GREEN;
            node.ref_count = 1;
        }
        heap.roots.push(id);
    }

    let mut pendulum = PendulumSystem::new();
    let center_x = w / 2.0;
    let center_y = h / 2.0 - 200.0;

    let p_root = pendulum.add_node(vec2(center_x, center_y), 1.0, true, "root".to_string());
    let p_node1 = pendulum.add_node(
        vec2(center_x + 100.0, center_y),
        10.0,
        false,
        "node1".to_string(),
    );
    let p_node2 = pendulum.add_node(
        vec2(center_x + 200.0, center_y),
        10.0,
        false,
        "node2".to_string(),
    );

    pendulum.add_link(p_root, p_node1, 150.0);
    pendulum.add_link(p_node1, p_node2, 150.0);
    pendulum.gravity = vec2(0.0, 400.0);
    pendulum.friction = 0.999;

    let mut path_history = Vec::new();

    loop {
        clear_background(Color::new(0.05, 0.0, 0.05, 1.0));

        let dt = get_frame_time().min(0.05);

        pendulum.step(dt);

        let bob_pos = pendulum.nodes[p_node2].pos;
        path_history.push(bob_pos);
        if path_history.len() > 100 {
            path_history.remove(0);
        }

        for i in 1..path_history.len() {
            let alpha = i as f32 / path_history.len() as f32;
            draw_line(
                path_history[i - 1].x,
                path_history[i - 1].y,
                path_history[i].x,
                path_history[i].y,
                2.0,
                Color::new(1.0, 0.2, 0.2, alpha * 0.5),
            );
        }

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
            let pos = vec2(screen_width() / 2.0, screen_height() / 2.0);
            let a = heap.allocate(pos + vec2(-20.0, 0.0));
            let b = heap.allocate(pos + vec2(20.0, 0.0));
            heap.link(a, b);
            heap.link(b, a);
        }

        if auto_alloc && get_time() - last_alloc > 0.05 {
            last_alloc = get_time();
            let parents: Vec<usize> = heap
                .nodes
                .values()
                .filter(|n| n.alive && n.children.len() < 3)
                .map(|n| n.id)
                .collect();

            if !parents.is_empty() {
                let parent_id = parents[rng.gen_range(0..parents.len())];
                let parent_pos = heap.nodes[&parent_id].pos;

                let angle = rng.gen_range(-std::f32::consts::PI..0.0);
                let dist = rng.gen_range(20.0..60.0);
                let new_pos = parent_pos + vec2(angle.cos() * dist, angle.sin() * dist);

                let new_id = heap.allocate(new_pos);
                heap.link(parent_id, new_id);

                if rng.gen_bool(0.1) {
                    let other_parents: Vec<usize> = heap
                        .nodes
                        .values()
                        .filter(|n| n.alive && n.id != parent_id && n.id != new_id)
                        .map(|n| n.id)
                        .collect();

                    if !other_parents.is_empty() {
                        let op_id = other_parents[rng.gen_range(0..other_parents.len())];
                        if !heap.roots.contains(&new_id) {
                            heap.link(op_id, new_id);
                        }
                    }
                }
            }
        }

        match gc_mode {
            GcMode::Manual => {}
            GcMode::ReferenceCounting => {
                heap.reference_counting();
            }
            GcMode::MarkAndSweep => {
                let t = get_time();
                if t % 2.0 < 0.1 {
                    heap.mark_from_roots();
                } else if t % 2.0 > 1.9 {
                    heap.sweep();
                }
            }
        }

        let mut to_remove = Vec::new();
        let mut updates = Vec::new();

        for node in heap.nodes.values_mut() {
            if node.alive {
                node.age += dt;

                let force = vec2(rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0));

                if !heap.roots.contains(&node.id) {
                    updates.push((node.id, node.parents.clone(), force));
                }
            } else {
                node.size -= dt * 5.0;
                if node.size <= 0.0 {
                    to_remove.push(node.id);
                }
            }
        }

        for (id, parents, mut force) in updates {
            for pid in parents {
                if let Some(parent) = heap.nodes.get(&pid) {
                    let parent_pos = parent.pos;
                    if let Some(node) = heap.nodes.get(&id) {
                        let diff = parent_pos - node.pos;
                        let dist = diff.length();
                        if dist > 30.0 {
                            force += diff.normalize() * (dist - 30.0) * 0.5;
                        }
                    }
                }
            }
            if let Some(node) = heap.nodes.get_mut(&id) {
                node.pos += force * dt;
            }
        }

        let reaper_radius = 20.0;
        let mut to_kill_reaper = Vec::new();
        for node in heap.nodes.values() {
            if node.alive && !heap.roots.contains(&node.id) {
                let dist = (node.pos - bob_pos).length();
                if dist < reaper_radius + node.size {
                    to_kill_reaper.push(node.id);
                }
            }
        }

        for id in to_kill_reaper {
            if let Some(node) = heap.nodes.get_mut(&id) {
                node.alive = false;
                node.color = RED;
                let children = node.children.clone();
                for child_id in children {
                    if let Some(child) = heap.nodes.get_mut(&child_id) {
                        if child.ref_count > 0 {
                            child.ref_count -= 1;
                        }
                    }
                }
            }
        }

        for id in to_remove {
            heap.nodes.remove(&id);
        }

        for node in heap.nodes.values() {
            for &child_id in &node.children {
                if let Some(child) = heap.nodes.get(&child_id) {
                    let color = if node.alive && child.alive {
                        if node.marked && child.marked {
                            Color::new(0.0, 1.0, 0.0, 0.5)
                        } else {
                            Color::new(1.0, 1.0, 1.0, 0.2)
                        }
                    } else {
                        Color::new(0.5, 0.3, 0.2, 0.2)
                    };
                    draw_line(node.pos.x, node.pos.y, child.pos.x, child.pos.y, 2.0, color);

                    if node.alive && child.alive {
                        let dir = (child.pos - node.pos).normalize();
                        let arrow_pos = child.pos - dir * (child.size + 2.0);
                        draw_circle(arrow_pos.x, arrow_pos.y, 2.0, color);
                    }
                }
            }
        }

        for node in heap.nodes.values() {
            let color = if !node.alive {
                node.color
            } else if heap.roots.contains(&node.id) {
                GREEN
            } else if node.marked {
                Color::new(0.2, 0.8, 0.2, 1.0)
            } else {
                Color::new(0.8, 0.8, 0.8, 1.0)
            };

            draw_circle(node.pos.x, node.pos.y, node.size, color);
            draw_circle_lines(node.pos.x, node.pos.y, node.size, 1.0, BLACK);

            if gc_mode == GcMode::ReferenceCounting && node.alive {
                draw_text(
                    &node.ref_count.to_string(),
                    node.pos.x - 4.0,
                    node.pos.y + 4.0,
                    16.0,
                    BLACK,
                );
            }
        }

        for link in &pendulum.links {
            let pos_a = pendulum.nodes[link.a].pos;
            let pos_b = pendulum.nodes[link.b].pos;
            draw_line(pos_a.x, pos_a.y, pos_b.x, pos_b.y, 4.0, DARKGRAY);
        }
        for (i, node) in pendulum.nodes.iter().enumerate() {
            let r = if node.fixed { 8.0 } else { 15.0 };
            let color = if i == p_node2 {
                Color::new(1.0, 0.0, 0.0, 0.8)
            } else {
                LIGHTGRAY
            };
            draw_circle(node.pos.x, node.pos.y, r, color);
        }

        let mode_str = match gc_mode {
            GcMode::Manual => "Manual",
            GcMode::ReferenceCounting => "Reference Counting (Fast Rot)",
            GcMode::MarkAndSweep => "Mark & Sweep (Seasons)",
        };

        draw_text("CHAOS REAPER", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Mode (R): {}", mode_str),
            10.0,
            60.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            &format!("Auto-Allocate (A): {}", auto_alloc),
            10.0,
            80.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text("Create Cycle (C)", 10.0, 100.0, 20.0, LIGHTGRAY);
        let live_nodes = heap.nodes.values().filter(|n| n.alive).count();
        draw_text(
            &format!("Live Nodes: {}", live_nodes),
            10.0,
            120.0,
            20.0,
            LIGHTGRAY,
        );

        if gc_mode == GcMode::Manual {
            draw_text("Mark (M) | Sweep (S) | Full GC (Space)", 10.0, 140.0, 20.0, ORANGE);
        }

        next_frame().await;
    }
}
