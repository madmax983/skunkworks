use macroquad::prelude::*;

mod grammar;
mod forest;
mod scheduler;
mod turtle;

use grammar::LSystem;
use forest::{Forest, Tree, TreeID};
use scheduler::{Scheduler, RoundRobin, RandomScheduler, PriorityScheduler};
use turtle::{Turtle, Action};

#[macroquad::main("Scheduler Forest")]
async fn main() {
    let mut forest = Forest::new();
    let mut scheduler: Box<dyn Scheduler> = Box::new(RoundRobin::new());

    // Define L-Systems
    let fractal_plant = LSystem::new("X", vec![('X', "F+[[X]-X]-F[-FX]+X"), ('F', "FF")]);
    let dragon_curve = LSystem::new("FX", vec![('X', "X+YF+"), ('Y', "-FX-Y")]);
    let binary_tree = LSystem::new("G", vec![('G', "F[-G]+G"), ('F', "FF")]);
    let sierpinski = LSystem::new("F-G-G", vec![('F', "F-G+F+G-F"), ('G', "GG")]);

    // Add trees
    forest.add_tree(Tree::new(
        0,
        &fractal_plant,
        4,
        -200.0,
        (0.2, 0.8, 0.2) // Green
    ));
    forest.add_tree(Tree::new(
        1,
        &dragon_curve,
        10,
        -50.0,
        (0.8, 0.2, 0.2) // Red
    ));
    forest.add_tree(Tree::new(
        2,
        &binary_tree,
        5,
        100.0,
        (0.2, 0.2, 0.8) // Blue
    ));
    forest.add_tree(Tree::new(
        3,
        &sierpinski,
        4,
        250.0,
        (0.8, 0.8, 0.2) // Yellow
    ));

    let mut paused = false;
    let mut speed = 5;
    let mut active_tree_id: Option<TreeID> = None;

    let mut cam = Camera2D {
        zoom: vec2(0.005, 0.005),
        target: vec2(0.0, -150.0),
        ..Default::default()
    };

    loop {
        // Input
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_pressed(KeyCode::R) {
            // Reset growth
            for tree in &mut forest.trees {
                tree.growth_cursor = 0;
            }
        }
        if is_key_pressed(KeyCode::Tab) {
            // Cycle scheduler
            if scheduler.name() == "Round Robin" {
                scheduler = Box::new(RandomScheduler);
            } else if scheduler.name() == "Random" {
                scheduler = Box::new(PriorityScheduler);
            } else {
                scheduler = Box::new(RoundRobin::new());
            }
        }
        if is_key_pressed(KeyCode::Right) {
            speed = (speed + 1).min(100);
        }
        if is_key_pressed(KeyCode::Left) {
            speed = (speed - 1).max(1);
        }

        // Camera
        if is_mouse_button_down(MouseButton::Right) {
            let delta = mouse_delta_position();
            cam.target.x -= delta.x / cam.zoom.x;
            cam.target.y += delta.y / cam.zoom.y;
        }
        let wheel = mouse_wheel().1;
        if wheel != 0.0 {
            cam.zoom *= if wheel > 0.0 { 1.1 } else { 0.9 };
        }

        // Logic
        if !paused {
            active_tree_id = scheduler.schedule(&forest);
            if let Some(id) = active_tree_id {
                if let Some(tree) = forest.get_tree_mut(id) {
                    tree.grow(speed);
                }
            }
        }

        // Render
        clear_background(BLACK);
        set_camera(&cam);

        // Ground
        draw_line(-1000.0, 0.0, 1000.0, 0.0, 2.0, DARKGRAY);

        // Trees
        for tree in &forest.trees {
            let mut turtle = Turtle::new(
                Vec2::new(tree.x_pos, 0.0),
                -90.0f32.to_radians(), // Start pointing UP
                2.0, // Step size
                25.0f32.to_radians(), // Turn angle (fixed for now, should be per-tree)
                0.9, // Width decay
            );

            // Override turn angle based on tree type (hacky detection via color/id)
            if tree.id == 1 { turtle.turn_angle = 90.0f32.to_radians(); } // Dragon
            if tree.id == 2 { turtle.turn_angle = 45.0f32.to_radians(); } // Binary
            if tree.id == 3 { turtle.turn_angle = 60.0f32.to_radians(); } // Sierpinski

            // Draw only up to growth_cursor
            for (i, c) in tree.full_dna.chars().enumerate() {
                if i >= tree.growth_cursor {
                    break;
                }
                let action = turtle.interpret(c);
                match action {
                    Action::Move(start, end) => {
                        let color = if Some(tree.id) == active_tree_id {
                            WHITE // Highlight active tree growth
                        } else {
                            Color::new(tree.color.0, tree.color.1, tree.color.2, 1.0)
                        };
                        draw_line(start.x, start.y, end.x, end.y, turtle.state.width, color);
                    }
                    _ => {}
                }
            }
        }

        set_default_camera();

        // UI
        draw_text(
            &format!("Scheduler: {} (Tab)", scheduler.name()),
            20.0,
            30.0,
            30.0,
            WHITE,
        );
        draw_text(
            &format!("Speed: {} (Left/Right)", speed),
            20.0,
            60.0,
            30.0,
            WHITE,
        );
        draw_text(
            if paused { "PAUSED (Space)" } else { "RUNNING (Space)" },
            20.0,
            90.0,
            30.0,
            if paused { RED } else { GREEN },
        );
        if let Some(id) = active_tree_id {
            draw_text(
                &format!("Active Tree: {}", id),
                20.0,
                120.0,
                30.0,
                YELLOW,
            );
        }

        next_frame().await;
    }
}
