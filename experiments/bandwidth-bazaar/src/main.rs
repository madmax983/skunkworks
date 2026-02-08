use crate::model::{LinkId, NodeId, PacketId, World};
use macroquad::prelude::*;

mod model;

#[macroquad::main("Bandwidth Bazaar")]
async fn main() {
    let mut world = World::new();
    let screen_w = screen_width();
    let screen_h = screen_height();

    // Generate Graph
    let num_nodes = 30;
    let mut nodes = Vec::new();

    for i in 0..num_nodes {
        let x = rand::gen_range(50.0, screen_w - 50.0);
        let y = rand::gen_range(50.0, screen_h - 50.0);
        world.add_node(i, (x, y));
        nodes.push((i, x, y));
    }

    // Connect nodes to 3 nearest neighbors
    let mut link_id_counter = 0;
    for i in 0..num_nodes {
        let (id_a, x_a, y_a) = nodes[i];

        // Find distances
        let mut distances: Vec<(usize, f32)> = nodes
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx != i)
            .map(|(idx, (_, x, y))| {
                let dx = x - x_a;
                let dy = y - y_a;
                (idx, (dx * dx + dy * dy).sqrt())
            })
            .collect();

        distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Connect to closest 3
        for (target_idx, dist) in distances.iter().take(3.min(distances.len())) {
            let target_node_id = nodes[*target_idx].0;

            // Add directed link A -> B
            // Check if link already exists? World allows multiple links.
            // But let's check if we already added A->B to avoid duplicates if K-nearest is symmetric (it's not always).
            // Simplest: Just add.

            let capacity = rand::gen_range(5, 15);
            let base_cost = *dist / 10.0; // Cost proportional to distance
            let length = *dist;

            world.add_link(
                link_id_counter,
                id_a,
                target_node_id,
                capacity,
                base_cost,
                length,
            );
            link_id_counter += 1;

            // Add return link B -> A ?
            // Let's rely on B's nearest neighbor search to add it.
            // But nearest neighbor is not symmetric. If A is closest to B, B might have C closer.
            // To ensure connectivity, maybe add bidirectional?
            // Let's add bidirectional explicitly to ensure graph is strongly connected (mostly).
            world.add_link(
                link_id_counter,
                target_node_id,
                id_a,
                capacity,
                base_cost,
                length,
            );
            link_id_counter += 1;
        }
    }

    loop {
        // Handle Input
        if is_key_pressed(KeyCode::Space) {
            // Spawn massive traffic
            // Flood from random node 0 to random node N-1
            let src = NodeId(0);
            let dst = NodeId(num_nodes - 1);

            // Spawn 100 packets
            for _ in 0..100 {
                let pid = PacketId(world.next_packet_id);
                world.next_packet_id += 1;

                // Find route from src
                if let Some(node) = world.nodes.get(&src) {
                    if let Some((link_id, _)) = node.routing_table.get(&dst) {
                        // Dereference link_id because get returns reference
                        let link_id_val = *link_id;
                        if let Some(link) = world.links.get_mut(&link_id_val) {
                            link.queue.push_back(pid);

                            let packet = crate::model::Packet {
                                id: pid,
                                source: src,
                                destination: dst,
                                current_link: Some(link_id_val),
                                t: 0.0,
                                budget: 100.0,
                            };
                            world.packets.insert(pid, packet);
                        }
                    }
                }
            }
        }

        world.step();

        clear_background(BLACK);

        // Draw Links
        for link in world.links.values() {
            if let (Some(node_from), Some(node_to)) =
                (world.nodes.get(&link.from), world.nodes.get(&link.to))
            {
                let (x1, y1) = node_from.pos;
                let (x2, y2) = node_to.pos;

                // Color based on congestion
                // Ratio of current cost to base cost
                let ratio = link.current_cost / link.base_cost;
                // ratio 1.0 = blue. ratio > 5.0 = red.
                let t = ((ratio - 1.0) / 5.0).clamp(0.0, 1.0);
                let color = Color::new(t, 0.0, 1.0 - t, 0.5); // alpha 0.5 for lines

                let thickness = (link.capacity as f32) / 5.0;

                draw_line(x1, y1, x2, y2, thickness, color);
            }
        }

        // Draw Nodes
        for node in world.nodes.values() {
            let (x, y) = node.pos;
            draw_circle(x, y, 5.0, WHITE);
            // Draw routing table size or something? No, too cluttered.
        }

        // Draw Packets
        for packet in world.packets.values() {
            if let Some(link_id) = packet.current_link {
                if let Some(link) = world.links.get(&link_id) {
                    if let (Some(node_from), Some(node_to)) =
                        (world.nodes.get(&link.from), world.nodes.get(&link.to))
                    {
                        let (x1, y1) = node_from.pos;
                        let (x2, y2) = node_to.pos;

                        let t = packet.t;
                        let x = x1 + (x2 - x1) * t;
                        let y = y1 + (y2 - y1) * t;

                        draw_circle(x, y, 3.0, YELLOW);
                    }
                }
            }
            // If packet is in queue, it has current_link = Some(link_id) ?
            // In my model logic:
            // - `spawn`: sets `current_link = Some(start_link)`, adds to `link.queue`.
            // - `transitions`: sets `current_link = Some(next_link)`, adds to `link.queue`.
            // So packets in queue ALSO have `current_link`.
            // But `t` is 0.0.
            // So they will be drawn at `node_from`.
            // To visualize queue, maybe stack them?
            // Or just letting them bunch up at start node is fine.
        }

        // Stats
        draw_text(
            &format!("Packets: {}", world.packets.len()),
            10.0,
            20.0,
            30.0,
            WHITE,
        );
        draw_text(
            "Routes updating based on congestion...",
            10.0,
            50.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
