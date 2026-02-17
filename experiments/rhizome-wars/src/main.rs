use macroquad::prelude::*;

mod rrt;
mod simulation;

use rrt::{Tree, Obstacle};
use simulation::{Scheduler, Resource};

#[macroquad::main("Rhizome Wars")]
async fn main() {
    let screen_w = screen_width();
    let screen_h = screen_height();

    // Setup bounds (slightly inside screen)
    let bounds = Rect::new(0.0, 0.0, screen_w, screen_h);

    // Obstacles
    let mut obstacles = Vec::new();
    // Add some random rocks
    for _ in 0..10 {
        obstacles.push(Obstacle {
            pos: Vec2::new(
                rand::gen_range(100.0, screen_w - 100.0),
                rand::gen_range(100.0, screen_h - 100.0),
            ),
            radius: rand::gen_range(20.0, 50.0),
        });
    }

    // Trees
    let mut tree1 = Tree::new(Vec2::new(50.0, screen_h / 2.0), GREEN);
    let mut tree2 = Tree::new(Vec2::new(screen_w - 50.0, screen_h / 2.0), RED);

    let mut scheduler = Scheduler::new();

    // Initial Resources
    for _ in 0..5 {
        scheduler.update(10.0, bounds); // Force spawn
    }

    let mut running = true;

    loop {
        // Handle input
        if is_key_pressed(KeyCode::Space) {
            running = !running;
        }
        if is_key_pressed(KeyCode::R) {
            // Reset
            tree1 = Tree::new(Vec2::new(50.0, screen_h / 2.0), GREEN);
            tree2 = Tree::new(Vec2::new(screen_w - 50.0, screen_h / 2.0), RED);
            obstacles.clear();
            for _ in 0..10 {
                obstacles.push(Obstacle {
                    pos: Vec2::new(
                        rand::gen_range(100.0, screen_w - 100.0),
                        rand::gen_range(100.0, screen_h - 100.0),
                    ),
                    radius: rand::gen_range(20.0, 50.0),
                });
            }
            scheduler = Scheduler::new();
        }

        // Add obstacle with mouse
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            obstacles.push(Obstacle {
                pos: Vec2::new(mx, my),
                radius: 10.0,
            });
        }

        if running {
            let dt = get_frame_time();

            // Update Scheduler
            scheduler.update(dt, bounds);

            // Update Trees
            // Target strategy: find nearest resource for each tree
            let target1 = find_nearest_resource(tree1.nodes.last().unwrap().pos, &scheduler.resources);
            let target2 = find_nearest_resource(tree2.nodes.last().unwrap().pos, &scheduler.resources);

            // Grow multiple steps per frame for speed
            for _ in 0..10 {
                // Pass other tree as reference for collision
                tree1.grow_step(target1, &obstacles, &[&tree2], bounds);
                tree2.grow_step(target2, &obstacles, &[&tree1], bounds);
            }

            // Check resource consumption
            // If any node is within resource radius, consume it
            consume_resources(&mut tree1, &mut scheduler);
            consume_resources(&mut tree2, &mut scheduler);
        }

        // Draw
        clear_background(Color::new(0.1, 0.05, 0.0, 1.0)); // Dark soil

        // Draw Obstacles
        for obs in &obstacles {
            draw_circle(obs.pos.x, obs.pos.y, obs.radius, GRAY);
        }

        // Draw Resources
        for res in &scheduler.resources {
            draw_circle(res.pos.x, res.pos.y, res.radius, BLUE);
            // Pulse effect?
            draw_circle_lines(res.pos.x, res.pos.y, res.radius + (get_time() as f32 * 5.0).sin() * 2.0 + 2.0, 1.0, SKYBLUE);
        }

        // Draw Trees
        draw_tree(&tree1);
        draw_tree(&tree2);

        // UI
        draw_text("Rhizome Wars", 20.0, 30.0, 30.0, WHITE);
        draw_text("Space: Pause | R: Reset | Click: Add Rock", 20.0, 60.0, 20.0, LIGHTGRAY);
        draw_text(&format!("Green: {}", tree1.nodes.len()), 20.0, 90.0, 20.0, GREEN);
        draw_text(&format!("Red: {}", tree2.nodes.len()), 20.0, 110.0, 20.0, RED);

        next_frame().await
    }
}

fn find_nearest_resource(pos: Vec2, resources: &[Resource]) -> Option<Vec2> {
    if resources.is_empty() {
        return None;
    }
    let mut min_dist = f32::MAX;
    let mut nearest = None;
    for res in resources {
        let d = pos.distance_squared(res.pos);
        if d < min_dist {
            min_dist = d;
            nearest = Some(res.pos);
        }
    }
    nearest
}

fn consume_resources(tree: &mut Tree, scheduler: &mut Scheduler) {
    // Naive O(N*M) check
    // Optimization: Spatial partition or check only new nodes.
    // For now, iterate all resources and see if any node is close.
    // Actually, iterate resources and check against *all* nodes? Too slow.
    // Iterate *new* nodes? We don't track new nodes easily here without changing API.
    // Let's iterate resources and find nearest node in tree (tree has spatial index? No).
    // Let's just check the last added node? Roots grow from tips.
    // But multiple tips might exist? RRT grows from anywhere.
    // grow_step adds one node.
    // Let's check the last few nodes?

    // Better: Scheduler checks trees?
    // Let's just brute force for now, N and M are small (< 5000 nodes, < 10 resources).
    // 5000 * 10 = 50,000 checks. Fast enough.

    scheduler.resources.retain(|res| {
        for node in &tree.nodes {
            if node.pos.distance(res.pos) < res.radius + 2.0 {
                // Consumed!
                return false;
            }
        }
        true
    });
}

fn draw_tree(tree: &Tree) {
    for node in &tree.nodes {
        if let Some(parent_idx) = node.parent_index {
            let parent = tree.nodes[parent_idx];
            draw_line(parent.pos.x, parent.pos.y, node.pos.x, node.pos.y, 1.5, tree.color);
        }
    }
}
