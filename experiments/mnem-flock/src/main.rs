use macroquad::prelude::*;
use std::f32::consts::PI;

mod boid;
mod glitch;
mod graph;

use boid::Boid;
use glitch::TextGlitcher;
use graph::Graph;

#[macroquad::main("Mnem Flock")]
async fn main() {
    let mut graph = Graph::new();
    println!("Scanning directory...");

    // Attempt scan (try src first, fallback to dot)
    if std::path::Path::new("experiments/mnem-flock/src").exists() {
        graph.scan_directory("experiments/mnem-flock/src");
    } else {
        graph.scan_directory(".");
    }

    if graph.nodes.is_empty() {
        // Fallback dummy nodes if empty (e.g. running from wrong dir)
        graph.add_node("Core".to_string(), "struct System { entropy: f32 }".to_string());
        graph.add_node("Render".to_string(), "fn draw() { loop {} }".to_string());
        graph.add_node("Logic".to_string(), "impl Logic for System {}".to_string());
    }

    // Spawn Flock
    let num_boids = 100;
    let mut boids: Vec<Boid> = (0..num_boids)
        .map(|_| Boid::new(rand::gen_range(0.0, screen_width()), rand::gen_range(0.0, screen_height())))
        .collect();

    let mut decay_timer = 0.0;
    let decay_rate = 0.05;

    loop {
        let dt = get_frame_time();
        decay_timer += dt;

        let w = screen_width();
        let h = screen_height();

        // --- 1. Update Graph (Entropy) ---
        // Nodes decay over time
        if decay_timer > 0.1 {
            let node_count = graph.nodes.len();
            for i in 0..node_count {
                // Borrow checker dance: we need to mutate node `i` but read others
                let (head, tail) = graph.nodes.split_at_mut(i + 1);
                let node = &mut head[i];
                let others = &tail; // This only covers nodes AFTER i, which is incomplete for all-to-all.
                // A simpler way for this experiment: Update physics in one pass (copy positions), then apply.

                // Decay
                node.health -= 0.001;
                if node.health < 0.1 { node.health = 0.1; }

                // Physics (Wall bounce only to avoid O(N^2) borrow issues for now, or use index based lookups carefully)
                // Let's just do wall bounce and decay in this loop, and repulsion in a separate loop if needed.
                // For a small graph, we can clone positions.
            }

            // Repulsion Pass
            let positions: Vec<(usize, Vec2)> = graph.nodes.iter().map(|n| (n.id, n.pos)).collect();
            for node in &mut graph.nodes {
                 for (other_id, other_pos) in &positions {
                    if node.id != *other_id {
                        let diff = node.pos - *other_pos;
                        let dist_sq = diff.length_squared();
                        if dist_sq < 10000.0 && dist_sq > 0.0 {
                            node.vel += diff.normalize() * (100.0 / dist_sq);
                        }
                    }
                }

                // Wall bounce
                if node.pos.x < 50.0 { node.vel.x += 0.5; }
                if node.pos.x > w - 50.0 { node.vel.x -= 0.5; }
                if node.pos.y < 50.0 { node.vel.y += 0.5; }
                if node.pos.y > h - 50.0 { node.vel.y -= 0.5; }

                // Damp
                node.vel *= 0.95;
                node.pos += node.vel;
            }
            decay_timer = 0.0;
        }

        // --- 2. Update Flock (Behavior) ---
        // We need a snapshot of boids for reading neighbors
        let old_boids = boids.clone();

        for (i, boid) in boids.iter_mut().enumerate() {
            let mut sep = Vec2::ZERO;
            let mut ali = Vec2::ZERO;
            let mut coh = Vec2::ZERO;
            let mut count = 0;

            // Flock Rules
            for (j, other) in old_boids.iter().enumerate() {
                if i == j { continue; }

                let dist = boid.position.distance(other.position);

                if dist < boid.dna.view_radius {
                    // Separation
                    let diff = boid.position - other.position;
                    sep += diff.normalize() / dist; // Weight by inverse distance

                    // Alignment
                    ali += other.velocity;

                    // Cohesion
                    coh += other.position;

                    count += 1;

                    // Firefly Coupling
                    if dist < boid.dna.coupling_radius {
                         // Kuramoto model for phase sync
                        let delta = (other.phase - boid.phase) * 2.0 * PI;
                        boid.phase += boid.dna.coupling_strength * delta.sin();
                    }
                }
            }

            if count > 0 {
                sep = sep.normalize() * boid.dna.max_speed;
                sep -= boid.velocity;
                sep = sep.clamp_length_max(boid.dna.max_force) * boid.dna.separation_weight;

                ali = (ali / count as f32).normalize() * boid.dna.max_speed;
                ali -= boid.velocity;
                ali = ali.clamp_length_max(boid.dna.max_force) * boid.dna.alignment_weight;

                coh = (coh / count as f32 - boid.position).normalize() * boid.dna.max_speed;
                coh -= boid.velocity;
                coh = coh.clamp_length_max(boid.dna.max_force) * boid.dna.cohesion_weight;
            }

            // --- Taxis: Seek Decay (The Novel Trait) ---
            let mut seek = Vec2::ZERO;
            let mut nearest_dist = f32::MAX;
            let mut target_node: Option<usize> = None;

            for (nid, node) in graph.nodes.iter().enumerate() {
                // Find rotting nodes
                if node.health < 0.8 {
                    let dist = boid.position.distance(node.pos);
                    // Weight attraction by how rotted it is (1.0 - health)
                    // and by distance
                    if dist < nearest_dist {
                         nearest_dist = dist;
                         if dist < 200.0 { // Perception range for "smell" of rot
                             target_node = Some(nid);
                         }
                    }
                }
            }

            if let Some(nid) = target_node {
                let node = &graph.nodes[nid];
                let desired = (node.pos - boid.position).normalize() * boid.dna.max_speed;
                let steer = (desired - boid.velocity).clamp_length_max(boid.dna.max_force);

                // Stronger pull if health is lower
                let urgency = (1.0 - node.health) * boid.dna.heal_weight;
                seek = steer * urgency;

                // Repair Logic
                if nearest_dist < 20.0 {
                    boid.healing = true;
                    // Heal the node in the graph (requires mutable access later,
                    // or we cheat and use unsafe, or we gather heal events.
                    // Let's gather heal events or just modify graph after boid loop.)
                    // For now, visual feedback only on boid
                } else {
                    boid.healing = false;
                }
            } else {
                boid.healing = false;
            }

            // Apply Forces
            boid.apply_force(sep);
            boid.apply_force(ali);
            boid.apply_force(coh);
            boid.apply_force(seek);

            // Natural frequency
            boid.phase += boid.dna.natural_freq;

            boid.update_flash();
            boid.update_physics(w, h);
        }

        // Apply Healing (Separate pass to avoid borrow issues)
        for boid in &boids {
             if boid.healing {
                 for node in &mut graph.nodes {
                     if boid.position.distance(node.pos) < 20.0 {
                         node.health += 0.005; // Heal
                         if node.health > 1.0 { node.health = 1.0; }
                     }
                 }
             }
        }


        // --- Render ---
        clear_background(BLACK);

        // Draw Edges
        for edge in &graph.edges {
             let n1 = graph.nodes[edge.from].pos;
             let n2 = graph.nodes[edge.to].pos;
             draw_line(n1.x, n1.y, n2.x, n2.y, 1.0, DARKGRAY);
        }

        // Draw Nodes
        for node in &graph.nodes {
            // Color based on health
            let color = if node.health > 0.8 { GREEN }
                       else if node.health > 0.4 { YELLOW }
                       else { RED };

            draw_circle_lines(node.pos.x, node.pos.y, 10.0 + node.health * 5.0, 2.0, color);

            // Text Glitch
            let intensity = (1.0 - node.health).powf(2.0); // Non-linear glitch
            let text = TextGlitcher::corrupt(&node.name, intensity);

            draw_text(&text, node.pos.x + 12.0, node.pos.y, 16.0, color);

            // Health bar
            draw_rectangle(node.pos.x - 10.0, node.pos.y - 15.0, 20.0 * node.health, 3.0, color);
        }

        // Draw Boids
        for boid in &boids {
            let angle = boid.velocity.y.atan2(boid.velocity.x);
            let size = 4.0;

            let tip = boid.position + vec2(angle.cos(), angle.sin()) * size * 2.0;
            let left = boid.position + vec2((angle + 2.5).cos(), (angle + 2.5).sin()) * size;
            let right = boid.position + vec2((angle - 2.5).cos(), (angle - 2.5).sin()) * size;

            let mut color = boid.dna.color;

            // Flash logic
            if boid.flash_timer > 0 || boid.healing {
                color = WHITE;
                // Draw glow
                draw_circle(boid.position.x, boid.position.y, size * 3.0, Color::new(1.0, 1.0, 1.0, 0.2));
            } else {
                // Dim when not flashing
                color.a = 0.5 + 0.5 * boid.phase;
            }

            draw_triangle(tip, left, right, color);
        }

        // UI
        draw_text("Mnem-Flock: Repair Drones", 10.0, 20.0, 20.0, WHITE);
        draw_text(format!("Entropy Active. Nodes: {}", graph.nodes.len()).as_str(), 10.0, 40.0, 16.0, GRAY);

        next_frame().await
    }
}
